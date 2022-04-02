use crate::dir::DirectoryEntry;
use crate::oxcmsg::PropertyId;

use std::rc::Rc;

use encoding::all::UTF_16LE;
use encoding::{Encoding, DecoderTrap};
use chrono::{DateTime, Duration, Utc, TimeZone};
use nom::{
	IResult,
	bytes::streaming::{tag, take, take_until},
	number::streaming::{u8, le_i16, le_u16, le_i32, le_u32, le_f32, le_i64, le_f64},
	combinator::{map, map_opt, map_res},
	branch::alt,
	multi::{many0, count},
	sequence::terminated,
};

pub fn date(input: &[u8]) -> IResult<&[u8], DateTime<Utc>> {
	map_opt(
		le_i64,
		|hundred_ns_intervals| Utc.ymd(1601, 1, 1)
			.and_hms(0, 0, 0)
			.checked_add_signed(Duration::microseconds(hundred_ns_intervals / 10))
	)(input)
}

pub fn date_opt(input: &[u8]) -> IResult<&[u8], Option<DateTime<Utc>>> {
	alt((
		map(tag([0u8; 8]), |_| None),
		map(date, |value| Some(value)),
	))(input)
}

pub fn fdate(input: &[u8]) -> IResult<&[u8], DateTime<Utc>> {
	map_opt(
		le_f64,
		|days| Utc.ymd(1899, 12, 30)
			.and_hms(0, 0, 0)
			.checked_add_signed(Duration::days(days.trunc() as i64))
			.map(|datetime| datetime
				.checked_add_signed(Duration::hours((days.fract() * 24.0).round() as i64))
			)
			.flatten()
	)(input)
}

pub fn binary(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
	let (input, c) = le_u16(input)?;
	map(take(c), |b| Vec::from(b))(input)
}

pub fn guid(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
	map(take(16usize), |b| Vec::from(b))(input)
}

pub fn null_terminated_string(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
	terminated(map(take_until("\x00"), |b| Vec::from(b)), take(1usize))(input)
}

pub fn utf16le_string(len: usize) -> impl Fn(&[u8]) -> IResult<&[u8], String> {
	move |input| {
		map_res(take(len), |bytes| UTF_16LE.decode(bytes, DecoderTrap::Strict))(input)
	}
}

pub fn complete_utf16le_string(input: &[u8]) -> IResult<&[u8], String> {
	if input.len() == 0 {
		return Ok((input, String::new()))
	}
	if let Some(terminator_chunk) = input.chunks_exact(2).position(|chunk| chunk == b"\x00\x00") {
		let len = terminator_chunk * 2;
		terminated(map_res(take(len), |bytes| UTF_16LE.decode(bytes, DecoderTrap::Strict)), take(2usize))(input)
	} else if input.len() % 2 == 0 {
		map_res(take(input.len()), |bytes| UTF_16LE.decode(bytes, DecoderTrap::Strict))(input)
	} else {
		Err(nom::Err::Incomplete(nom::Needed::new(input.len() % 2)))
	}
}

