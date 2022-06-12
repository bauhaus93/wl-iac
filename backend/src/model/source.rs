use mongodb::bson::{document::Document, oid::ObjectId};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Source {
    #[serde(skip)]
    id: Option<ObjectId>,
    name: Option<String>,
    url: Option<String>,
}

impl From<&Document> for Source {
    fn from(doc: &Document) -> Self {
        Self {
            id: doc.get_object_id("_id").map(|id| id.clone()).ok(),
            name: doc.get_str("name").map(String::from).ok(),
            url: doc.get_str("url").map(String::from).ok(),
        }
    }
}
