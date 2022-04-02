use crate::error::{BoxError, BoxResult};
use crate::fat::{self, Fat};
use crate::dir;

use std::io::{Read, Seek, SeekFrom};
use std::rc::Rc;

use nom::{
	IResult,
	bytes::streaming::{tag, take},
	number::streaming::{le_u16, le_u32},
	combinator::map_res,
	multi::count,
};

pub const HEADER_SIZE: usize = 8 + 16 + (2 * 5) + 6 + (4 * 9) + (4 * 109);
pub const SIGNATURE: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];

pub const V3: u16 = 0x0003;
pub const V4: u16 = 0x0004;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompoundFileHeader {
	pub signature: [u8; 8],
	pub clsid: [u8; 16],
	pub version_minor: u16,
	pub version_major: u16,
	pub byte_order: u16,
	pub sector_shift: u16,
	pub mini_sector_shift: u16,
	pub reserved: [u8; 6],
	pub dir_sectors: u32,
	pub fat_sectors: u32,
	pub dir_first_sector: u32, // sector number
	pub tx_sig_num: u32,
	pub mini_stream_cutoff_size: u32,
	pub minifat_first_sector: u32, // sector number
	pub minifat_sectors: u32,
	pub difat_first_sector: u32, // sector number
	pub difat_sectors: u32,
	pub difat: [u32; 109],
}

impl CompoundFileHeader {
	pub fn sector_size(&self) -> usize {
		2usize.pow(self.sector_shift as u32)
	}

	pub fn sector_offset(&self, sector: u32) -> usize {
		((sector as usize) + 1) * self.sector_size()
	}

	pub fn new_v3() -> Self {
		Self {
			signature: SIGNATURE,
			clsid: [0; 16],
			version_minor: 0x003E,
			version_major: V3,
			byte_order: 0xFFFE,
			sector_shift: 0x0009,
			mini_sector_shift: 0x0006,
			reserved: [0; 6],
			dir_sectors: 0,
			fat_sectors: 0,
			dir_first_sector: fat::ENDOFCHAIN,
			tx_sig_num: 0,
			mini_stream_cutoff_size: 0x00001000,
			minifat_first_sector: fat::ENDOFCHAIN,
			minifat_sectors: 0,
			difat_first_sector: fat::ENDOFCHAIN,
			difat_sectors: 0,
			difat: [0; 109],
		}
	}

	pub fn new_v4() -> Self {
		Self {
			signature: SIGNATURE,
			clsid: [0; 16],
			version_minor: 0x003E,
			version_major: V4,
			byte_order: 0xFFFE,
			sector_shift: 0x000C,
			mini_sector_shift: 0x0006,
			reserved: [0; 6],
			dir_sectors: 0,
			fat_sectors: 0,
			dir_first_sector: fat::ENDOFCHAIN,
			tx_sig_num: 0,
			mini_stream_cutoff_size: 0x00001000,
			minifat_first_sector: fat::ENDOFCHAIN,
			minifat_sectors: 0,
			difat_first_sector: fat::ENDOFCHAIN,
			difat_sectors: 0,
			difat: [0; 109],
		}
	}

	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, signature) = map_res(tag(SIGNATURE), |b: &[u8]| b.try_into())(input)?;
		let (input, clsid) = map_res(take(16usize), |b: &[u8]| b.try_into())(input)?;
		let (input, version_minor) = le_u16(input)?;
		let (input, version_major) = le_u16(input)?;
		let (input, byte_order) = le_u16(input)?;
		let (input, sector_shift) = le_u16(input)?;
		let (input, mini_sector_shift) = le_u16(input)?;
		let (input, reserved) = map_res(take(6usize), |b: &[u8]| b.try_into())(input)?;
		let (input, dir_sectors) = le_u32(input)?;
		let (input, fat_sectors) = le_u32(input)?;
		let (input, dir_first_sector) = le_u32(input)?;
		let (input, tx_sig_num) = le_u32(input)?;
		let (input, mini_stream_cutoff_size) = le_u32(input)?;
		let (input, minifat_first_sector) = le_u32(input)?;
		let (input, minifat_sectors) = le_u32(input)?;
		let (input, difat_first_sector) = le_u32(input)?;
		let (input, difat_sectors) = le_u32(input)?;
		let (input, difat) = map_res(count(le_u32, 109usize), |b: Vec<u32>| b.try_into())(input)?;
		Ok((input, Self {
			signature,
			clsid,
			version_minor,
			version_major,
			byte_order,
			sector_shift,
			mini_sector_shift,
			reserved,
			dir_sectors,
			fat_sectors,
			dir_first_sector,
			tx_sig_num,
			mini_stream_cutoff_size,
			minifat_first_sector,
			minifat_sectors,
			difat_first_sector,
			difat_sectors,
			difat,
		}))
	}
}

