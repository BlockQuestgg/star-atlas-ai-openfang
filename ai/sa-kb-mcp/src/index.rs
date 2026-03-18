use std::collections::HashMap;
use std::path::Path;

use crate::types::{Document, Section};

const SECTION_DESCRIPTIONS: &[(&str, &str)] = &[
    (
        "developer",
        "F-KIT, SDKs, Solana programs, building on the Star Atlas platform",
    ),
    (
        "economy",
        "ATLAS/POLIS tokens, galactic marketplace, trading, and DeFi",
    ),
    (
        "game-guides",
        "SAGE gameplay, fleet management, resources, crafting, and roadmap",
    ),
    (
        "governance",
        "DAO structure, POLIS voting, PIPs, treasury management",
    ),
    (
        "lore",
        "Factions (MUD, ONI, Ustur), Galia Expanse, narrative, and world-building",
    ),
];

pub struct Index {
    pub documents: Vec<Document>,
    pub sections: Vec<Section>,
}

impl Index {
    pub fn build(vault_path: &Path) -> Self {
        let mut documents = Vec::new();

        if vault_path.is_dir() {
            scan_dir(vault_path, vault_path, &mut documents);
        }

        let sections = build_sections(&documents);

        Index {
            documents,
            sections,
        }
    }

    pub fn find_by_path(&self, path: &str) -> Option<&Document> {
        self.documents.iter().find(|d| d.path == path)
    }

    pub fn find_by_title(&self, title: &str) -> Option<&Document> {
        let lower = title.to_lowercase();
        self.documents
            .iter()
            .find(|d| d.title.to_lowercase() == lower)
    }
}

fn scan_dir(base: &Path, dir: &Path, docs: &mut Vec<Document>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Skip hidden directories (.obsidian, etc.)
        if path.is_dir() {
            if path
                .file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with('.'))
            {
                continue;
            }
            scan_dir(base, &path, docs);
        } else if path.extension().is_some_and(|e| e == "md")
            && let Some(doc) = parse_document(base, &path)
        {
            docs.push(doc);
        }
    }
}

fn parse_document(base: &Path, file_path: &Path) -> Option<Document> {
    let content = std::fs::read_to_string(file_path).ok()?;
    let rel_path = file_path.strip_prefix(base).ok()?;
    let rel_str = rel_path.to_string_lossy().to_string();

    let section = if rel_path.components().count() > 1 {
        rel_path
            .components()
            .next()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .unwrap_or_default()
    } else {
        String::new()
    };

    let (frontmatter_title, tags, body) = parse_frontmatter(&content);

    let title = frontmatter_title
        .or_else(|| {
            body.lines()
                .find(|l| l.starts_with("# "))
                .map(|l| l.trim_start_matches("# ").to_string())
        })
        .unwrap_or_else(|| {
            rel_path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default()
        });

    Some(Document {
        path: rel_str,
        title,
        tags,
        body,
        section,
    })
}

fn parse_frontmatter(content: &str) -> (Option<String>, Vec<String>, String) {
    if !content.starts_with("---") {
        return (None, vec![], content.to_string());
    }

    let after_first = &content[3..];
    if let Some(end) = after_first.find("\n---") {
        let yaml_str = &after_first[..end];
        let body = &after_first[end + 4..];

        let (title, tags) = extract_frontmatter(yaml_str);
        (title, tags, body.trim_start_matches('\n').to_string())
    } else {
        (None, vec![], content.to_string())
    }
}

fn extract_frontmatter(yaml_str: &str) -> (Option<String>, Vec<String>) {
    #[derive(serde::Deserialize)]
    struct Frontmatter {
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        tags: Vec<String>,
    }

    serde_yaml::from_str::<Frontmatter>(yaml_str)
        .map(|fm| (fm.title, fm.tags))
        .unwrap_or_default()
}

fn build_sections(documents: &[Document]) -> Vec<Section> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for doc in documents {
        if !doc.section.is_empty() {
            *counts.entry(doc.section.clone()).or_default() += 1;
        }
    }

    let mut sections: Vec<Section> = counts
        .into_iter()
        .map(|(name, doc_count)| {
            let description = SECTION_DESCRIPTIONS
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, d)| d.to_string())
                .unwrap_or_default();

            Section {
                name,
                description,
                doc_count,
            }
        })
        .collect();

    sections.sort_by(|a, b| a.name.cmp(&b.name));
    sections
}
