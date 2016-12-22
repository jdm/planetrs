use chrono;
use uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub info: FeedInfo,
    pub title: String,
    pub date: chrono::datetime::DateTime<chrono::UTC>,
    pub hdate: String,
    pub uid: String,
    pub link: String,
    pub resume: String,
}

impl Entry {
    pub fn new() -> Entry {
        Entry {info: FeedInfo::new(),
               title: "".to_string(),
               date: chrono::UTC::now(),
               hdate: "".to_string(),
               uid: "".to_string(),
               link: "".to_string(),
               resume: "".to_string()}
    }

    pub fn generate_uid(&mut self) {
        let data = (*self.title).to_string() + &self.info.id;
        self.uid = uuid::Uuid::new_v5(&uuid::NAMESPACE_OID, &data).hyphenated().to_string();
    }

    pub fn generate_human_date(&mut self) {
        self.hdate = self.date.format("%B %d, %Y").to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedInfo {
    pub id: String,
    pub name: String,
    pub feedurl: String,
    pub homepage: String,
}

impl FeedInfo {
    pub fn new() -> FeedInfo {
        FeedInfo {id: "".to_string(),
                  name: "".to_string(),
                  feedurl: "".to_string(),
                  homepage: "".to_string()}
    }
}