impl Default for CompoundFileHeader {
	fn default() -> Self {
		Self::new_v4()
	}
}

impl std::fmt::Display for CompoundFileHeader {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "CFB header v{}.{}, sectors: (DIR: {}, FAT: {}, MiniFAT: {}, DIFAT: {})", self.version_major, self.version_minor, self.dir_sectors, self.fat_sectors, self.minifat_sectors, self.difat_sectors)
	}
}


fn get_sector_bytes<R: Read + Seek>(reader: &mut R, buf: &mut Vec<u8>, header: &CompoundFileHeader, sector: u32) -> BoxResult<()> {
	buf.clear();
	buf.resize(header.sector_size(), 0);
	reader.seek(SeekFrom::Start(header.sector_offset(sector) as u64))?;
	reader.read_exact(buf)?;
	Ok(())
}

fn get_fat_data<R: Read + Seek>(reader: &mut R, buf: &mut Vec<u8>, header: &CompoundFileHeader, entries: &[u32], mut sector: u32, size: usize) -> BoxResult<Vec<u8>> {
	let mut bytes = Vec::new();
	while sector != fat::ENDOFCHAIN {
		get_sector_bytes(reader, buf, header, sector)?;
		bytes.extend_from_slice(&buf);
		if let Some(new_sector) = entries.get(sector as usize) {
			sector = *new_sector;
		} else {
			sector = fat::ENDOFCHAIN;
		}
	}
	if size > bytes.len() || size < bytes.len() - header.sector_size() {
		return Err("FAT stream size does not match number of sectors".into());
	}
	bytes.resize(size, 0);
	Ok(bytes)
}

fn get_minifat_data(entries: &[u32], mini_stream_bytes: &[u8], mut sector: u32, size: usize) -> BoxResult<Vec<u8>> {
	let mut bytes = Vec::new();
	while sector != fat::ENDOFCHAIN {
		let start = (sector as usize) * fat::MINIFAT_SECTOR_SIZE;
		bytes.extend_from_slice(&mini_stream_bytes[start..start+(fat::MINIFAT_SECTOR_SIZE)]);
		if let Some(new_sector) = entries.get(sector as usize) {
			sector = *new_sector;
		} else {
			sector = fat::ENDOFCHAIN;
		}
	}
	bytes.resize(size, 0);
	Ok(bytes)
}

fn get_directory_data<R: Read + Seek>(reader: &mut R, buf: &mut Vec<u8>, header: &CompoundFileHeader, fat: &[u32], minifat: &[u32], mini_stream_bytes: &[u8], entry: &Rc<dir::DirectoryEntry>) -> BoxResult<Vec<u8>> {
	if entry.object_type != dir::OBJECT_STREAM {
		return Err("Directory entry is not a stream object".into());
	}
	if entry.stream_size < header.mini_stream_cutoff_size as u64 {
		get_minifat_data(minifat, mini_stream_bytes, entry.starting_sector, entry.stream_size as usize)
	} else {
		get_fat_data(reader, buf, header, fat, entry.starting_sector, entry.stream_size as usize)
	}
}

fn extend_fat(buf: &[u8], header: &CompoundFileHeader, entries: &mut Vec<u32>) -> BoxResult<()> {
	match header.version_major {
		V3 => {
			let (_, fat) = fat::FatV3::parse(&buf).map_err(|err| BoxError::from(err.to_owned()))?;
			entries.extend_from_slice(fat.entries());
		}
		V4 => {
			let (_, fat) = fat::FatV4::parse(&buf).map_err(|err| BoxError::from(err.to_owned()))?;
			entries.extend_from_slice(fat.entries());
		}
		_ => {}
	}
	Ok(())
}

