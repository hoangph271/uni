pub mod about_pc;
pub mod clock;
pub mod paid_entries;
pub mod preferences;

use crate::app;

pub trait IPage<T>: Default {
    fn view(&self) -> cosmic::Element<T>;

    fn subscription(&self) -> cosmic::iced::Subscription<T>;

    fn update(&mut self, message: T) -> cosmic::Task<T>;
}

#[derive(Debug, Clone)]
pub enum Message {
    AboutPc(about_pc::AboutPcPageMessage),
    Clock(clock::ClockPageMessage),
    Preferences(preferences::PreferencesPageMessage),
    PaidEntries(paid_entries::PaidEntriesPageMessage),
}

impl From<Message> for app::Message {
    fn from(message: Message) -> Self {
        Self::Page(message)
    }
}