pub fn complete_many_utf16le_string(input: &[u8]) -> IResult<&[u8], Vec<String>> {
	let num = input.chunks_exact(2).filter(|chunk| chunk == b"\x00\x00").count();
	count(complete_utf16le_string, num)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FolderId {
	pub replica_id: u16,
	pub global_counter: u64,
}

impl FolderId {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, replica_id) = le_u16(input)?;
		let (input, global_counter_bytes): (&[u8], [u8; 6]) = map_res(take(6usize), |b: &[u8]| b.try_into())(input)?;
		let global_counter_bytes = [
			global_counter_bytes[0],
			global_counter_bytes[1],
			global_counter_bytes[2],
			global_counter_bytes[3],
			global_counter_bytes[4],
			global_counter_bytes[5],
			0,
			0,
		];
		let global_counter = u64::from_be_bytes(global_counter_bytes);
		Ok((input, Self { replica_id, global_counter }))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageId {
	pub replica_id: u16,
	pub global_counter: u64,
}

impl MessageId {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, replica_id) = le_u16(input)?;
		let (input, global_counter_bytes): (&[u8], [u8; 6]) = map_res(take(6usize), |b: &[u8]| b.try_into())(input)?;
		let global_counter_bytes = [
			global_counter_bytes[0],
			global_counter_bytes[1],
			global_counter_bytes[2],
			global_counter_bytes[3],
			global_counter_bytes[4],
			global_counter_bytes[5],
			0,
			0,
		];
		let global_counter = u64::from_be_bytes(global_counter_bytes);
		Ok((input, Self { replica_id, global_counter }))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerId {
	pub ours: bool,
	pub folder_id: FolderId,
	pub msg_id: MessageId,
	pub instance: u32,
}

impl ServerId {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, ours) = map(u8, |value| value == 0x01)(input)?;
		let (input, folder_id) = FolderId::parse(input)?;
		let (input, msg_id) = MessageId::parse(input)?;
		let (input, instance) = le_u32(input)?;
		Ok((input, Self { ours, folder_id, msg_id, instance }))
	}
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnownPropertyType {
    Unspecified = 0x0000,
    Null = 0x0001,
	Integer16 = 0x0002,
    Integer32 = 0x0003,
    Integer64 = 0x0014,
    Floating32 = 0x0004,
    Floating64 = 0x0005,
    FloatingTime = 0x0007,
    Currency = 0x0006,
    ErrorCode = 0x000A,
    Boolean = 0x000B,
    Object = 0x000D,
    String = 0x001F,
    String8 = 0x001E,
    Time = 0x0040,
    Guid = 0x0048,
    ServerId = 0x00FB,
    Restriction = 0x00FD,
    RuleAction = 0x00FE,
    Binary = 0x0102,
    MultipleInteger16 = 0x1002,
    MultipleInteger32 = 0x1003,
    MultipleInteger64 = 0x1014,
    MultipleFloating32 = 0x1004,
    MultipleFloating64 = 0x1005,
    MultipleCurrency = 0x1006,
    MultipleFloatingTime = 0x1007,
    MultipleString = 0x101F,
    MultipleString8 = 0x101E,
    MultipleTime = 0x1040,
    MultipleGuid = 0x1048,
    MultipleBinary = 0x1102,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertyType {
	Unknown(u16),
	Known(KnownPropertyType),
}

impl PropertyType {
	pub fn from_u16(input: u16) -> Self {
		match input {
			0x0000 => Self::Known(KnownPropertyType::Unspecified),
			0x0001 => Self::Known(KnownPropertyType::Null),
			0x0002 => Self::Known(KnownPropertyType::Integer16),
			0x0003 => Self::Known(KnownPropertyType::Integer32),
			0x0014 => Self::Known(KnownPropertyType::Integer64),
			0x0004 => Self::Known(KnownPropertyType::Floating32),
			0x0005 => Self::Known(KnownPropertyType::Floating64),
			0x0007 => Self::Known(KnownPropertyType::FloatingTime),
			0x0006 => Self::Known(KnownPropertyType::Currency),
			0x000A => Self::Known(KnownPropertyType::ErrorCode),
			0x000B => Self::Known(KnownPropertyType::Boolean),
			0x000D => Self::Known(KnownPropertyType::Object),
			0x001F => Self::Known(KnownPropertyType::String),
			0x001E => Self::Known(KnownPropertyType::String8),
			0x0040 => Self::Known(KnownPropertyType::Time),
			0x0048 => Self::Known(KnownPropertyType::Guid),
			0x00FB => Self::Known(KnownPropertyType::ServerId),
			0x00FD => Self::Known(KnownPropertyType::Restriction),
			0x00FE => Self::Known(KnownPropertyType::RuleAction),
			0x0102 => Self::Known(KnownPropertyType::Binary),
			0x1002 => Self::Known(KnownPropertyType::MultipleInteger16),
			0x1003 => Self::Known(KnownPropertyType::MultipleInteger32),
			0x1014 => Self::Known(KnownPropertyType::MultipleInteger64),
			0x1004 => Self::Known(KnownPropertyType::MultipleFloating32),
			0x1005 => Self::Known(KnownPropertyType::MultipleFloating64),
			0x1006 => Self::Known(KnownPropertyType::MultipleCurrency),
			0x1007 => Self::Known(KnownPropertyType::MultipleFloatingTime),
			0x101F => Self::Known(KnownPropertyType::MultipleString),
			0x101E => Self::Known(KnownPropertyType::MultipleString8),
			0x1040 => Self::Known(KnownPropertyType::MultipleTime),
			0x1048 => Self::Known(KnownPropertyType::MultipleGuid),
			0x1102 => Self::Known(KnownPropertyType::MultipleBinary),
			value => Self::Unknown(value),
		}
	}

	pub fn is_variable(&self) -> bool {
		match self {
			Self::Known(KnownPropertyType::String) => true,
			Self::Known(KnownPropertyType::String8) => true,
			Self::Known(KnownPropertyType::Binary) => true,
			Self::Known(KnownPropertyType::Guid) => true,
			Self::Known(KnownPropertyType::Object) => true,
			Self::Known(KnownPropertyType::ServerId) => true,
			Self::Known(KnownPropertyType::RuleAction) => true,
			Self::Known(KnownPropertyType::Restriction) => true,
			Self::Known(KnownPropertyType::MultipleString) => true,
			Self::Known(KnownPropertyType::MultipleString8) => true,
			_ => false,
		}
	}

	pub fn is_multi(&self) -> bool {
		match self {
			Self::Known(KnownPropertyType::MultipleInteger16) => true,
			Self::Known(KnownPropertyType::MultipleInteger32) => true,
			Self::Known(KnownPropertyType::MultipleInteger64) => true,
			Self::Known(KnownPropertyType::MultipleFloating32) => true,
			Self::Known(KnownPropertyType::MultipleFloating64) => true,
			Self::Known(KnownPropertyType::MultipleCurrency) => true,
			Self::Known(KnownPropertyType::MultipleFloatingTime) => true,
			Self::Known(KnownPropertyType::MultipleTime) => true,
			Self::Known(KnownPropertyType::MultipleGuid) => true,
			Self::Known(KnownPropertyType::MultipleBinary) => true,
			Self::Known(KnownPropertyType::MultipleString) => true,
			Self::Known(KnownPropertyType::MultipleString8) => true,
			_ => false,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyTag {
	pub id: PropertyId,
	pub prop_type: PropertyType,
}

impl PropertyTag {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, prop_type) = map(le_u16, |prop_type| PropertyType::from_u16(prop_type))(input)?;
		let (input, id) = map(le_u16, |id| PropertyId::from_u16(id))(input)?;
		Ok((input, Self {
			id,
			prop_type,
		}))
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
	Unspecified(Vec<u8>),
	Null,
	Integer16(i16),
	Integer32(i32),
	Integer64(i64),
	Floating32(f32),
	Floating64(f64),
	FloatingTime(DateTime<Utc>),
	Currency(i64),
	ErrorCode(u32),
	Boolean(bool),
	Object(Vec<u8>),
	String(String),
	String8(Vec<u8>),
	Time(DateTime<Utc>),
	Guid(Vec<u8>),
	ServerId(ServerId),
	Restriction(Vec<u8>),
	RuleAction(Vec<u8>),
	Binary(Vec<u8>),
	MultipleInteger16(Vec<i16>),
	MultipleInteger32(Vec<i32>),
	MultipleInteger64(Vec<i64>),
	MultipleFloating32(Vec<f32>),
	MultipleFloating64(Vec<f64>),
	MultipleCurrency(Vec<i64>),
	MultipleFloatingTime(Vec<DateTime<Utc>>),
	MultipleString(Vec<String>),
	MultipleString8(Vec<Vec<u8>>),
	MultipleTime(Vec<DateTime<Utc>>),
	MultipleGuid(Vec<Vec<u8>>),
	MultipleBinary(Vec<Vec<u8>>),
}

impl PropertyValue {
	pub fn prop_type(&self) -> PropertyType {
		match self {
			Self::Unspecified(_) => PropertyType::Known(KnownPropertyType::Unspecified),
			Self::Null => PropertyType::Known(KnownPropertyType::Null),
			Self::Integer16(_) => PropertyType::Known(KnownPropertyType::Integer16),
			Self::Integer32(_) => PropertyType::Known(KnownPropertyType::Integer32),
			Self::Integer64(_) => PropertyType::Known(KnownPropertyType::Integer64),
			Self::Floating32(_) => PropertyType::Known(KnownPropertyType::Floating32),
			Self::Floating64(_) => PropertyType::Known(KnownPropertyType::Floating64),
			Self::FloatingTime(_) => PropertyType::Known(KnownPropertyType::FloatingTime),
			Self::Currency(_) => PropertyType::Known(KnownPropertyType::Currency),
			Self::ErrorCode(_) => PropertyType::Known(KnownPropertyType::ErrorCode),
			Self::Boolean(_) => PropertyType::Known(KnownPropertyType::Boolean),
			Self::Object(_) => PropertyType::Known(KnownPropertyType::Object),
			Self::String(_) => PropertyType::Known(KnownPropertyType::String),
			Self::String8(_) => PropertyType::Known(KnownPropertyType::String8),
			Self::Time(_) => PropertyType::Known(KnownPropertyType::Time),
			Self::Guid(_) => PropertyType::Known(KnownPropertyType::Guid),
			Self::ServerId(_) => PropertyType::Known(KnownPropertyType::ServerId),
			Self::Restriction(_) => PropertyType::Known(KnownPropertyType::Restriction),
			Self::RuleAction(_) => PropertyType::Known(KnownPropertyType::RuleAction),
			Self::Binary(_) => PropertyType::Known(KnownPropertyType::Binary),
			Self::MultipleInteger16(_) => PropertyType::Known(KnownPropertyType::MultipleInteger16),
			Self::MultipleInteger32(_) => PropertyType::Known(KnownPropertyType::MultipleInteger32),
			Self::MultipleInteger64(_) => PropertyType::Known(KnownPropertyType::MultipleInteger64),
			Self::MultipleFloating32(_) => PropertyType::Known(KnownPropertyType::MultipleFloating32),
			Self::MultipleFloating64(_) => PropertyType::Known(KnownPropertyType::MultipleFloating64),
			Self::MultipleCurrency(_) => PropertyType::Known(KnownPropertyType::MultipleCurrency),
			Self::MultipleFloatingTime(_) => PropertyType::Known(KnownPropertyType::MultipleFloatingTime),
			Self::MultipleString(_) => PropertyType::Known(KnownPropertyType::MultipleString),
			Self::MultipleString8(_) => PropertyType::Known(KnownPropertyType::MultipleString8),
			Self::MultipleTime(_) => PropertyType::Known(KnownPropertyType::MultipleTime),
			Self::MultipleGuid(_) => PropertyType::Known(KnownPropertyType::MultipleGuid),
			Self::MultipleBinary(_) => PropertyType::Known(KnownPropertyType::MultipleBinary),
		}
	}

	pub fn parse(input: &[u8], prop_type: PropertyType) -> IResult<&[u8], Self> {
		match prop_type {
			PropertyType::Unknown(_) => Ok((input, Self::Unspecified(Vec::new()))),
			PropertyType::Known(KnownPropertyType::Unspecified) => Ok((input, Self::Unspecified(Vec::new()))),
			PropertyType::Known(KnownPropertyType::Null) => Ok((input, Self::Null)),
			PropertyType::Known(KnownPropertyType::Integer16) => map(le_i16, |value| Self::Integer16(value))(input),
			PropertyType::Known(KnownPropertyType::Integer32) => map(le_i32, |value| Self::Integer32(value))(input),
			PropertyType::Known(KnownPropertyType::Integer64) => map(le_i64, |value| Self::Integer64(value))(input),
			PropertyType::Known(KnownPropertyType::Floating32) => map(le_f32, |value| Self::Floating32(value))(input),
			PropertyType::Known(KnownPropertyType::Floating64) => map(le_f64, |value| Self::Floating64(value))(input),
			PropertyType::Known(KnownPropertyType::FloatingTime) => map(fdate, |datetime| Self::FloatingTime(datetime))(input),
			PropertyType::Known(KnownPropertyType::Currency) => map(le_i64, |value| Self::Currency(value))(input),
			PropertyType::Known(KnownPropertyType::ErrorCode) => map(le_u32, |value| Self::ErrorCode(value))(input),
			PropertyType::Known(KnownPropertyType::Boolean) => map(u8, |value| Self::Boolean(value == 1))(input),
			PropertyType::Known(KnownPropertyType::Object) => { Ok((input, Self::Object(Vec::new()))) }
			PropertyType::Known(KnownPropertyType::String) => map(complete_utf16le_string, |value| Self::String(value))(input),
			PropertyType::Known(KnownPropertyType::String8) => map(null_terminated_string, |value| Self::String8(value))(input),
			PropertyType::Known(KnownPropertyType::Time) => map(date, |datetime| Self::Time(datetime))(input),
			PropertyType::Known(KnownPropertyType::Guid) => map(guid, |value| Self::Guid(value))(input),
			PropertyType::Known(KnownPropertyType::ServerId) => map(ServerId::parse, |value| Self::ServerId(value))(input),
			PropertyType::Known(KnownPropertyType::Restriction) => Ok((input, Self::Restriction(Vec::new()))),
			PropertyType::Known(KnownPropertyType::RuleAction) => map(binary, |value| Self::RuleAction(value))(input),
			PropertyType::Known(KnownPropertyType::Binary) => Ok((&input[input.len()..], Self::Binary(Vec::from(input)))),
			PropertyType::Known(KnownPropertyType::MultipleInteger16) => map(many0(le_i16), |value| Self::MultipleInteger16(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleInteger32) => map(many0(le_i32), |value| Self::MultipleInteger32(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleInteger64) => map(many0(le_i64), |value| Self::MultipleInteger64(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleFloating32) => map(many0(le_f32), |value| Self::MultipleFloating32(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleFloating64) => map(many0(le_f64), |value| Self::MultipleFloating64(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleCurrency) => map(many0(le_i64), |value| Self::MultipleCurrency(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleFloatingTime) => map(many0(fdate), |value| Self::MultipleFloatingTime(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleString) => map(complete_many_utf16le_string, |value| Self::MultipleString(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleString8) => map(many0(null_terminated_string), |value| Self::MultipleString8(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleTime) => map(many0(date), |value| Self::MultipleTime(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleGuid) => map(many0(guid), |value| Self::MultipleGuid(value))(input),
			PropertyType::Known(KnownPropertyType::MultipleBinary) => map(many0(binary), |value| Self::MultipleBinary(value))(input),
		}
	}
}

pub const SUB_PREFIX: &'static str = "__substg1.0_";

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyEntry {
	pub tag: PropertyTag,
	pub flags: u32,
	pub value: PropertyValue,
}

impl PropertyEntry {
	pub fn parse<'a>(input: &'a [u8], parent_dir: &Rc<DirectoryEntry>) -> IResult<&'a [u8], Self> {
		let (input, entry) = take(16usize)(input)?;
		let (entry, tag_raw) = take(4usize)(entry)?;
		let (_, tag) = PropertyTag::parse(tag_raw)?;
		let (_, tag_raw) = le_u32(tag_raw)?;
		let (entry, flags) = le_u32(entry)?;
		let value = if tag.prop_type.is_multi() || tag.prop_type.is_variable() {
			// variable-length value
			let (_entry, size) = le_u32(entry)?;
			// let (_entry, reserved) = le_u32(_entry)?;
			let stream_name = format!("{}{:08X}", SUB_PREFIX, tag_raw);
			if let Some(bytes) = parent_dir.children.borrow().get(&stream_name).map(|child| child.data.borrow()) {
				if bytes.len() != size as usize && ((tag.prop_type == PropertyType::Known(KnownPropertyType::String) && bytes.len() + 2 != size as usize) || (tag.prop_type == PropertyType::Known(KnownPropertyType::String8) && bytes.len() + 1 != size as usize)) {
					eprintln!("[debug] Stream {:?} invalid size {}, expected {}", &stream_name, bytes.len(), size);
					return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
				}
				if tag.prop_type.is_multi() && tag.prop_type.is_variable() {
					let len_item_size = if tag.prop_type == PropertyType::Known(KnownPropertyType::MultipleBinary) { 8usize } else { 4usize };
					let value_lengths: Vec<u32> = bytes.chunks(len_item_size).map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])).collect();
					let mut value_bytes = Vec::new();
					for i in 0..value_lengths.len() {
						let index_stream_name = format!("{}-{:08X}", stream_name, i);
						if let Some(index_stream_bytes) = parent_dir.children.borrow().get(&index_stream_name).map(|child| child.data.borrow()) {
							value_bytes.extend_from_slice(&index_stream_bytes);
						} else {
							eprintln!("[debug] Could not find stream {:?} in parent {:?}", &index_stream_name, &parent_dir.name);
							return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
						}
					}
					match PropertyValue::parse(&value_bytes, tag.prop_type) {
						Ok((_, value)) => value,
						Err(e) => {
							eprintln!("[debug] Could not parse property value of type {:?}: {}", tag.prop_type, e);
							return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail)))
						}
					}
				} else {
					match PropertyValue::parse(&bytes, tag.prop_type) {
						Ok((_, value)) => value,
						Err(e) => {
							eprintln!("[debug] Could not parse property value of type {:?}: {}", tag.prop_type, e);
							return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail)))
						}
					}
				}
			} else {
				eprintln!("[debug] Could not find stream {:?} in parent {:?}", &stream_name, &parent_dir.name);
				return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
			}
		} else {
			// fixed-length value
			match PropertyValue::parse(entry, tag.prop_type) {
				Ok((_, value)) => value,
				Err(e) => {
					eprintln!("[debug] Could not parse property value of type {:?}: {}", tag.prop_type, e);
					return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail)))
				}
			}
		};
		Ok((input, Self { tag, flags, value }))
	}
}