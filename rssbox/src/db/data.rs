use crate::slint_generatedAppWindow::{RssConfig as UIRssConfig, RssEntry as UIRssEntry, RssList};
use std::fmt::Debug;

fn feed_format_default() -> String {
    "RSS".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RssConfig {
    pub name: String,
    pub url: String,
    pub icon_index: i32,
    pub use_proxy: bool,
    pub is_mark: bool,
    pub update_time: String,

    #[serde(default = "feed_format_default")]
    pub feed_format: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct RssEntry {
    pub uuid: String,
    pub url: String,
    pub title: String,
    pub pub_date: String,
    pub tags: String,
    pub author: String,
    pub summary: String,
    pub is_read: bool,
}

impl From<&RssList> for RssConfig {
    fn from(conf: &RssList) -> Self {
        RssConfig {
            name: conf.name.clone().into(),
            url: conf.url.clone().into(),
            icon_index: conf.icon_index,
            use_proxy: conf.use_proxy,
            is_mark: conf.is_mark,
            update_time: conf.update_time.clone().into(),
            feed_format: conf.feed_format.clone().into(),
        }
    }
}

impl From<&UIRssConfig> for RssConfig {
    fn from(conf: &UIRssConfig) -> Self {
        RssConfig {
            name: conf.name.clone().into(),
            url: conf.url.clone().into(),
            icon_index: conf.icon_index,
            use_proxy: conf.use_proxy,
            feed_format: conf.feed_format.clone().into(),
            is_mark: false,
            update_time: "".into(),
        }
    }
}

impl From<&UIRssEntry> for RssEntry {
    fn from(entry: &UIRssEntry) -> Self {
        RssEntry {
            uuid: entry.uuid.clone().into(),
            url: entry.url.clone().into(),
            title: entry.title.clone().into(),
            tags: entry.tags.clone().into(),
            pub_date: entry.pub_date.clone().into(),
            author: entry.author.clone().into(),
            summary: entry.summary.clone().into(),
            is_read: entry.is_read,
        }
    }
}

impl From<RssEntry> for UIRssEntry {
    fn from(entry: RssEntry) -> Self {
        UIRssEntry {
            suuid: "".into(),
            uuid: entry.uuid.into(),
            url: entry.url.into(),
            pub_date: entry.pub_date.into(),
            title: entry.title.into(),
            tags: entry.tags.into(),
            author: entry.author.into(),
            summary: entry.summary.into(),
            is_read: entry.is_read,
        }
    }
}
