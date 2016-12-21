use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use serde_json;

use entry::Entry;

pub fn merge_entries<P: AsRef<Path>>(entries: &mut Vec<Entry>, filepath: P) {
    if !filepath.as_ref().exists() {
        let mut f = File::create(&filepath).expect("Storer cant create file");
        let serialized = serde_json::to_string(entries).expect("Storer cant json entries (create)");
        f.write_all(&serialized.into_bytes()).expect("Cant write entries (create)");
        entries.sort_by(|a, b| a.date.cmp(&b.date).reverse());
    }
    else {
        let f = File::open(&filepath).expect("Storer cant open file");
        let serialized: Vec<Entry> = serde_json::from_reader(&f).expect("cant read store file");
        for entry_ser in serialized {
            entries.push(entry_ser);
        }
        entries.sort_by(|a, b| a.uid.cmp(&b.uid));
        entries.dedup_by(|e1, e2| e1.uid == e2.uid);

        drop(f);
        let mut f = File::create(&filepath).expect("Storer cant create file");
        let serialized = serde_json::to_string(entries).expect("Storer cant json entries (merge)");
        f.write_all(&serialized.into_bytes()).expect("Cant write entries (merge)");
        entries.sort_by(|a, b| a.date.cmp(&b.date).reverse());
    }
}
