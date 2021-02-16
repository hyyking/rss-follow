extern crate inventory;
extern crate reqwest;
extern crate rss;
extern crate tokio;

mod fmt;
mod subscribe;

use crate::{
    fmt::{Options, SmallFormatter},
    subscribe::Subscribe,
};

use rss::Channel;
use tokio::runtime::Runtime;

macro_rules! submit_all {
    ($($sub:expr),+) => { $( inventory::submit! { $sub } )+ }
}

inventory::collect!(Subscribe);
submit_all! {
    Subscribe::new("TWIR", "https://this-week-in-rust.org/rss.xml"),
    Subscribe::with_formatter("twir", "https://this-week-in-rust.org/rss.xml", SmallFormatter),
    Subscribe::new("Rust-os", "https://os.phil-opp.com/rss.xml"),
    Subscribe::new("DT", "https://www.distrotube.com/videos/index.xml"),
    Subscribe::new("GFR", "https://www.theguardian.com/world/france/rss")
}

type Error = Box<dyn std::error::Error>;

async fn run(feeds: Option<Vec<&str>>, options: Options) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let subs = if let Some(feeds) = feeds {
        inventory::iter::<Subscribe>()
            .filter(|el| feeds.contains(&el.name()))
            .collect::<Vec<_>>()
    } else {
        inventory::iter::<Subscribe>().collect::<Vec<_>>()
    };

    for subscription in subs {
        let res = client.get(subscription.url()).send().await?.text().await?;
        let feed = res.parse::<Channel>()?;
        println!("{}", subscription.formatter().rss_fmt(&feed, &options))
    }
    Ok(())
}

fn args() -> Vec<clap::Arg<'static, 'static>> {
    use clap::Arg;
    vec![
        Arg::with_name("num")
            .short("e")
            .long("entries")
            .takes_value(true)
            .help("number of entries to print per rss feed"),
        Arg::with_name("feeds")
            .long("feeds")
            .takes_value(true)
            .value_delimiter(",")
            .help("filter the feeds"),
    ]
}

fn main() -> Result<(), Error> {
    let mut rt = Runtime::new()?;
    let m = clap::App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .about("rss feeds formatter")
        .args(&args())
        .get_matches();
    let options = Options {
        num: m.value_of("num").map(|num| num.parse()).transpose()?,
    };
    let filters = m.values_of("feeds").map(|feeds| feeds.collect::<Vec<_>>());
    rt.block_on(run(filters, options))
}
