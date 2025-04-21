use crate::{app, config, fl, pages};
use cosmic::{
    cosmic_config,
    widget::{container, text_input},
    Task,
};

#[derive(Default)]
pub struct PreferencesPage {
    config_handler: Option<cosmic_config::Config>,
    config: config::UniConfig,
}

impl PreferencesPage {
    #[must_use]
    pub fn new(config: config::UniConfig, config_handler: Option<cosmic_config::Config>) -> Self {
        Self {
            config_handler,
            config,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PreferencesPageMessage {
    ConfigUpdated(config::UniConfig),
    Username(String),
}

impl pages::IPage<PreferencesPageMessage> for PreferencesPage {
    fn view(&self) -> cosmic::Element<PreferencesPageMessage> {
        container(
            text_input(fl!("username"), &self.config.username)
                .label(fl!("username"))
                .on_input(PreferencesPageMessage::Username),
        )
        .into()
    }

    fn update(&mut self, message: PreferencesPageMessage) -> cosmic::Task<PreferencesPageMessage> {
        match message {
            PreferencesPageMessage::Username(username) => {
                if let Some(config_handler) = &self.config_handler {
                    if let Err(err) = self.config.set_username(config_handler, username) {
                        tracing::error!("Error setting username: {err}");
                    }
                }
            }
            PreferencesPageMessage::ConfigUpdated(config) => self.config = config,
        }

        Task::none()
    }
}

impl From<PreferencesPageMessage> for app::UniAppMessage {
    fn from(message: PreferencesPageMessage) -> Self {
        pages::Message::Preferences(message).into()
    }
}
