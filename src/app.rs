// SPDX-License-Identifier: MPL-2.0

use crate::config::{self, UniConfig};
use crate::view::lib::nav_bar::init_nav_bar;
use crate::{
    fl,
    pages::{self, IPage, Page},
};
use cosmic::app::{context_drawer, Core, Task};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Alignment, Subscription};
use cosmic::widget::{self, menu, nav_bar};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Element};
use std::collections::HashMap;

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    config_handler: Option<cosmic_config::Config>,
    // Configuration data that persists between application runs.
    config: UniConfig,
    //#region Application specific fields
    // ? Pages
    about_pc_page: pages::about_pc::AboutPcPage,
    clock_page: pages::clock::ClockPage,
    preferences_page: pages::preferences::PreferencesPage,
    paid_entries_page: pages::paid_entries::PaidEntriesPage,
    //#endregion
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum UniAppMessage {
    OpenRepositoryUrl,
    ToggleContextPage(ContextPage),
    UpdateConfig(UniConfig),
    LaunchUrl(String),
    Page(pages::Message),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Flags {
    pub config_handler: Option<cosmic_config::Config>,
    pub config: config::UniConfig,
}

/// Create a COSMIC application from the app model
impl Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = Flags;

    /// Messages which the application and its widgets will emit.
    type Message = UniAppMessage;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.github.pop-os.cosmic-app-template";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(core: Core, flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app_config = cosmic_config::Config::new(Self::APP_ID, UniConfig::VERSION)
            .map(UniConfig::map_config_result)
            .unwrap_or_default();

        let mut app = AppModel {
            core,
            context_page: ContextPage::default(),
            nav: init_nav_bar(&app_config.last_active_page),
            key_binds: HashMap::new(),
            // Optional configuration file for an application.
            config_handler: flags.config_handler.clone(),
            config: app_config.clone(),
            about_pc_page: pages::about_pc::AboutPcPage::default(),
            clock_page: pages::clock::ClockPage::default(),
            preferences_page: pages::preferences::PreferencesPage::new(
                app_config.clone(),
                flags.config_handler.clone(),
            ),
            paid_entries_page: pages::paid_entries::PaidEntriesPage::new(
                app_config,
                flags.config_handler,
            ),
        };

        // Create a startup command that sets the window title.
        let command = Task::batch([app.update_title(), app.on_page_init()]);

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![
                    menu::Item::Button(fl!("about"), None, MenuAction::About),
                    menu::Item::Button(fl!("settings"), None, MenuAction::Settings),
                ],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => context_drawer::context_drawer(
                self.about(),
                UniAppMessage::ToggleContextPage(ContextPage::About),
            )
            .title(fl!("about")),
            ContextPage::Settings => context_drawer::context_drawer(
                self.settings(),
                UniAppMessage::ToggleContextPage(ContextPage::Settings),
            )
            .title(fl!("settings")),
        })
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// Application events will be processed through the view. Any messages emitted by
    /// events received by widgets will be passed to the update method.
    fn view(&self) -> Element<Self::Message> {
        match self.nav.active_data::<Page>() {
            Some(page) => match page {
                Page::AboutPc => self.about_pc_page.view().map(Into::into),
                Page::Clock => self.clock_page.view().map(Into::into),
                Page::Preferences => self.preferences_page.view().map(Into::into),
                Page::PaidEntries => self.paid_entries_page.view().map(Into::into),
            },
            None => self.about_pc_page.view().map(Into::into),
        }
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            self.about_pc_page.subscription().map(Into::into),
            self.clock_page.subscription().map(Into::into),
            self.preferences_page.subscription().map(Into::into),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<UniConfig>(Self::APP_ID)
                .map(|update| {
                    for why in update.errors {
                        tracing::error!(?why, "app config error");
                    }

                    UniAppMessage::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            UniAppMessage::OpenRepositoryUrl => {
                _ = open::that_detached(REPOSITORY);
            }
            UniAppMessage::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }
            }
            UniAppMessage::UpdateConfig(config) => {
                self.config = config.clone();
                _ = self.preferences_page.update(
                    pages::preferences::PreferencesPageMessage::ConfigUpdated(config),
                );
            }
            UniAppMessage::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("failed to open {url:?}: {err}");
                }
            },
            UniAppMessage::Page(message) => match message {
                pages::Message::AboutPc(about_pc_page_message) => {
                    _ = self.about_pc_page.update(about_pc_page_message);
                }
                pages::Message::Clock(clock_page_message) => {
                    _ = self.clock_page.update(clock_page_message);
                }
                pages::Message::Preferences(config_page_message) => {
                    _ = self.preferences_page.update(config_page_message);
                }
                pages::Message::PaidEntries(paid_entries_page_message) => {
                    return self
                        .paid_entries_page
                        .update(paid_entries_page_message)
                        .map(|task| UniAppMessage::Page(pages::Message::PaidEntries(task)).into())
                }
            },
        }
        Task::none()
    }

    fn dialog(&self) -> Option<Element<Self::Message>> {
        match self.nav.active_data::<Page>() {
            Some(Page::PaidEntries) => self.paid_entries_page.dialog().map(|it| it.map(Into::into)),
            _ => None,
        }
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        self.nav.activate(id);

        if let (Some(page), Some(config_handler)) =
            (self.nav.active_data::<Page>(), self.config_handler.as_mut())
        {
            if let Err(e) = self
                .config
                .set_last_active_page(config_handler, page.clone())
            {
                tracing::error!("Error setting active page {e}");
            }
        }

        Task::batch([self.on_page_init(), self.update_title()])
    }
}

#[allow(clippy::unused_self)]
impl AppModel {
    pub fn about(&self) -> Element<UniAppMessage> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(APP_ICON));

        let title = widget::text::title3(fl!("app-title"));

        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");

        let link = widget::button::link(REPOSITORY)
            .on_press(UniAppMessage::OpenRepositoryUrl)
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .push(
                widget::button::link(fl!(
                    "git-description",
                    hash = short_hash.as_str(),
                    date = date
                ))
                .on_press(UniAppMessage::LaunchUrl(format!(
                    "{REPOSITORY}/commits/{hash}"
                )))
                .padding(0),
            )
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    pub fn settings(&self) -> Element<UniAppMessage> {
        widget::column().push(widget::text(fl!("settings"))).into()
    }

    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<UniAppMessage> {
        let mut window_title = fl!("app-title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }

    pub fn on_page_init(&self) -> Task<UniAppMessage> {
        match self.nav.active_data::<Page>() {
            Some(Page::PaidEntries) => self
                .paid_entries_page
                .on_init()
                .map(|it| pages::Message::PaidEntries(it).into()),
            _ => Task::none(),
        }
    }
}

impl From<pages::Message> for cosmic::action::Action<UniAppMessage> {
    fn from(page_message: pages::Message) -> Self {
        Self::App(UniAppMessage::Page(page_message))
    }
}

/// The context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
    Settings,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
    Settings,
}

impl menu::action::MenuAction for MenuAction {
    type Message = UniAppMessage;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => UniAppMessage::ToggleContextPage(ContextPage::About),
            MenuAction::Settings => UniAppMessage::ToggleContextPage(ContextPage::Settings),
        }
    }
}
