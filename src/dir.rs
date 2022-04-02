use crate::oxcdata::{date_opt, complete_utf16le_string};

use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{Formatter, Result, Display};

use chrono::{DateTime, Utc};
use nom::{
	IResult,
	bytes::streaming::take,
	number::streaming::{u8, le_u16, le_u32, le_u64},
	combinator::map_res,
};

pub const ENTRY_SIZE: usize = 128;
pub const OBJECT_UNKNOWN: u8 = 0x00;
pub const OBJECT_STORAGE: u8 = 0x01; // folder
pub const OBJECT_STREAM: u8 = 0x02; // file
pub const OBJECT_ROOT_STORAGE: u8 = 0x05;
pub const MAXREGSID: u32 = 0xFFFFFFFA;
pub const NOSTREAM: u32 = 0xFFFFFFFF;
pub const COLOR_RED: u8 = 0x00;
pub const COLOR_BLACK: u8 = 0x01;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
	pub name: String,
	pub object_type: u8,
	pub color_flag: u8,
	pub left_sibling_id: u32,
	pub right_sibling_id: u32,
	pub child_id: u32,
	pub clsid: [u8; 16],
	pub state_bits: u32,
	pub creation_time: Option<DateTime<Utc>>,
	pub modified_time: Option<DateTime<Utc>>,
	pub starting_sector: u32,
	pub stream_size: u64,
	pub children: RefCell<BTreeMap<String, Rc<DirectoryEntry>>>,
	pub data: RefCell<Vec<u8>>,
}

impl DirectoryEntry {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, name) = take(64u16)(input)?;
		let (input, name_len) = le_u16(input)?;
		let (_, name) = complete_utf16le_string(&name[..name_len as usize])?;
		let (input, object_type) = u8(input)?;
		let (input, color_flag) = u8(input)?;
		let (input, left_sibling_id) = le_u32(input)?;
		let (input, right_sibling_id) = le_u32(input)?;
		let (input, child_id) = le_u32(input)?;
		let (input, clsid) = map_res(take(16usize), |b: &[u8]| b.try_into())(input)?;
		let (input, state_bits) = le_u32(input)?;
		let (input, creation_time) = date_opt(input)?;
		let (input, modified_time) = date_opt(input)?;
		let (input, starting_sector) = le_u32(input)?;
		let (input, stream_size) = le_u64(input)?;
		Ok((input, Self {
			name,
			object_type,
			color_flag,
			left_sibling_id,
			right_sibling_id,
			child_id,
			clsid,
			state_bits,
			creation_time,
			modified_time,
			starting_sector,
			stream_size,
			..Default::default()
		}))
	}

	fn list_children(&self, f: &mut Formatter<'_>, level: usize, expand: bool) -> Result {
		let line_prefix = "\t".repeat(level);
		for (name, child_entry) in self.children.borrow().iter() {
			if child_entry.object_type == OBJECT_STORAGE {
				writeln!(f, "{}{} ({} children)", line_prefix, name, child_entry.children.borrow().len())?;
			} else {
				writeln!(f, "{}{}", line_prefix, name)?;
			}
			if expand {
				child_entry.list_children(f, level + 1, expand)?;
			}
		}
		Ok(())
	}
}

impl Display for DirectoryEntry {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		if self.object_type == OBJECT_ROOT_STORAGE || self.object_type == OBJECT_STORAGE {
			writeln!(f, "{} ({} children)", self.name, self.children.borrow().len())?;
		} else {
			writeln!(f, "{}", self.name)?;
		}
		self.list_children(f, 1, true)
	}
}

impl Default for DirectoryEntry {
	fn default() -> Self {
		Self {
			name: String::new(),
			object_type: OBJECT_UNKNOWN,
			color_flag: COLOR_RED,
			left_sibling_id: NOSTREAM,
			right_sibling_id: NOSTREAM,
			child_id: NOSTREAM,
			clsid: [0; 16],
			state_bits: 0x00000000,
			creation_time: None,
			modified_time: None,
			starting_sector: 0x00000000,
			stream_size: 0x0000000000000000,
			children: RefCell::new(BTreeMap::new()),
			data: RefCell::new(Vec::new()),
		}
	}
}