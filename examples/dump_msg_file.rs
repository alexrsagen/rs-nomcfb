use nomcfb::error::BoxResult;
use nomcfb::oxcmsg::{Message, Recipient, RecipientOwned, Attachment, AttachmentOwned};
use std::fs::File;

fn main() -> BoxResult<()> {
	let args: Vec<String> = std::env::args().into_iter().collect();
	if args.len() < 2 {
		eprintln!("Usage: {} [filename]", &args[0]);
		std::process::exit(1);
	}

	let mut file = File::open(&args[1])?;
	let cfb = nomcfb::cfb::CompoundFile::parse_from_reader(&mut file)?;
	let msg_file = nomcfb::oxmsg::MsgFile::from_cfb(cfb)?;
	let message = Message::from(&msg_file.properties).to_owned();
	let recipients: Vec<RecipientOwned> = msg_file.recipients.iter().map(|recipient| Recipient::from(recipient).to_owned()).collect();
	let attachments: Vec<AttachmentOwned> = msg_file.attachments.iter().map(|attachment| Attachment::from(attachment).to_owned()).collect();

	println!("message: {:#?}", message);
	println!("recipients: {:#?}", recipients);
	println!("attachments: {:#?}", attachments);

	Ok(())
}