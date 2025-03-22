use crate::{app, pages};

#[derive(Default)]
pub struct ConfigPage {}

#[derive(Debug, Clone)]
pub struct ConfigPageMessage {}

impl pages::IPage<ConfigPageMessage> for ConfigPage {
    fn view(&self) -> cosmic::Element<ConfigPageMessage> {
        todo!()
    }

    fn subscription(&self) -> cosmic::iced::Subscription<ConfigPageMessage> {
        todo!()
    }

    fn update(&mut self, _message: ConfigPageMessage) -> cosmic::Task<ConfigPageMessage> {
        todo!()
    }
}

impl From<ConfigPageMessage> for app::Message {
    fn from(message: ConfigPageMessage) -> Self {
        pages::Message::Config(message).into()
    }
}
