use chrono::{DateTime, Utc};
use cosmic::{
    iced::{Length, Subscription},
    widget, Element, Task,
};
use futures_util::SinkExt as _;
use tokio_stream::StreamExt as _;

use crate::{app, fl, locale::get_locale, pages};

#[derive(Default)]
pub struct ClockPage {
    pub system_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum ClockPageMessage {
    SystemTimeTick(DateTime<Utc>),
}

impl From<ClockPageMessage> for app::UniAppMessage {
    fn from(message: ClockPageMessage) -> Self {
        pages::Message::Clock(message).into()
    }
}

#[allow(clippy::unused_self)]
impl pages::IPage<ClockPageMessage> for ClockPage {
    fn view(&self) -> Element<ClockPageMessage> {
        widget::container(
            widget::column().push(
                widget::text::title1(if let Some(system_time) = self.system_time {
                    system_time.format_localized("%T", get_locale()).to_string()
                } else {
                    fl!("system-time-na")
                })
                .font(cosmic::font::mono()),
            ),
        )
        .center(Length::Fill)
        .into()
    }

    fn update(&mut self, message: ClockPageMessage) -> Task<ClockPageMessage> {
        match message {
            ClockPageMessage::SystemTimeTick(date_time) => {
                self.system_time = Some(date_time);
            }
        }

        Task::none()
    }

    fn subscription(&self) -> Subscription<ClockPageMessage> {
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
