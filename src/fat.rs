use nom::{
	IResult,
	number::streaming::le_u32,
	combinator::map_res,
	multi::count,
};

pub const MINIFAT_SECTOR_SIZE: usize = 64;

pub const MAXREGSECT: u32   = 0xFFFFFFFA; // Maximum regular sector number.
pub const RESERVEDSECT: u32 = 0xFFFFFFFB; // Reserved for future use.
pub const DIFSECT: u32      = 0xFFFFFFFC; // Specifies a DIFAT sector in the FAT
pub const FATSECT: u32      = 0xFFFFFFFD; // Specifies a FAT sector in the FAT
pub const ENDOFCHAIN: u32   = 0xFFFFFFFE; // End of a linked chain of sectors.
pub const FREESECT: u32     = 0xFFFFFFFF; // Specifies an unallocated sector in the FAT, Mini FAT, or DIFAT.

pub trait Fat<'a> {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> where Self: Sized;
	fn entries(&'a self) -> &'a [u32];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FatV3 {
	pub entries: [u32; 128],
}

impl<'a> Fat<'a> for FatV3 {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, entries) = map_res(count(le_u32, 128usize), |b: Vec<u32>| b.try_into())(input)?;
		Ok((input, Self { entries }))
	}
	fn entries(&'a self) -> &'a [u32] {
		&self.entries
	}
}

impl Default for FatV3 {
	fn default() -> Self {
		Self { entries: [0; 128] }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FatV4 {
	pub entries: [u32; 1024],
}

impl<'a> Fat<'a> for FatV4 {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, entries) = map_res(count(le_u32, 1024usize), |b: Vec<u32>| b.try_into())(input)?;
		Ok((input, Self { entries }))
	}
	fn entries(&'a self) -> &'a [u32] {
		&self.entries
	}
}

impl Default for FatV4 {
	fn default() -> Self {
		Self { entries: [0; 1024] }
	}
}