use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use tera::{Tera, Context};

use entry::Entry;
use entry::FeedInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub feeds: Vec<FeedInfo>,
    pub entries: Vec<Entry>,
}

impl Data {
    pub fn new() -> Data {
        Data {feeds: Vec::new(), entries: Vec::new()}
    }
}

pub fn render<P: AsRef<Path>>(data: &Data, template_name: &str, outputfile: P) {
    let mut tera = Tera::new("./templates/**/*.html").expect("Cant compile html");
    tera.autoescape_on(vec![]);
    let mut context = Context::new();
    context.add("data", data);
    let output = tera.render(template_name, context).expect("Tera couldnt render output");
    let mut f = File::create(outputfile).expect("Cant create file for html output");
    let _ = f.write_all(output.as_bytes());
}
