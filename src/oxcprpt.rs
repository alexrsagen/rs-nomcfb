#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnownObjectType {
	StoreObject = 0x00000001,
	AddressBookObject = 0x00000002,
	AddressBookContainer = 0x00000004,
	MessageObject = 0x00000005,
	MailUser = 0x00000006,
	AttachmentObject = 0x00000007,
	DistributionList = 0x00000008,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectType {
	Unknown(i32),
	Known(KnownObjectType),
}

impl ObjectType {
	pub fn from_i32(input: i32) -> Self {
		match input {
			0x00000001 => Self::Known(KnownObjectType::StoreObject),
			0x00000002 => Self::Known(KnownObjectType::AddressBookObject),
			0x00000004 => Self::Known(KnownObjectType::AddressBookContainer),
			0x00000005 => Self::Known(KnownObjectType::MessageObject),
			0x00000006 => Self::Known(KnownObjectType::MailUser),
			0x00000007 => Self::Known(KnownObjectType::AttachmentObject),
			0x00000008 => Self::Known(KnownObjectType::DistributionList),
			value => Self::Unknown(value),
		}
	}
}