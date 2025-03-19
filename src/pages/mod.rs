pub mod about_pc;
use crate::app;

#[derive(Debug, Clone)]
pub enum Message {
    AboutPc(about_pc::AboutPcPageMessage),
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        Self::Page(message)
    }
}
