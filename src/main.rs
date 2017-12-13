#[macro_use]
extern crate clap;
extern crate url;

use std::process::Command;
use url::Url;
use clap::{App, Arg};

fn main() {
    let app = App::new(crate_name!())
        .about(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(Arg::with_name("QUERY")
             .help("Query to search for")
             .required(true)
             .index(1)
             .multiple(true));
    let matches = app.get_matches();
    let query_words: Vec<String> = matches.values_of("QUERY")
                    .expect("Query not provided by user")
                    .map(|s| s.to_string()).collect();
    let query = query_words.join(" ");
    let target_url = Url::parse_with_params("https://duckduckgo.com/lite/", &[("q", query)])
                    .expect("URL parsing library is broken");
    // TODO: custom browser support
    let browser = Command::new("elinks").arg(target_url.to_string()).status();
}
