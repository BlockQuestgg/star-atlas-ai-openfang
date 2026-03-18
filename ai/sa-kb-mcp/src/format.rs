use serde::Serialize;

use crate::search::SearchResult;
use crate::types::{Document, Section};

#[derive(Serialize)]
pub struct SectionOutput {
    pub name: String,
    pub description: String,
    pub doc_count: usize,
}

#[derive(Serialize)]
pub struct DocumentOutput {
    pub path: String,
    pub title: String,
    pub tags: Vec<String>,
    pub section: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResultOutput {
    pub path: String,
    pub title: String,
    pub section: String,
    pub score: f64,
    pub excerpt: String,
}

#[derive(Serialize)]
pub struct SearchOutput {
    pub query: String,
    pub total: usize,
    pub results: Vec<SearchResultOutput>,
}

pub fn format_sections(sections: &[Section]) -> String {
    let output: Vec<SectionOutput> = sections
        .iter()
        .map(|s| SectionOutput {
            name: s.name.clone(),
            description: s.description.clone(),
            doc_count: s.doc_count,
        })
        .collect();

    serde_json::to_string_pretty(&output).unwrap_or_default()
}

pub fn format_document(doc: &Document, include_content: bool) -> String {
    let output = DocumentOutput {
        path: doc.path.clone(),
        title: doc.title.clone(),
        tags: doc.tags.clone(),
        section: doc.section.clone(),
        content: if include_content {
            Some(doc.body.clone())
        } else {
            None
        },
    };

    serde_json::to_string_pretty(&output).unwrap_or_default()
}

pub fn format_search(query: &str, results: &[SearchResult], documents: &[Document]) -> String {
    let result_outputs: Vec<SearchResultOutput> = results
        .iter()
        .map(|r| {
            let doc = &documents[r.doc_index];
            SearchResultOutput {
                path: doc.path.clone(),
                title: doc.title.clone(),
                section: doc.section.clone(),
                score: (r.score * 1000.0).round() / 1000.0,
                excerpt: r.excerpt.clone(),
            }
        })
        .collect();

    let output = SearchOutput {
        query: query.to_string(),
        total: result_outputs.len(),
        results: result_outputs,
    };

    serde_json::to_string_pretty(&output).unwrap_or_default()
}
