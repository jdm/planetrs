use std::string::String;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

use curl::easy::Easy;
use chrono;
use rss;
use atom_syndication;

use entry::FeedInfo;
use entry::Entry;

pub fn get_entries(feeds: &Vec<FeedInfo>) -> Vec<Entry> {
    let inner_fi = Arc::new(Mutex::new(feeds.clone()));
    let mut th_entries = Arc::new(Mutex::new(Vec::<Entry>::new()));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let inner_fi = inner_fi.clone();
        let th_entries = th_entries.clone();
        handles.push(
            thread::spawn(move || {
                while let Some(fi) = inner_fi.lock().expect("Cant lock inner_fi").pop() {
                    let mut dst = Vec::new();
                    let mut handle = Easy::new();
                    handle.timeout(Duration::new(10, 0)).expect("Cant set timeout");
                    handle.url(&fi.feedurl).expect("Cant set url");
                    handle.get(true).expect("Cant set Get");
                    {
                        let mut transfer = handle.transfer();
                        transfer.write_function(|data| {
                            dst.extend_from_slice(data);
                            Ok(data.len())
                        }).expect("Cant set write_fn");
                        transfer.perform().expect("Get perform failed");
                    }

                    let buf = String::from_utf8(dst).expect("Cant convert dst to buf");
                    if let Ok(f) = buf.parse::<rss::Channel>() {
                        rss_to_entries(f, &fi, &th_entries)
                    }
                    if let Ok(f) = buf.parse::<atom_syndication::Feed>() {
                        atom_to_entries(f, &fi, &th_entries)
                    }
                }
            })
        )
    }

    for h in handles {
        h.join().expect("join handle failed");
    }

    let v = Arc::get_mut(&mut th_entries).expect("getmut arc failed").get_mut().expect("getmut mutex failed").clone();
    v
}

fn rss_to_entries(f: rss::Channel, info: &FeedInfo, v: &Arc<Mutex<Vec<Entry>>>) {
    for item in f.items.iter() {
        let mut entry = Entry::new();
        entry.info = (*info).clone();
        entry.title = item.clone().title.expect("rss title failed");
        entry.link = item.clone().link.expect("rss link failed");
        let temp_resume = item.clone().description.expect("rss content failed");
        entry.resume = select_first_paragraph(temp_resume);
        entry.date = chrono::DateTime::parse_from_rfc2822(item.clone().pub_date.expect("rss date failed").as_ref()).expect("parse date failed").with_timezone(&chrono::UTC);
        entry.generate_human_date();
        entry.generate_uid();
        v.lock().expect("v lock failed").push(entry);
    }
}

fn atom_to_entries(f: atom_syndication::Feed, info: &FeedInfo, v: &Arc<Mutex<Vec<Entry>>>) {
    for item in f.entries.iter() {
        let mut entry = Entry::new();
        entry.info = (*info).clone();
        entry.title = item.clone().title;
        entry.link = item.clone().links[0].clone().href;
        if let atom_syndication::Content::Text(txt) = item.clone().content.expect("no atom content") {
            entry.resume = select_first_paragraph(txt)
        }
        if let atom_syndication::Content::Html(txt) = item.clone().content.expect("no atom content") {
            entry.resume = select_first_paragraph(txt)
        }
        let temp = item.clone().updated;
        entry.date = chrono::DateTime::parse_from_rfc3339(temp.as_ref()).expect("rss date failed").with_timezone(&chrono::UTC);
        entry.generate_human_date();
        entry.generate_uid();
        v.lock().expect("v lock failed").push(entry);
    }
}

fn select_first_paragraph(txt: String) -> String {
    let temp_str = txt.split("<p>").nth(1).expect("Bad <p> split");
    let temp_str = temp_str.split("</p>").nth(0).expect("Bad </p> split");
    temp_str.to_string()
}
