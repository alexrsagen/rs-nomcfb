use crate::oxcdata::{PropertyEntry, PropertyValue};
use crate::oxmsg::PropertyStream;
use crate::oxomsg::RecipientType;
use crate::oxcprpt::ObjectType;
use crate::oxnspi::DisplayType;

use chrono::{DateTime, Utc};

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnownPropertyId {
	Unspecified = 0x0000,
	Importance = 0x0017,
	MessageClassW = 0x001A,
	Sensitivity = 0x0036,
	SubjectW = 0x0037,
	ClientSubmitTime = 0x0039,
	SentRepresentingNameW = 0x0042,
	MessageToMe = 0x0057,
	MessageCcMe = 0x0058,
	ConversationTopicW = 0x0070,
	ConversationIndex = 0x0071,
	OriginalDisplayTo = 0x0074,
	TransportMessageHeaders = 0x007D,
	RecipientType = 0x0c15,
	SenderName = 0x0C1A,
	DisplayCcW = 0x0E03,
	DisplayToW = 0x0E04,
	MessageDeliveryTime = 0x0E06,
	MessageFlags = 0x0E07,
	MessageSize = 0x0E08,
	Responsibility = 0x0E0F,
	MessageStatus = 0x0E17,
	HasAttachments = 0x0E1B,
	AttachmentSize = 0x0E20,
	ReplItemid = 0x0E30,
	ReplChangenum = 0x0E33,
	ReplVersionHistory = 0x0E34,
	ReplFlags = 0x0E38,
	ReplCopiedfromVersionhistory = 0x0E3C,
	ReplCopiedfromItemid = 0x0E3D,
	Read = 0x0E69,
	RecordKey = 0x0FF9,
	ObjectType = 0x0FFE,
	EntryID = 0x0FFF,
	Body = 0x1000,
	RtfCompressed = 0x1009,
	ItemTemporaryFlags = 0x1097,
	DisplayName = 0x3001,
	AddressType = 0x3002,
	EmailAddress = 0x3003,
	CreationTime = 0x3007,
	LastModificationTime = 0x3008,
	SearchKey = 0x300B,
	IpmSubTreeEntryId = 0x35E0,
	IpmWastebasketEntryId = 0x35E3,
	FinderEntryId = 0x35E7,
	ContentCount = 0x3602,
	ContentUnreadCount = 0x3603,
	Subfolders = 0x360A,
	ContainerClass = 0x3613,
	AttachData = 0x3701,
	AttachExtension = 0x3703,
	AttachFilename = 0x3704,
	AttachMethod = 0x3705,
	AttachLongFilename = 0x3707,
	RenderingPosition = 0x370B,
	AttachMimeTag = 0x370E,
	DisplayType = 0x3900,
	SevenBitDisplayName = 0x39FF,
	SendRichInfo = 0x3A40,
	SenderSmtpAddress = 0x5D01,
	SentRepresentingSmtpAddress = 0x5D02,
	SecureSubmitFlags = 0x65C6,
	PstBestBodyProptag = 0x661D,
	PstHiddenCount = 0x6635,
	PstHiddenUnread = 0x6636,
	PstIpmsubTreeDescendant = 0x6705,
	PstSubTreeContainer = 0x6772,
	LtpParentNid = 0x67F1,
	LtpRowId = 0x67F2,
	LtpRowVer = 0x67F3,
	PstPassword = 0x67FF,
	OfflineAddressBookName = 0x6800,
	SendOutlookRecallReport = 0x6803,
	OfflineAddressBookTruncatedProperties = 0x6805,
	MapiFormComposeCommand = 0x682F,
	ViewDescriptorFlags = 0x7003,
	ViewDescriptorLinkTo = 0x7004,
	ViewDescriptorViewFolder = 0x7005,
	ViewDescriptorName = 0x7006,
	ViewDescriptorVersion = 0x7007,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertyId {
	Unknown(u16),
	Known(KnownPropertyId),
}

impl PropertyId {
	pub fn from_u16(input: u16) -> Self {
		match input {
			0x0000 => Self::Known(KnownPropertyId::Unspecified),
			0x0017 => Self::Known(KnownPropertyId::Importance),
			0x001A => Self::Known(KnownPropertyId::MessageClassW),
			0x0036 => Self::Known(KnownPropertyId::Sensitivity),
			0x0037 => Self::Known(KnownPropertyId::SubjectW),
			0x0039 => Self::Known(KnownPropertyId::ClientSubmitTime),
			0x0042 => Self::Known(KnownPropertyId::SentRepresentingNameW),
			0x0057 => Self::Known(KnownPropertyId::MessageToMe),
			0x0058 => Self::Known(KnownPropertyId::MessageCcMe),
			0x0070 => Self::Known(KnownPropertyId::ConversationTopicW),
			0x0071 => Self::Known(KnownPropertyId::ConversationIndex),
			0x0074 => Self::Known(KnownPropertyId::OriginalDisplayTo),
			0x007D => Self::Known(KnownPropertyId::TransportMessageHeaders),
			0x0c15 => Self::Known(KnownPropertyId::RecipientType),
			0x0C1A => Self::Known(KnownPropertyId::SenderName),
			0x0E03 => Self::Known(KnownPropertyId::DisplayCcW),
			0x0E04 => Self::Known(KnownPropertyId::DisplayToW),
			0x0E06 => Self::Known(KnownPropertyId::MessageDeliveryTime),
			0x0E07 => Self::Known(KnownPropertyId::MessageFlags),
			0x0E08 => Self::Known(KnownPropertyId::MessageSize),
			0x0E0F => Self::Known(KnownPropertyId::Responsibility),
			0x0E17 => Self::Known(KnownPropertyId::MessageStatus),
			0x0E1B => Self::Known(KnownPropertyId::HasAttachments),
			0x0E20 => Self::Known(KnownPropertyId::AttachmentSize),
			0x0E30 => Self::Known(KnownPropertyId::ReplItemid),
			0x0E33 => Self::Known(KnownPropertyId::ReplChangenum),
			0x0E34 => Self::Known(KnownPropertyId::ReplVersionHistory),
			0x0E38 => Self::Known(KnownPropertyId::ReplFlags),
			0x0E3C => Self::Known(KnownPropertyId::ReplCopiedfromVersionhistory),
			0x0E3D => Self::Known(KnownPropertyId::ReplCopiedfromItemid),
			0x0E69 => Self::Known(KnownPropertyId::Read),
			0x0FF9 => Self::Known(KnownPropertyId::RecordKey),
			0x0FFE => Self::Known(KnownPropertyId::ObjectType),
			0x0FFF => Self::Known(KnownPropertyId::EntryID),
			0x1000 => Self::Known(KnownPropertyId::Body),
			0x1009 => Self::Known(KnownPropertyId::RtfCompressed),
			0x1097 => Self::Known(KnownPropertyId::ItemTemporaryFlags),
			0x3001 => Self::Known(KnownPropertyId::DisplayName),
			0x3002 => Self::Known(KnownPropertyId::AddressType),
			0x3003 => Self::Known(KnownPropertyId::EmailAddress),
			0x3007 => Self::Known(KnownPropertyId::CreationTime),
			0x3008 => Self::Known(KnownPropertyId::LastModificationTime),
			0x300B => Self::Known(KnownPropertyId::SearchKey),
			0x35E0 => Self::Known(KnownPropertyId::IpmSubTreeEntryId),
			0x35E3 => Self::Known(KnownPropertyId::IpmWastebasketEntryId),
			0x35E7 => Self::Known(KnownPropertyId::FinderEntryId),
			0x3602 => Self::Known(KnownPropertyId::ContentCount),
			0x3603 => Self::Known(KnownPropertyId::ContentUnreadCount),
			0x360A => Self::Known(KnownPropertyId::Subfolders),
			0x3613 => Self::Known(KnownPropertyId::ContainerClass),
			0x3701 => Self::Known(KnownPropertyId::AttachData),
			0x3703 => Self::Known(KnownPropertyId::AttachExtension),
			0x3704 => Self::Known(KnownPropertyId::AttachFilename),
			0x3705 => Self::Known(KnownPropertyId::AttachMethod),
			0x3707 => Self::Known(KnownPropertyId::AttachLongFilename),
			0x370B => Self::Known(KnownPropertyId::RenderingPosition),
			0x370E => Self::Known(KnownPropertyId::AttachMimeTag),
			0x3900 => Self::Known(KnownPropertyId::DisplayType),
			0x39FF => Self::Known(KnownPropertyId::SevenBitDisplayName),
			0x3A40 => Self::Known(KnownPropertyId::SendRichInfo),
			0x5D01 => Self::Known(KnownPropertyId::SenderSmtpAddress),
			0x5D02 => Self::Known(KnownPropertyId::SentRepresentingSmtpAddress),
			0x65C6 => Self::Known(KnownPropertyId::SecureSubmitFlags),
			0x661D => Self::Known(KnownPropertyId::PstBestBodyProptag),
			0x6635 => Self::Known(KnownPropertyId::PstHiddenCount),
			0x6636 => Self::Known(KnownPropertyId::PstHiddenUnread),
			0x6705 => Self::Known(KnownPropertyId::PstIpmsubTreeDescendant),
			0x6772 => Self::Known(KnownPropertyId::PstSubTreeContainer),
			0x67F1 => Self::Known(KnownPropertyId::LtpParentNid),
			0x67F2 => Self::Known(KnownPropertyId::LtpRowId),
			0x67F3 => Self::Known(KnownPropertyId::LtpRowVer),
			0x67FF => Self::Known(KnownPropertyId::PstPassword),
			0x6800 => Self::Known(KnownPropertyId::OfflineAddressBookName),
			0x6803 => Self::Known(KnownPropertyId::SendOutlookRecallReport),
			0x6805 => Self::Known(KnownPropertyId::OfflineAddressBookTruncatedProperties),
			0x682F => Self::Known(KnownPropertyId::MapiFormComposeCommand),
			0x7003 => Self::Known(KnownPropertyId::ViewDescriptorFlags),
			0x7004 => Self::Known(KnownPropertyId::ViewDescriptorLinkTo),
			0x7005 => Self::Known(KnownPropertyId::ViewDescriptorViewFolder),
			0x7006 => Self::Known(KnownPropertyId::ViewDescriptorName),
			0x7007 => Self::Known(KnownPropertyId::ViewDescriptorVersion),
			value => Self::Unknown(value),
		}
	}
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnownAttachMethod {
	None = 0x00000000,
	ByValue = 0x00000001,
	ByReference = 0x00000002,
	ByReferenceOnly = 0x00000004,
	EmbeddedMessage = 0x00000005,
	Storage = 0x00000006,
	ByWebReference = 0x00000007,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttachMethod {
	Unknown(i32),
	Known(KnownAttachMethod),
}

impl AttachMethod {
	pub fn from_i32(input: i32) -> Self {
		match input {
			0x00000000 => Self::Known(KnownAttachMethod::None),
			0x00000001 => Self::Known(KnownAttachMethod::ByValue),
			0x00000002 => Self::Known(KnownAttachMethod::ByReference),
			0x00000004 => Self::Known(KnownAttachMethod::ByReferenceOnly),
			0x00000005 => Self::Known(KnownAttachMethod::EmbeddedMessage),
			0x00000006 => Self::Known(KnownAttachMethod::Storage),
			0x00000007 => Self::Known(KnownAttachMethod::ByWebReference),
			value => Self::Unknown(value),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Recipient<'a> {
	pub recipient_type: Option<&'a PropertyEntry>,
	pub display_name: Option<&'a PropertyEntry>,
	pub object_type: Option<&'a PropertyEntry>,
	pub address_type: Option<&'a PropertyEntry>,
	pub email_address: Option<&'a PropertyEntry>,
	pub display_type: Option<&'a PropertyEntry>,
}

impl<'a> From<&'a PropertyStream> for Recipient<'a> {
	fn from(input: &'a PropertyStream) -> Self {
		Self {
			recipient_type: input.properties.get(&PropertyId::Known(KnownPropertyId::RecipientType)),
			display_name: input.properties.get(&PropertyId::Known(KnownPropertyId::DisplayName)),
			object_type: input.properties.get(&PropertyId::Known(KnownPropertyId::ObjectType)),
			address_type: input.properties.get(&PropertyId::Known(KnownPropertyId::AddressType)),
			email_address: input.properties.get(&PropertyId::Known(KnownPropertyId::EmailAddress)),
			display_type: input.properties.get(&PropertyId::Known(KnownPropertyId::DisplayType)),
		}
	}
}

impl<'a> Recipient<'a> {
	pub fn to_owned(&self) -> RecipientOwned {
		let recipient_type = if let Some(value) = self.recipient_type {
			if let PropertyValue::Integer32(value) = &value.value {
				Some(RecipientType::from_i32(*value))
			} else {
				None
			}
		} else {
			None
		};
		let display_name = if let Some(value) = self.display_name {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let object_type = if let Some(value) = self.object_type {
			if let PropertyValue::Integer32(value) = &value.value {
				Some(ObjectType::from_i32(*value))
			} else {
				None
			}
		} else {
			None
		};
		let address_type = if let Some(value) = self.address_type {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let email_address = if let Some(value) = self.email_address {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let display_type = if let Some(value) = self.display_type {
			if let PropertyValue::Integer32(value) = &value.value {
				Some(DisplayType::from_i32(*value))
			} else {
				None
			}
		} else {
			None
		};
		RecipientOwned {
			recipient_type,
			display_name,
			object_type,
			address_type,
			email_address,
			display_type,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipientOwned {
	pub recipient_type: Option<RecipientType>,
	pub display_name: Option<String>,
	pub object_type: Option<ObjectType>,
	pub address_type: Option<String>,
	pub email_address: Option<String>,
	pub display_type: Option<DisplayType>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Attachment<'a> {
	pub display_name: Option<&'a PropertyEntry>,
	pub attach_method: Option<&'a PropertyEntry>,
	pub size: Option<&'a PropertyEntry>,
	pub short_filename: Option<&'a PropertyEntry>,
	pub long_filename: Option<&'a PropertyEntry>,
	pub data: Option<&'a PropertyEntry>,
	pub mime_tag: Option<&'a PropertyEntry>,
	pub extension: Option<&'a PropertyEntry>,
}

impl<'a> From<&'a PropertyStream> for Attachment<'a> {
	fn from(input: &'a PropertyStream) -> Self {
		Self {
			display_name: input.properties.get(&PropertyId::Known(KnownPropertyId::DisplayName)),
			attach_method: input.properties.get(&PropertyId::Known(KnownPropertyId::AttachMethod)),
			size: input.properties.get(&PropertyId::Known(KnownPropertyId::AttachmentSize)),
			short_filename: input.properties.get(&PropertyId::Known(KnownPropertyId::AttachFilename)),
			long_filename: input.properties.get(&PropertyId::Known(KnownPropertyId::AttachLongFilename)),
			data: input.properties.get(&PropertyId::Known(KnownPropertyId::AttachData)),
			mime_tag: input.properties.get(&PropertyId::Known(KnownPropertyId::AttachMimeTag)),
			extension: input.properties.get(&PropertyId::Known(KnownPropertyId::AttachExtension)),
		}
	}
}

impl<'a> Attachment<'a> {
	pub fn to_owned(&self) -> AttachmentOwned {
		let display_name = if let Some(value) = self.display_name {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let attach_method = if let Some(value) = self.attach_method {
			if let PropertyValue::Integer32(value) = &value.value {
				Some(AttachMethod::from_i32(*value))
			} else {
				None
			}
		} else {
			None
		};
		let size = if let Some(value) = self.size {
			if let PropertyValue::Integer32(value) = &value.value {
				Some(*value)
			} else {
				None
			}
		} else {
			None
		};
		let short_filename = if let Some(value) = self.short_filename {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let long_filename = if let Some(value) = self.long_filename {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let data = if let Some(value) = self.data {
			if let PropertyValue::Binary(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let mime_tag = if let Some(value) = self.mime_tag {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let extension = if let Some(value) = self.extension {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		AttachmentOwned {
			display_name,
			attach_method,
			size,
			short_filename,
			long_filename,
			data,
			mime_tag,
			extension,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentOwned {
	pub display_name: Option<String>,
	pub attach_method: Option<AttachMethod>,
	pub size: Option<i32>,
	pub short_filename: Option<String>,
	pub long_filename: Option<String>,
	pub data: Option<Vec<u8>>,
	pub mime_tag: Option<String>,
	pub extension: Option<String>,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageFlags {
	Read = 0x00000001,
	Unsent = 0x00000008,
}

impl MessageFlags {
	pub fn from_i32(input: i32) -> Option<Self> {
		match input {
			0x00000001 => Some(Self::Read),
			0x00000008 => Some(Self::Unsent),
			_ => None,
		}
	}
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnownMessageStatus {
	RemoteDownload = 0x00001000,
	InConflict = 0x00000800,
	RemoteDelete = 0x00002000,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageStatus {
	Unknown(i32),
	Known(KnownMessageStatus),
}

impl MessageStatus {
	pub fn from_i32(input: i32) -> Self {
		match input {
			0x00001000 => Self::Known(KnownMessageStatus::RemoteDownload),
			0x00000800 => Self::Known(KnownMessageStatus::InConflict),
			0x00002000 => Self::Known(KnownMessageStatus::RemoteDelete),
			value => Self::Unknown(value),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Message<'a> {
	pub subject: Option<&'a PropertyEntry>,
	pub client_submit_time: Option<&'a PropertyEntry>,
	pub sent_representing_name: Option<&'a PropertyEntry>,
	pub sender_name: Option<&'a PropertyEntry>,
	pub sender_smtp_address: Option<&'a PropertyEntry>,
	pub delivery_time: Option<&'a PropertyEntry>,
	pub flags: Option<&'a PropertyEntry>,
	pub status: Option<&'a PropertyEntry>,
	pub size: Option<&'a PropertyEntry>,
	pub body: Option<&'a PropertyEntry>,
	pub transport_message_headers: Option<&'a PropertyEntry>,
	pub display_to: Option<&'a PropertyEntry>,
}

impl<'a> From<&'a PropertyStream> for Message<'a> {
	fn from(input: &'a PropertyStream) -> Self {
		Self {
			subject: input.properties.get(&PropertyId::Known(KnownPropertyId::SubjectW)),
			client_submit_time: input.properties.get(&PropertyId::Known(KnownPropertyId::ClientSubmitTime)),
			sent_representing_name: input.properties.get(&PropertyId::Known(KnownPropertyId::SentRepresentingNameW)),
			sender_name: input.properties.get(&PropertyId::Known(KnownPropertyId::SenderName)),
			sender_smtp_address: input.properties.get(&PropertyId::Known(KnownPropertyId::SenderSmtpAddress)),
			delivery_time: input.properties.get(&PropertyId::Known(KnownPropertyId::MessageDeliveryTime)),
			flags: input.properties.get(&PropertyId::Known(KnownPropertyId::MessageFlags)),
			status: input.properties.get(&PropertyId::Known(KnownPropertyId::MessageStatus)),
			size: input.properties.get(&PropertyId::Known(KnownPropertyId::MessageSize)),
			body: input.properties.get(&PropertyId::Known(KnownPropertyId::Body)),
			transport_message_headers: input.properties.get(&PropertyId::Known(KnownPropertyId::TransportMessageHeaders)),
			display_to: input.properties.get(&PropertyId::Known(KnownPropertyId::DisplayToW)),
		}
	}
}

impl<'a> Message<'a> {
	pub fn to_owned(&self) -> MessageOwned {
		let subject = if let Some(value) = self.subject {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let client_submit_time = if let Some(value) = self.client_submit_time {
			if let PropertyValue::Time(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let sent_representing_name = if let Some(value) = self.sent_representing_name {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let sender_name = if let Some(value) = self.sender_name {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let sender_smtp_address = if let Some(value) = self.sender_smtp_address {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let delivery_time = if let Some(value) = self.delivery_time {
			if let PropertyValue::Time(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let flags = if let Some(value) = self.flags {
			if let PropertyValue::Integer32(value) = &value.value {
				Some(*value)
			} else {
				None
			}
		} else {
			None
		};
		let status = if let Some(value) = self.status {
			if let PropertyValue::Integer32(value) = &value.value {
				Some(MessageStatus::from_i32(*value))
			} else {
				None
			}
		} else {
			None
		};
		let size = if let Some(value) = self.size {
			if let PropertyValue::Integer32(value) = &value.value {
				Some(*value)
			} else {
				None
			}
		} else {
			None
		};
		let body = if let Some(value) = self.body {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let transport_message_headers = if let Some(value) = self.transport_message_headers {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		let display_to = if let Some(value) = self.display_to {
			if let PropertyValue::String(value) = &value.value {
				Some(value.clone())
			} else {
				None
			}
		} else {
			None
		};
		MessageOwned {
			subject,
			client_submit_time,
			sent_representing_name,
			sender_name,
			sender_smtp_address,
			delivery_time,
			flags,
			status,
			size,
			body,
			transport_message_headers,
			display_to,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageOwned {
	pub subject: Option<String>,
	pub client_submit_time: Option<DateTime<Utc>>,
	pub sent_representing_name: Option<String>,
	pub sender_name: Option<String>,
	pub sender_smtp_address: Option<String>,
	pub delivery_time: Option<DateTime<Utc>>,
	pub flags: Option<i32>, // MessageFlags
	pub status: Option<MessageStatus>,
	pub size: Option<i32>,
	pub body: Option<String>,
	pub transport_message_headers: Option<String>,
	pub display_to: Option<String>,
}