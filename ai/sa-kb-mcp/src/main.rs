mod cli;
mod format;
mod index;
mod search;
mod server;
mod tools;
mod types;

use std::path::PathBuf;

use clap::Parser;
use rmcp::ServiceExt;
use rmcp::transport::io::stdio;

use crate::index::Index;
use crate::search::SearchEngine;
use crate::server::SaKbMcpServer;

struct Config {
    vault_path: String,
}

fn load_config() -> Config {
    // 1. SA_COMMUNITY_VAULT_PATH env var (preferred for cross-project use)
    if let Ok(path) = std::env::var("SA_COMMUNITY_VAULT_PATH") {
        let p = PathBuf::from(&path);
        if p.is_dir() {
            return Config { vault_path: path };
        }
    }

    // 2. Compile-time fallback (cargo run from repo)
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("failed to find repo root");

    Config {
        vault_path: repo_root
            .join("vaults")
            .join("community")
            .to_string_lossy()
            .to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    let vault_path = PathBuf::from(&config.vault_path);

    let index = Index::build(&vault_path);
    let search_engine = SearchEngine::build(&index.documents);

    // Dual-mode: CLI if args provided, MCP stdio server otherwise
    if std::env::args().len() > 1 {
        let parsed = cli::Cli::parse();
        cli::run(parsed, &index, &search_engine, &vault_path);
        return Ok(());
    }

    // MCP stdio server mode
    let server = SaKbMcpServer::new(index, search_engine, vault_path);
    let service = server.serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}
