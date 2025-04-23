pub mod about_pc;
pub mod clock;
pub mod paid_entries;
pub mod preferences;

use crate::app;

pub trait IPage<T>: Default {
    fn view(&self) -> cosmic::Element<T>;

    fn subscription(&self) -> cosmic::iced::Subscription<T> {
        cosmic::iced::Subscription::none()
    }

    fn update(&mut self, message: T) -> cosmic::Task<T>;

    fn on_init(&self) -> cosmic::Task<T> {
        cosmic::Task::<T>::none()
    }

    fn dialog(&self) -> Option<cosmic::Element<T>> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    AboutPc(about_pc::AboutPcPageMessage),
    Clock(clock::ClockPageMessage),
    Preferences(preferences::PreferencesPageMessage),
    PaidEntries(paid_entries::PaidEntriesPageMessage),
}

impl From<Message> for app::UniAppMessage {
    fn from(message: Message) -> Self {
        Self::Page(message)
    }
}

/// The page to display in the application.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum Page {
    #[default]
    AboutPc,
    Clock,
    Preferences,
    PaidEntries,
}
