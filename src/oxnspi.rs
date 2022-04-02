#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnownDisplayType {
	MailUser = 0x00000000,
	DistList = 0x00000001,
	Forum = 0x00000002,
	Agent = 0x00000003,
	Organization = 0x00000004,
	PrivateDistList = 0x00000005,
	RemoteMailUser = 0x00000006,
	Container = 0x00000100,
	Template = 0x00000101,
	AddressTemplate = 0x00000102,
	Search = 0x00000200,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisplayType {
	Unknown(i32),
	Known(KnownDisplayType),
}

impl DisplayType {
	pub fn from_i32(input: i32) -> Self {
		match input {
			0x00000000 => Self::Known(KnownDisplayType::MailUser),
			0x00000001 => Self::Known(KnownDisplayType::DistList),
			0x00000002 => Self::Known(KnownDisplayType::Forum),
			0x00000003 => Self::Known(KnownDisplayType::Agent),
			0x00000004 => Self::Known(KnownDisplayType::Organization),
			0x00000005 => Self::Known(KnownDisplayType::PrivateDistList),
			0x00000006 => Self::Known(KnownDisplayType::RemoteMailUser),
			0x00000100 => Self::Known(KnownDisplayType::Container),
			0x00000101 => Self::Known(KnownDisplayType::Template),
			0x00000102 => Self::Known(KnownDisplayType::AddressTemplate),
			0x00000200 => Self::Known(KnownDisplayType::Search),
			value => Self::Unknown(value),
		}
	}
}