// SPDX-License-Identifier: MPL-2.0

use config::UniConfig;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod app;
mod config;
mod i18n;
mod locale;
mod pages;

fn main() -> cosmic::iced::Result {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting global_default tracing::subscriber failed");

    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Settings for configuring the application window and iced runtime.
    let settings = cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(360.0)
            .min_height(180.0),
    );

    // Starts the application's event loop with `()` as the application's flags.
    cosmic::app::run::<app::AppModel>(settings, UniConfig::flags())
}
