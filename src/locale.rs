use chrono::{format, Locale};
use std::str::FromStr;

pub fn get_locale() -> Locale {
    let locale = std::env::var("LC_TIME").or_else(|_| std::env::var("LANG"));

    if let Some(locale) = locale
        .ok()
        .and_then(|locale| locale.split('.').next().map(ToString::to_string))
    {
        format::Locale::from_str(&locale).unwrap_or(format::Locale::POSIX)
    } else {
        format::Locale::POSIX
    }
}
