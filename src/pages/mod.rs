pub mod about_pc;
pub mod clock;

use crate::app;

#[derive(Debug, Clone)]
pub enum Message {
    AboutPc(about_pc::AboutPcPageMessage),
    Clock(clock::ClockPageMessage),
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        Self::Page(message)
    }
}
