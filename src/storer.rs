use std::path::Path;
use std::fs::File;
use std::io::Write;
use zip;

use serde_json;

use entry::Entry;

pub fn merge_entries<P: AsRef<Path>>(entries: &mut Vec<Entry>, filepath: P) {
    if !filepath.as_ref().exists() {
        let f = File::create(&filepath).expect("Storer cant create file");
        let serialized = serde_json::to_string(entries).expect("Storer cant json entries (create)");

        let mut zipw = zip::ZipWriter::new(&f);
        zipw.start_file("entries.json", zip::write::FileOptions::default()).expect("Cant start file (create)");
        zipw.write(&serialized.into_bytes()).expect("Cant write entries (create)");
        zipw.finish().expect("Cant finish zip (create)");

        entries.sort_by(|a, b| a.date.cmp(&b.date).reverse());
    }
    else {
        let f = File::open(&filepath).expect("Storer cant open file");
        let mut zipr = zip::ZipArchive::new(&f).expect("Cant read zip file");
        let zipfile = zipr.by_index(0).expect("Cant get zipped entries");
        let serialized: Vec<Entry> = serde_json::from_reader(zipfile).expect("cant read store file");
        for entry_ser in serialized {
            entries.push(entry_ser);
        }
        entries.sort_by(|a, b| a.uid.cmp(&b.uid));
        entries.dedup_by(|e1, e2| e1.uid == e2.uid);

        let f = File::create(&filepath).expect("Storer cant create file");
        let serialized = serde_json::to_string(entries).expect("Storer cant json entries (merge)");

        let mut zipw = zip::ZipWriter::new(&f);
        zipw.start_file("entries.json", zip::write::FileOptions::default()).expect("Cant start file (merge)");
        zipw.write(&serialized.into_bytes()).expect("Cant write entries (create)");
        zipw.finish().expect("Cant finish zip (merge)");

        entries.sort_by(|a, b| a.date.cmp(&b.date).reverse());
    }
}
