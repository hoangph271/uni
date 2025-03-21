use chrono::{DateTime, Utc};
use cosmic::{
    iced::{Length, Subscription},
    widget, Element, Task,
};
use futures_util::SinkExt as _;
use tokio_stream::StreamExt as _;

use crate::{app, fl, locale::get_locale, pages};

pub struct ClockPage {
    pub system_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum ClockPageMessage {
    SystemTimeTick(DateTime<Utc>),
}

impl From<ClockPageMessage> for app::Message {
    fn from(message: ClockPageMessage) -> Self {
        pages::Message::Clock(message).into()
    }
}

#[allow(clippy::unused_self)]
impl ClockPage {
    pub fn new() -> Self {
        Self { system_time: None }
    }

    pub fn view(&self) -> Element<ClockPageMessage> {
        widget::container(widget::column().push(widget::text::monotext(
            if let Some(system_time) = self.system_time {
                system_time.format_localized("%T", get_locale()).to_string()
            } else {
                fl!("system-time-na")
            },
        )))
        .center(Length::Fill)
        .into()
    }

    pub fn update(&mut self, message: &ClockPageMessage) -> Task<ClockPageMessage> {
        match message {
            ClockPageMessage::SystemTimeTick(date_time) => {
                self.system_time = Some(*date_time);
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<ClockPageMessage> {
        struct SystemTimeTickSubscription;

        Subscription::run_with_id(
            std::any::TypeId::of::<SystemTimeTickSubscription>(),
            cosmic::iced::stream::channel(
                std::mem::size_of::<ClockPageMessage>(),
                move |mut channel| async move {
                    let interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
                    let mut stream = tokio_stream::wrappers::IntervalStream::new(interval);

                    while stream.next().await.is_some() {
                        let system_time = chrono::Utc::now();

                        _ = channel
                            .send(ClockPageMessage::SystemTimeTick(system_time))
                            .await;
                    }
                },
            ),
        )
    }
}