fn set_entry_children(entries: &mut Vec<Rc<dir::DirectoryEntry>>, entry_index: usize) -> BoxResult<()> {
	if entries.len() <= entry_index {
		return Ok(())
	}
	if entries[entry_index].child_id == dir::NOSTREAM {
		return Ok(())
	}
	let mut child_ids_queue = Vec::new();
	child_ids_queue.push(entries[entry_index].child_id);
	while let Some(child_id) = child_ids_queue.pop() {
		if let Some(child_entry) = entries.get(child_id as usize) {
			if entries[entry_index].children.borrow().contains_key(&child_entry.name) {
				return Err(format!("Duplicate directory entry name {}", child_entry.name).into());
			}
			entries[entry_index].children.borrow_mut().insert(child_entry.name.clone(), entries[child_id as usize].clone());
			if child_entry.left_sibling_id != dir::NOSTREAM {
				child_ids_queue.push(child_entry.left_sibling_id);
			}
			if child_entry.right_sibling_id != dir::NOSTREAM {
				child_ids_queue.push(child_entry.right_sibling_id);
			}
			if child_entry.child_id != dir::NOSTREAM {
				set_entry_children(entries, child_id as usize)?;
			}
		} else {
			return Err(format!("Invalid child ID {}", child_id).into());
		}
	}
	Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundFile {
	pub header: CompoundFileHeader,
	pub fat: Vec<u32>,
	pub minifat: Vec<u32>,
	pub dirs: Vec<Rc<dir::DirectoryEntry>>,
}

impl CompoundFile {
	pub fn parse_from_reader<R: Read + Seek>(reader: &mut R) -> BoxResult<Self> {
		let mut buf = Vec::with_capacity(512);

		// read header
		buf.resize(HEADER_SIZE, 0);
		reader.read_exact(&mut buf)?;
		let (_, header) = CompoundFileHeader::parse(&buf).map_err(|err| BoxError::from(err.to_owned()))?;

		// reserve buffer space for a single sector
		buf.reserve(header.sector_size() - buf.len());

		// get FAT
		let mut fat = Vec::new();
		for sector in header.difat {
			if sector == fat::FREESECT {
				break
			}
			get_sector_bytes(reader, &mut buf, &header, sector)?;
			extend_fat(&buf, &header, &mut fat)?;
		}

		// get MiniFAT
		let mut minifat = Vec::new();
		let mut sector = header.minifat_first_sector;
		while sector != fat::ENDOFCHAIN {
			get_sector_bytes(reader, &mut buf, &header, sector)?;
			extend_fat(&buf, &header, &mut minifat)?;
			if let Some(new_sector) = fat.get(sector as usize) {
				sector = *new_sector;
			} else {
				sector = fat::ENDOFCHAIN;
			}
		}

		// get DirectoryEntry
		let sector_directory_entry_count = header.sector_size() / dir::ENTRY_SIZE;
		let mut dirs = Vec::new();
		let mut sector = header.dir_first_sector;
		while sector != fat::ENDOFCHAIN {
			get_sector_bytes(reader, &mut buf, &header, sector)?;
			let mut input: &[u8] = &buf;
			for _ in 0..sector_directory_entry_count {
				let (new_input, entry) = dir::DirectoryEntry::parse(input).map_err(|err| BoxError::from(err.to_owned()))?;
				input = new_input;
				dirs.push(Rc::new(entry));
			}
			if let Some(new_sector) = fat.get(sector as usize) {
				sector = *new_sector;
			} else {
				sector = fat::ENDOFCHAIN;
			}
		}

		// establish DirectoryEntry hierarchy
		set_entry_children(&mut dirs, 0)?;

		// get mini stream data
		let mini_stream_sector = dirs[0].starting_sector;
		let mini_stream_size = dirs[0].stream_size as usize;
		let mini_stream_bytes = if mini_stream_sector == fat::ENDOFCHAIN {
			Vec::new()
		} else {
			get_fat_data(reader, &mut buf, &header, &fat, mini_stream_sector, mini_stream_size)?
		};

		// get DirectoryEntry data
		for entry in &dirs {
			if entry.object_type != dir::OBJECT_STREAM {
				continue
			}
			let mut data = entry.data.borrow_mut();
			*data = get_directory_data(reader, &mut buf, &header, &fat, &minifat, &mini_stream_bytes, &entry)?;
		}

		Ok(Self {
			header,
			fat,
			minifat,
			dirs,
		})
	}
}