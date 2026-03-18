use std::path::Path;

use clap::{Parser, Subcommand};

use crate::format;
use crate::index::Index;
use crate::search::SearchEngine;

#[derive(Parser)]
#[command(name = "sa-kb-mcp", about = "Star Atlas community knowledge base MCP server and CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Output as JSON (default, kept for compatibility)
    #[arg(long, global = true, default_value_t = true)]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// List all vault sections with doc counts
    ListSections,
    /// Get a document by path or title
    GetDocument {
        /// Document path (e.g. "game-guides/sage-overview.md") or title
        #[arg(long)]
        path: String,
    },
    /// Search across the community vault
    Search {
        /// Search query (supports phrases and boolean operators)
        #[arg(long)]
        query: String,
        /// Scope filter: a section name like "game-guides", "governance", "lore"
        #[arg(long)]
        scope: Option<String>,
        /// Maximum results (default: 10)
        #[arg(long, default_value_t = 10)]
        max_results: usize,
    },
    /// Rebuild the search index from disk
    Reindex,
}

pub fn run(cli: Cli, index: &Index, search_engine: &SearchEngine, vault_path: &Path) {
    match cli.command {
        Command::ListSections => {
            println!("{}", format::format_sections(&index.sections));
        }
        Command::GetDocument { path } => {
            let doc = index
                .find_by_path(&path)
                .or_else(|| index.find_by_title(&path));

            match doc {
                Some(doc) => println!("{}", format::format_document(doc, true)),
                None => {
                    eprintln!("Document not found: {}", path);
                    std::process::exit(1);
                }
            }
        }
        Command::Search {
            query,
            scope,
            max_results,
        } => {
            let results =
                search_engine.search(&index.documents, &query, scope.as_deref(), max_results);
            println!(
                "{}",
                format::format_search(&query, &results, &index.documents)
            );
        }
        Command::Reindex => {
            let new_index = search_engine.reindex(vault_path);
            println!(
                "Reindexed {} documents across {} sections",
                new_index.documents.len(),
                new_index.sections.len()
            );
        }
    }
}
