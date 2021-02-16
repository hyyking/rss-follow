use crate::fmt::{DefaultFormatter, RssFormatter};

use reqwest::Url;

pub struct Subscribe {
    name: &'static str,
    url: reqwest::Url,
    formatter: Box<dyn RssFormatter>,
}

impl Subscribe {
    pub fn new(name: &'static str, url: &str) -> Self {
        Self {
            url: Url::parse(url).unwrap(),
            formatter: Box::new(DefaultFormatter),
            name,
        }
    }
    pub fn with_formatter<T>(name: &'static str, url: &str, fmt: T) -> Self
    where
        T: RssFormatter + 'static,
    {
        Self {
            url: Url::parse(url).unwrap(),
            formatter: Box::new(fmt),
            name,
        }
    }
    pub fn url(&self) -> Url {
        self.url.clone()
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn formatter(&self) -> &dyn RssFormatter {
        &*self.formatter
    }
}
