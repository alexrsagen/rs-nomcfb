#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnownRecipientType {
	MessageOriginator = 0x00000000,
	PrimaryRecipient = 0x00000001,
	CcRecipient = 0x00000002,
	BccRecipient = 0x00000003,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecipientType {
	Unknown(i32),
	Known(KnownRecipientType),
}

impl RecipientType {
	pub fn from_i32(input: i32) -> Self {
		match input {
			0x00000000 => Self::Known(KnownRecipientType::MessageOriginator),
			0x00000001 => Self::Known(KnownRecipientType::PrimaryRecipient),
			0x00000002 => Self::Known(KnownRecipientType::CcRecipient),
			0x00000003 => Self::Known(KnownRecipientType::BccRecipient),
			value => Self::Unknown(value),
		}
	}
}