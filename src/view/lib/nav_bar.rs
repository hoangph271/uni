use crate::fl;
use crate::pages::Page;
use cosmic::widget::{icon, nav_bar};

pub fn init_nav_bar(active_page: &Page) -> nav_bar::Model {
    let mut nav = nav_bar::Model::default();

    nav.insert()
        .text(fl!("page-about-pc"))
        .data::<Page>(Page::AboutPc)
        .icon(icon::from_name("applications-science-symbolic"))
        .activate();

    nav.insert()
        .text(fl!("page-clock"))
        .data::<Page>(Page::Clock)
        .icon(icon::from_name("applications-office-symbolic"));

    nav.insert()
        .text(fl!("page-config"))
        .data::<Page>(Page::Preferences)
        .icon(icon::from_name("applications-games-symbolic"));

    nav.insert()
        .text(fl!("page-paid-entries"))
        .data::<Page>(Page::PaidEntries)
        .icon(icon::from_name("zoom-original-symbolic"));

    let id = {
        nav.iter()
            .find(|item_id| {
                if let Some(item_id) = nav.data::<Page>(*item_id) {
                    *item_id == *active_page
                } else {
                    false
                }
            })
            .unwrap_or_default()
    };

    nav.activate(id);

    nav
}
