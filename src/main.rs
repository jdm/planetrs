#![feature(dedup_by)]
#![feature(proc_macro)]
#[macro_use] extern crate serde_derive;
extern crate curl;
extern crate chrono;
extern crate atom_syndication;
extern crate rss;
extern crate serde_yaml;
extern crate serde_json;
#[macro_use] extern crate tera;
extern crate uuid;

mod entry;
mod reader;
mod catcher;
mod renderer;
mod storer;

fn main() {
    let feeds = reader::read_feeds("feeds.yml");
    let mut entries = catcher::get_entries(&feeds);

    storer::merge_entries(&mut entries, "back.txt");

    let mut data = renderer::Data::new();
    data.feeds = feeds;
    entries.truncate(12);
    data.entries = entries;
    renderer::render(&data, "templates/main.html", "./output/index.html");
}
