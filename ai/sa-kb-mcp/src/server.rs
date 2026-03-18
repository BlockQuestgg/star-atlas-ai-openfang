use std::path::PathBuf;
use std::sync::Arc;

use rmcp::ServerHandler;
use rmcp::model::{Implementation, ServerCapabilities, ServerInfo};
use tokio::sync::RwLock;

use crate::index::Index;
use crate::search::SearchEngine;
use crate::tools;

#[derive(Clone)]
pub struct SaKbMcpServer {
    pub(crate) index: Arc<RwLock<Index>>,
    pub(crate) search_engine: Arc<SearchEngine>,
    pub(crate) vault_path: PathBuf,
    tool_router: rmcp::handler::server::router::tool::ToolRouter<Self>,
}

impl SaKbMcpServer {
    pub fn new(index: Index, search_engine: SearchEngine, vault_path: PathBuf) -> Self {
        Self {
            index: Arc::new(RwLock::new(index)),
            search_engine: Arc::new(search_engine),
            vault_path,
            tool_router: tools::combined_router(),
        }
    }
}

#[rmcp::tool_handler]
impl ServerHandler for SaKbMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("sa-kb-mcp", env!("CARGO_PKG_VERSION")))
            .with_instructions(
                "Star Atlas community knowledge base MCP server. Indexes the community \
                 vault — game guides, economy data, governance docs, developer resources, \
                 and lore. Use list_sections to discover content areas (developer, economy, \
                 game-guides, governance, lore), search to find documents by keyword, \
                 get_document to retrieve full content, and reindex to refresh after \
                 adding new files."
                    .to_string(),
            )
    }
}
