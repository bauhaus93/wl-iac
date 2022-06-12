use mongodb::bson::document::Document;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Datapoint {
    date: Option<String>,
    value: Option<i32>,
}

impl From<&Document> for Datapoint {
    fn from(doc: &Document) -> Self {
        Self {
            date: doc.get_str("date").map(|s| s.to_owned()).ok(),
            value: doc.get_i32("value").ok(),
        }
    }
}
