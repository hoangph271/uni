use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::fl;
use crate::{app, config, pages};
use cosmic::iced::alignment::Vertical;
use cosmic::widget::icon;
use cosmic::{cosmic_config, theme, widget, Apply, Element, Task};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum PaidEntriesPageMessage {
    ShowJsonPicker,
    RawJsonLoaded(RawJsonData),
    RawJsonUpdated(RawJsonData),
    RawJsonLoadingFailed(String),
    ClearDialog,
    CmcApiKeySubmit,
    CmcApiKeyInput(String),
    CmcApiKeyClearInput,
    ToggleOnEditApiKey,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuyEntry {
    // #[serde(rename = "isStableCoin", default)]
    // is_stable_coin: bool,
    // date: String,
    // #[serde(rename = "amountUsd")]
    // amount_usd: f64,
    // amount: f64,
}

type RawJsonData = HashMap<String, Vec<BuyEntry>>;

fn parse_paid_entries(raw_json: &str) -> Result<RawJsonData, serde_json::Error> {
    let json_data: RawJsonData = serde_json::from_str(raw_json)?;

    Ok(json_data)
}

struct DialogContent {
    title: String,
    body: String,
}

enum PaidEntriesDialogContent {
    Error(DialogContent),
    Success(DialogContent),
}

#[derive(Default)]
enum PaidEntriesJsonLoadState {
    #[default]
    NotLoaded,
    Loaded,
    Errored,
}

#[derive(Default)]
pub struct PaidEntriesPage {
    config_handler: Option<cosmic_config::Config>,
    config: config::UniConfig,
    dialog: Option<PaidEntriesDialogContent>,
    paid_entries_json_load_state: PaidEntriesJsonLoadState,
    raw_json_data: Option<RawJsonData>,
    is_edit_api_key_unlocked: bool,
    editing_cmc_api_key: String,
}

impl PaidEntriesPage {
    #[must_use]
    pub fn new(config: config::UniConfig, config_handler: Option<cosmic_config::Config>) -> Self {
        Self {
            config_handler,
            config,
            ..Default::default()
        }
    }

    fn load_paid_entries_json(json_path: PathBuf) -> Task<PaidEntriesPageMessage> {
        Task::future(async move {
            match tokio::fs::read_to_string(json_path).await {
                Ok(raw_json) => match parse_paid_entries(&raw_json) {
                    Ok(raw_json_data) => PaidEntriesPageMessage::RawJsonLoaded(raw_json_data),
                    Err(e) => PaidEntriesPageMessage::RawJsonLoadingFailed(e.to_string()),
                },
                Err(e) => {
                    tracing::error!("load_paid_entries_json failed: {e}");
                    PaidEntriesPageMessage::RawJsonLoadingFailed(e.to_string())
                }
            }
        })
    }
}

impl pages::IPage<PaidEntriesPageMessage> for PaidEntriesPage {
    fn view(&self) -> Element<PaidEntriesPageMessage> {
        let active_theme = theme::active();
        let cosmic_theme = active_theme.cosmic();

        widget::container(
            widget::column()
                .push(widget::text(fl!("cmc-api-key")))
                .push(widget::Space::with_height(cosmic_theme.space_xxs()))
                .push(
                    widget::row()
                        .align_y(Vertical::Center)
                        .push(
                            widget::text_input(
                                fl!("cmc-api-key"),
                                if self.is_edit_api_key_unlocked {
                                    &self.editing_cmc_api_key
                                } else {
                                    self.config
                                        .coin_market_cap_api_key
                                        .as_ref()
                                        .map_or("", |cmc_api_key| cmc_api_key)
                                },
                            )
                            .password()
                            .apply(|widget| {
                                if self.is_edit_api_key_unlocked {
                                    widget
                                        .on_clear(PaidEntriesPageMessage::CmcApiKeyClearInput)
                                        .on_input(PaidEntriesPageMessage::CmcApiKeyInput)
                                } else {
                                    widget
                                }
                            }),
                        )
                        .push(widget::Space::with_width(cosmic_theme.space_xxs()))
                        .push(
                            widget::button::icon(if self.is_edit_api_key_unlocked {
                                icon::from_name("checkbox-checked-symbolic")
                            } else {
                                icon::from_name("edit-symbolic")
                            })
                            .apply(|widget| {
                                if self.is_edit_api_key_unlocked {
                                    if self.editing_cmc_api_key.is_empty() {
                                        widget
                                    } else {
                                        widget.on_press(PaidEntriesPageMessage::CmcApiKeySubmit)
                                    }
                                } else {
                                    widget.on_press(PaidEntriesPageMessage::ToggleOnEditApiKey)
                                }
                            }),
                        ),
                )
                .push(
                    widget::text_input(
                        fl!("json-path"),
                        if let Some(json_path) = &self.config.paid_entries_json_path {
                            json_path.to_string_lossy()
                        } else {
                            Cow::Owned(String::new())
                        },
                    )
                    .label(fl!("json-path"))
                    .trailing_icon(
                        widget::button::icon(icon::from_name("edit-symbolic"))
                            .on_press(PaidEntriesPageMessage::ShowJsonPicker)
                            .into(),
                    ),
                )
                .push_maybe(self.raw_json_data.as_ref().map(|raw_json_data| {
                    widget::text(format!(
                        "You got {}",
                        raw_json_data
                            .keys()
                            .map(|it| &**it)
                            .collect::<Vec<&str>>()
                            .join(", ")
                    ))
                })),
        )
        .into()
    }

    fn update(&mut self, message: PaidEntriesPageMessage) -> cosmic::Task<PaidEntriesPageMessage> {
        match message {
            PaidEntriesPageMessage::ShowJsonPicker => {
                if let Some(json_path) = rfd::FileDialog::new()
                    .set_title(fl!("pick-json-dialog-title"))
                    .add_filter("json", &["json"])
                    .set_directory(
                        self.config
                            .paid_entries_json_path
                            .clone()
                            .map_or(PathBuf::new(), |path| {
                                path.parent().unwrap_or(&path).to_owned()
                            }),
                    )
                    .pick_file()
                {
                    if let Some(config_handler) = self.config_handler.as_ref() {
                        if let Err(e) = self
                            .config
                            .set_paid_entries_json_path(config_handler, Some(json_path.clone()))
                        {
                            tracing::error!("Error set_paid_entries_json_path: {e}");
                        }
                    }

                    return Self::load_paid_entries_json(json_path).map(|it| match it {
                        PaidEntriesPageMessage::RawJsonLoaded(raw_json_data) => {
                            PaidEntriesPageMessage::RawJsonUpdated(raw_json_data)
                        }
                        _ => it,
                    });
                }
            }
            PaidEntriesPageMessage::RawJsonUpdated(raw_json_data) => {
                self.dialog = Some(PaidEntriesDialogContent::Success(DialogContent {
                    title: "JSON loaded".to_owned(),
                    body: format!("Loaded: {} cryptos", raw_json_data.len()),
                }));

                return Task::done(PaidEntriesPageMessage::RawJsonLoaded(raw_json_data));
            }
            PaidEntriesPageMessage::RawJsonLoaded(raw_json_data) => {
                self.paid_entries_json_load_state = PaidEntriesJsonLoadState::Loaded;
                self.raw_json_data = Some(raw_json_data);
            }
            PaidEntriesPageMessage::RawJsonLoadingFailed(reason) => {
                self.dialog = Some(PaidEntriesDialogContent::Error(DialogContent {
                    title: fl!("error-loading-json-file"),
                    body: reason,
                }));
                self.paid_entries_json_load_state = PaidEntriesJsonLoadState::Errored;
            }
            PaidEntriesPageMessage::ClearDialog => {
                self.dialog = None;
            }
            PaidEntriesPageMessage::CmcApiKeySubmit => {
                assert!(
                    !self.editing_cmc_api_key.is_empty(),
                    "editing_cmc_api_key must NOT be empty"
                );

                if let Some(config_handler) = self.config_handler.as_ref() {
                    if let Err(e) = self.config.set_coin_market_cap_api_key(
                        config_handler,
                        Some(self.editing_cmc_api_key.clone()),
                    ) {
                        tracing::error!("Error set_coin_market_cap_api_key: {e}");
                    } else {
                        self.is_edit_api_key_unlocked = false;
                    }
                }
            }
            PaidEntriesPageMessage::ToggleOnEditApiKey => {
                self.is_edit_api_key_unlocked = true;
            }
            PaidEntriesPageMessage::CmcApiKeyInput(cmc_api_key) => {
                self.editing_cmc_api_key = cmc_api_key;
            }
            PaidEntriesPageMessage::CmcApiKeyClearInput => {
                self.editing_cmc_api_key = String::new();
                self.is_edit_api_key_unlocked = false;
            }
        }

        Task::none()
    }

    fn on_init(&self) -> cosmic::Task<PaidEntriesPageMessage> {
        match self.paid_entries_json_load_state {
            PaidEntriesJsonLoadState::Errored | PaidEntriesJsonLoadState::Loaded => Task::none(),
            PaidEntriesJsonLoadState::NotLoaded => {
                if let Some(json_path) = self.config.paid_entries_json_path.as_ref() {
                    Self::load_paid_entries_json(json_path.clone())
                } else {
                    cosmic::Task::none()
                }
            }
        }
    }

    fn dialog(&self) -> Option<cosmic::Element<PaidEntriesPageMessage>> {
        match &self.dialog {
            Some(PaidEntriesDialogContent::Error(message)) => Some(
                widget::dialog()
                    .title(&message.title)
                    .body(&message.body)
                    .icon(icon::from_name("process-stop"))
                    .primary_action(
                        widget::button::suggested(fl!("btn-confirm"))
                            .on_press(PaidEntriesPageMessage::ClearDialog),
                    )
                    .into(),
            ),
            Some(PaidEntriesDialogContent::Success(message)) => Some(
                widget::dialog()
                    .title(&message.title)
                    .body(&message.body)
                    .icon(icon::from_name("process-stop"))
                    .primary_action(
                        widget::button::standard(fl!("btn-confirm"))
                            .on_press(PaidEntriesPageMessage::ClearDialog),
                    )
                    .into(),
            ),
            None => None,
        }
    }
}

impl From<PaidEntriesPageMessage> for app::UniAppMessage {
    fn from(message: PaidEntriesPageMessage) -> Self {
        pages::Message::PaidEntries(message).into()
    }
}
