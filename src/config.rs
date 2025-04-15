// SPDX-License-Identifier: MPL-2.0

use cosmic::{
    cosmic_config::{
        self, cosmic_config_derive::CosmicConfigEntry, Config as CosmicConfig, CosmicConfigEntry,
    },
    Application,
};

use crate::app::{AppModel, Flags};

#[derive(Debug, Default, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct UniConfig {
    pub username: String,
}

pub const CONFIG_VERSION: u64 = 1;

impl UniConfig {
    #[allow(clippy::needless_pass_by_value)]
    pub fn map_config_result(context: cosmic_config::Config) -> Self {
        match UniConfig::get_entry(&context) {
            Ok(config) => {
                tracing::info!("{:?}", context);
                config
            }
            Err((errors, config)) => {
                for why in errors {
                    tracing::error!(%why, "error loading app config");
                }

                config
            }
        }
    }

    fn config_handler() -> Option<CosmicConfig> {
        CosmicConfig::new(AppModel::APP_ID, CONFIG_VERSION).ok()
    }

    fn config() -> UniConfig {
        match Self::config_handler() {
            Some(config_handler) => {
                UniConfig::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                    tracing::error!("errors loading config: {:?}", errs);
                    config
                })
            }
            None => UniConfig::default(),
        }
    }

    pub fn flags() -> Flags {
        let (config_handler, config) = (Self::config_handler(), Self::config());

        Flags {
            config_handler,
            config,
        }
    }
}
