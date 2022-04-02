use crate::oxcdata::PropertyEntry;
use crate::oxcmsg::PropertyId;
use crate::dir::DirectoryEntry;
use crate::cfb::CompoundFile;
use crate::error::{BoxError, BoxResult};

use std::collections::BTreeMap;
use std::rc::Rc;

use nom::{
	IResult,
	multi::fold_many_m_n,
	combinator::{map, map_res},
	bytes::streaming::take,
	number::streaming::le_u32,
};

pub const PROPERTY_STREAM_NAME: &'static str = "__properties_version1.0";
pub const TOPLEVEL_HEADER_SIZE: usize = 32;
pub const RECIP_OR_ATTACH_HEADER_SIZE: usize = 8;
pub const EMBEDDED_MSG_HEADER_SIZE: usize = 24;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyStreamHeader {
	pub reserved: [u8; 8],
	pub next_recipient_id: u32,
	pub next_attachment_id: u32,
	pub recipient_count: u32,
	pub attachment_count: u32,
}

impl PropertyStreamHeader {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, reserved) = map_res(take(8usize), |value: &[u8]| value.try_into())(input)?;
		let (input, next_recipient_id) = le_u32(input)?;
		let (input, next_attachment_id) = le_u32(input)?;
		let (input, recipient_count) = le_u32(input)?;
		let (input, attachment_count) = le_u32(input)?;
		Ok((input, Self {
			reserved,
			next_recipient_id,
			next_attachment_id,
			recipient_count,
			attachment_count,
		}))
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyStream {
	pub header: PropertyStreamHeader,
	pub properties: BTreeMap<PropertyId, PropertyEntry>,
}

impl PropertyStream {
	pub fn parse<'a>(input: &'a [u8], header_size: usize, parent_dir: &'a Rc<DirectoryEntry>) -> IResult<&'a [u8], Self> {
		let (input, header_bytes) = take(header_size)(input)?;
		let header = if header_size >= EMBEDDED_MSG_HEADER_SIZE {
			let (_, header) = PropertyStreamHeader::parse(header_bytes)?;
			header
		} else {
			PropertyStreamHeader::default()
		};
		if input.len() % 16 != 0 {
			return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Count)));
		}
		let num = input.len() / 16;
		map(fold_many_m_n(num, num, |input| PropertyEntry::parse(input, parent_dir), BTreeMap::new, |mut map, prop| {
			map.insert(prop.tag.id, prop);
			map
		}), move |properties| Self { header, properties })(input)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct MsgFile {
	pub cfb: CompoundFile,
	pub properties: PropertyStream,
	pub recipients: Vec<PropertyStream>,
	pub attachments: Vec<PropertyStream>,
}

impl MsgFile {
	pub fn from_cfb(cfb: CompoundFile) -> BoxResult<Self> {
		let root_dir = &cfb.dirs[0];
		let properties = if let Some(property_stream) = root_dir.children.borrow().get(PROPERTY_STREAM_NAME) {
			let (_, properties) = PropertyStream::parse(&property_stream.data.borrow(), TOPLEVEL_HEADER_SIZE, root_dir).map_err(|err| BoxError::from(err.to_owned()))?;
			properties
		} else {
			return Err("Property stream missing".into())
		};

		let mut recipients = Vec::new();
		let mut i: u32 = 0;
		loop {
			let dir_name = format!("__recip_version1.0_#{:08X}", i);
			if let Some(dir_entry) = root_dir.children.borrow().get(&dir_name) {
				if let Some(property_stream) = dir_entry.children.borrow().get(PROPERTY_STREAM_NAME) {
					let (_, recipient) = PropertyStream::parse(&property_stream.data.borrow(), RECIP_OR_ATTACH_HEADER_SIZE, dir_entry).map_err(|err| BoxError::from(err.to_owned()))?;
					recipients.push(recipient);
				} else {
					break;
				}
			} else {
				break;
			}
			i += 1;
		}

		let mut attachments = Vec::new();
		i = 0;
		loop {
			let dir_name = format!("__attach_version1.0_#{:08X}", i);
			if let Some(dir_entry) = root_dir.children.borrow().get(&dir_name) {
				if let Some(property_stream) = dir_entry.children.borrow().get(PROPERTY_STREAM_NAME) {
					let (_, attachment) = PropertyStream::parse(&property_stream.data.borrow(), RECIP_OR_ATTACH_HEADER_SIZE, dir_entry).map_err(|err| BoxError::from(err.to_owned()))?;
					attachments.push(attachment);
				} else {
					break;
				}
			} else {
				break;
			}
			i += 1;
		}

		Ok(Self { cfb, properties, recipients, attachments })
	}
}