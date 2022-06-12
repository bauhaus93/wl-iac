use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_offset")]
    offset: u64,
    #[serde(default = "default_size")]
    size: u64,
}

#[derive(Deserialize)]
pub struct CategoryQuery {
    #[serde(default = "Option::default")]
    category: Option<String>,
}

impl ListQuery {
    pub fn get_offset(&self) -> u64 {
        self.offset
    }
    pub fn get_size(&self) -> u64 {
        self.size
    }
}

impl CategoryQuery {
    pub fn get_category(&self) -> Option<&str> {
        match &self.category {
            Some(s) => Some(s.as_ref()),
            None => None,
        }
    }
}

fn default_offset() -> u64 {
    0
}

fn default_size() -> u64 {
    10
}
