use crate::fl;
use crate::{app, pages};
use cosmic::iced::Subscription;
use cosmic::widget;
use cosmic::{
    iced::{alignment::Horizontal, Alignment, Length},
    theme,
};
use cosmic::{prelude::*, Task};
use futures_util::SinkExt as _;

#[derive(Default)]
pub struct AboutPcPage {
    realname: Option<String>,
    distro: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AboutPcPageMessage {
    FetchedRealname(String),
    FetchedDistro(String),
}

impl From<AboutPcPageMessage> for app::Message {
    fn from(message: AboutPcPageMessage) -> Self {
        pages::Message::AboutPc(message).into()
    }
}

#[allow(clippy::unused_self)]
impl AboutPcPage {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn view(&self) -> cosmic::Element<AboutPcPageMessage> {
        widget::container(
            widget::column()
                .push(
                    widget::text::title1(if let Some(realname) = &self.realname {
                        fl!("welcome-user", name = realname)
                    } else {
                        fl!("welcome")
                    })
                    .apply(widget::container)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
                )
                .push_maybe(
                    self.distro
                        .as_ref()
                        .map(|distro| widget::text::monotext(distro).size(24)),
                )
                .push(widget::text::monotext(format!(
                    "{} - {}",
                    std::env::consts::OS,
                    std::env::consts::ARCH
                )))
                .spacing(theme::active().cosmic().space_m())
                .align_x(Alignment::Center),
        )
        .center(Length::Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<AboutPcPageMessage> {
        struct WhoamiSubscription;

        Subscription::run_with_id(
            std::any::TypeId::of::<WhoamiSubscription>(),
            cosmic::iced::stream::channel(
                std::mem::size_of::<AboutPcPageMessage>(),
                move |mut channel| async move {
                    _ = channel
                        .feed(AboutPcPageMessage::FetchedRealname(whoami::realname()))
                        .await;

                    _ = channel
                        .feed(AboutPcPageMessage::FetchedDistro(whoami::distro()))
                        .await;
                },
            ),
        )
    }

    pub fn update(&mut self, message: AboutPcPageMessage) -> Task<AboutPcPageMessage> {
        match message {
            AboutPcPageMessage::FetchedRealname(realname) => {
                self.realname = Some(realname);
            }
            AboutPcPageMessage::FetchedDistro(distro) => self.distro = Some(distro),
        }
        Task::none()
    }
}
