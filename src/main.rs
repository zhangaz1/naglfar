extern crate naglfar;
use naglfar::renderer;

extern crate clap;
use clap::{App, Arg};

use std::fs::OpenOptions;
use std::io::prelude::*;

const VERSION_STR: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let app = App::new("naglfar")
        .version(VERSION_STR)
        .author("uint256_t")
        .about("naglfar is a web browser implementation in Rust")
        .arg(Arg::with_name("FILE").help("Input file").index(1));
    let _app_matches = app.get_matches();

    let mut html_source_file = OpenOptions::new()
        .read(true)
        .open("./example/test.html")
        .unwrap();
    let mut html_source = "".to_string();
    html_source_file
        .read_to_string(&mut html_source)
        .ok()
        .expect("cannot read file");
    renderer::show_html(html_source.as_str());
}
