use std::fs::File;
use std::path::Path;

use entry::FeedInfo;
use serde_yaml;

pub fn read_feeds<P: AsRef<Path>>(path: P) -> Vec<FeedInfo> {
    let filestream = File::open(path).expect("Couldn't read feeds file");
    let feeds_seq: serde_yaml::Sequence = serde_yaml::from_reader(filestream).expect("Couldn't parse feeds list");

    let mut feeds_list: Vec<FeedInfo> = Vec::new();
    for feed in feeds_seq {
        feeds_list.push(parse_feed(feed));
    }
    feeds_list
}

fn parse_feed(yml_feed: serde_yaml::Value) -> FeedInfo {
    let mut fi = FeedInfo::new();
    let feed_map = yml_feed.as_mapping().expect("Couldnt parse feed as map");

    for a in feed_map.keys() {
        fi.id = a.as_str().expect("Cant get id of feed").to_string();
        let fi_values = feed_map.get(a).expect("Cant get values of feed").as_mapping().expect("values as map failed");

        let name_key = &serde_yaml::Value::String("name".to_owned());
        let url_key = &serde_yaml::Value::String("feedurl".to_owned());
        let home_key = &serde_yaml::Value::String("homepage".to_owned());

        fi.name = fi_values.get(name_key).expect("value name").as_str().expect("name str").to_string();
        fi.feedurl = fi_values.get(url_key).expect("value feedurl").as_str().expect("url str").to_string();
        fi.homepage = fi_values.get(home_key).expect("value homepage").as_str().expect("home str").to_string();
    }

    fi
}
