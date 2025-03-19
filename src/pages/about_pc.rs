use crate::fl;
use crate::locale::get_locale;
use chrono::{DateTime, Utc};
use cosmic::iced::{alignment::Horizontal, Alignment, Length};
use cosmic::prelude::*;
use cosmic::theme;
use cosmic::widget;

pub struct AboutPcPage {
    pub system_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum AboutPcPageMessage {}

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
}
