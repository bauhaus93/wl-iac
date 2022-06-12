use mongodb::bson::{document::Document, oid::ObjectId};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Category {
    #[serde(skip)]
    id: Option<ObjectId>,
    name: Option<String>,
}

impl Category {
    pub fn get_id(&self) -> Option<&ObjectId> {
        self.id.as_ref()
    }
}

impl Default for Category {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
        }
    }
}

impl From<&Document> for Category {
    fn from(doc: &Document) -> Self {
        Self {
            id: doc.get_object_id("_id").map(|id| id.clone()).ok(),
            name: doc.get_str("name").map(String::from).ok(),
        }
    }
}

impl From<Document> for Category {
    fn from(doc: Document) -> Self {
        Self::from(&doc)
    }
}
