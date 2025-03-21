use crate::fl;
use crate::{app, pages};
use cosmic::widget;
use cosmic::{
    iced::{alignment::Horizontal, Alignment, Length},
    theme,
};
use cosmic::{prelude::*, Task};

pub struct AboutPcPage {}

#[derive(Debug, Clone)]
pub enum AboutPcPageMessage {}

impl From<AboutPcPageMessage> for app::Message {
    fn from(message: AboutPcPageMessage) -> Self {
        pages::Message::AboutPc(message).into()
    }
}

#[allow(clippy::unused_self)]
impl AboutPcPage {
    pub fn new() -> Self {
        Self {}
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
                .spacing(theme::active().cosmic().space_m())
                .align_x(Alignment::Center),
        )
        .center(Length::Fill)
        .into()
    }

    pub fn update(&self, _message: &AboutPcPageMessage) -> Task<AboutPcPageMessage> {
        Task::none()
    }
}
