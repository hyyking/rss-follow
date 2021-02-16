use rss::{Channel, Item};
use std::fmt;

#[derive(Debug)]
pub struct Options {
    pub num: Option<usize>,
}

pub trait RssFormatter {
    fn rss_fmt(&self, channel: &Channel, options: &Options) -> String;
}

#[derive(Debug)]
pub struct DefaultFormatter;
impl RssFormatter for DefaultFormatter {
    fn rss_fmt(&self, channel: &Channel, options: &Options) -> String {
        struct DebugChannel<'a> {
            channel: &'a Channel,
            options: &'a Options,
        }
        struct DebugItem<'a> {
            item: &'a Item,
        }
        impl<'a> fmt::Debug for DebugChannel<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_map()
                    .entry(&"name", &self.channel.title())
                    .entry(&"link", &self.channel.link())
                    .entry(
                        &"items",
                        &self
                            .channel
                            .items()
                            .iter()
                            .take(self.options.num.unwrap_or(usize::MAX))
                            .map(|item| DebugItem { item })
                            .collect::<Vec<_>>(),
                    )
                    .finish()
            }
        }
        impl<'a> fmt::Debug for DebugItem<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_map()
                    .entry(&"name", &(self.item.title().unwrap_or("")))
                    .entry(&"date", &(self.item.pub_date().unwrap_or("")))
                    .entry(&"link", &(self.item.link().unwrap_or("")))
                    .finish()
            }
        }
        format!("{:#?}", DebugChannel { channel, options })
    }
}

#[derive(Debug)]
pub struct SmallFormatter;
impl RssFormatter for SmallFormatter {
    fn rss_fmt(&self, channel: &Channel, options: &Options) -> String {
        channel
            .items()
            .iter()
            .take(options.num.unwrap_or(usize::MAX))
            .map(|item| {
                format!(
                    "\n{} - {} > {}",
                    item.title().unwrap_or(""),
                    item.pub_date().unwrap_or(""),
                    item.link().unwrap_or("")
                )
            })
            .collect::<String>()
    }
}
