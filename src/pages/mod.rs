pub mod about_pc;

#[derive(Debug, Clone)]
pub enum Message {
    AboutPc(about_pc::AboutPcPageMessage),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> crate::app::Message {
        crate::app::Message::PageMessage(message)
    }
}
