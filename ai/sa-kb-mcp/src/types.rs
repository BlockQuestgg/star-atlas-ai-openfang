use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Document {
    pub path: String,
    pub title: String,
    pub tags: Vec<String>,
    pub body: String,
    pub section: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Section {
    pub name: String,
    pub description: String,
    pub doc_count: usize,
}
