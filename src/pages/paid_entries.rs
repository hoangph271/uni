use std::borrow::Cow;

use crate::fl;
use crate::{app, config, pages};
use cosmic::widget::icon;
use cosmic::{cosmic_config, widget, Element, Task};

#[derive(Debug, Clone)]
pub enum PaidEntriesPageMessage {
    ShowJsonPicker,
}

#[derive(Default)]
pub struct PaidEntriesPage {
    config_handler: Option<cosmic_config::Config>,
    config: config::UniConfig,
}

impl PaidEntriesPage {
    #[must_use]
    pub fn new(config: config::UniConfig, config_handler: Option<cosmic_config::Config>) -> Self {
        Self {
            config_handler,
            config,
        }
    }
}

impl pages::IPage<PaidEntriesPageMessage> for PaidEntriesPage {
    fn view(&self) -> Element<PaidEntriesPageMessage> {
        widget::container(
            widget::column().push(
                widget::text_input(
                    fl!("json-path"),
                    self.config
                        .paid_entries_json_path
                        .as_ref()
                        .map_or(Cow::Owned(String::new()), |json_path| {
                            json_path.to_string_lossy()
                        }),
                )
                .label(fl!("json-path"))
                .trailing_icon(
                    widget::button::icon(icon::from_name("edit-symbolic"))
                        .on_press(PaidEntriesPageMessage::ShowJsonPicker)
                        .into(),
                ),
            ),
        )
        .into()
    }

    fn subscription(&self) -> cosmic::iced::Subscription<PaidEntriesPageMessage> {
        todo!()
    }

    fn update(&mut self, message: PaidEntriesPageMessage) -> cosmic::Task<PaidEntriesPageMessage> {
        match message {
            PaidEntriesPageMessage::ShowJsonPicker => {
                if let Some(json_path) = rfd::FileDialog::new()
                    .set_title(fl!("pick-json-dialog-title"))
                    .add_filter("json", &["json"])
                    .pick_file()
                {
                    if let Some(config_handler) = self.config_handler.as_ref() {
                        if let Err(e) = self
                            .config
                            .set_paid_entries_json_path(config_handler, Some(json_path))
                        {
                            tracing::error!("Error set_paid_entries_json_path: {e}");
                        } else {
                            // TODO: Read JSON, update paid entries
                        }
                    }
                }
            }
        }

        Task::none()
    }
}

impl From<PaidEntriesPageMessage> for app::Message {
    fn from(message: PaidEntriesPageMessage) -> Self {
        pages::Message::PaidEntries(message).into()
    }
}
