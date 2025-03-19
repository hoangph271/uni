use crate::fl;
use crate::locale::get_locale;
use crate::{app, pages};
use chrono::{DateTime, Utc};
use cosmic::iced::Subscription;
use cosmic::iced::{alignment::Horizontal, Alignment, Length};
use cosmic::theme;
use cosmic::widget;
use cosmic::{prelude::*, Task};
use futures_util::{SinkExt as _, StreamExt as _};

pub struct AboutPcPage {
    pub system_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum AboutPcPageMessage {
    SystemTimeTick(DateTime<Utc>),
}

impl From<AboutPcPageMessage> for app::Message {
    fn from(message: AboutPcPageMessage) -> Self {
        pages::Message::AboutPc(message).into()
    }
}

#[allow(clippy::unused_self)]
impl AboutPcPage {
    pub fn new() -> Self {
        Self { system_time: None }
    }

    pub fn view(&self) -> cosmic::Element<AboutPcPageMessage> {
        widget::container(
            widget::column()
                .push(
                    widget::text::title1(fl!("welcome"))
                        .apply(widget::container)
                        .width(Length::Fill)
                        .align_x(Horizontal::Center),
                )
                .push(widget::text::monotext(format!(
                    "{} - {}",
                    std::env::consts::OS,
                    std::env::consts::ARCH
                )))
                .push(widget::text::monotext(
                    if let Some(system_time) = self.system_time {
                        system_time.format_localized("%T", get_locale()).to_string()
                    } else {
                        fl!("system-time-na")
                    },
                ))
                .spacing(theme::active().cosmic().space_m())
                .align_x(Alignment::Center),
        )
        .center(Length::Fill)
        .into()
    }

    pub fn update(&mut self, message: &AboutPcPageMessage) -> Task<AboutPcPageMessage> {
        match message {
            AboutPcPageMessage::SystemTimeTick(date_time) => {
                self.system_time = Some(*date_time);
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<AboutPcPageMessage> {
        struct SystemTimeTickSubscription;

        Subscription::run_with_id(
            std::any::TypeId::of::<SystemTimeTickSubscription>(),
            cosmic::iced::stream::channel(
                std::mem::size_of::<AboutPcPageMessage>(),
                move |mut channel| async move {
                    let interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
                    let mut stream = tokio_stream::wrappers::IntervalStream::new(interval);

                    while stream.next().await.is_some() {
                        let system_time = chrono::Utc::now();

                        _ = channel
                            .send(AboutPcPageMessage::SystemTimeTick(system_time))
                            .await;
                    }
                },
            ),
        )
    }
}
