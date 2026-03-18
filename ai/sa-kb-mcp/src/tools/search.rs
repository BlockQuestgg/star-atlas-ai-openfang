use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::format;
use crate::server::SaKbMcpServer;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Search query — supports terms, phrases ("exact match"), and boolean operators (AND, OR). Searches across titles, bodies, and tags with BM25 ranking.
    pub query: String,
    /// Optional scope filter: a section name like "developer", "economy", "game-guides", "governance", "lore"
    #[serde(default)]
    pub scope: Option<String>,
    /// Maximum number of results to return (default: 10)
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

fn default_max_results() -> usize {
    10
}

pub(crate) fn router() -> rmcp::handler::server::router::tool::ToolRouter<SaKbMcpServer> {
    SaKbMcpServer::search_router()
}

#[rmcp::tool_router(router = search_router)]
impl SaKbMcpServer {
    #[rmcp::tool(
        name = "search",
        description = "Full-text search across the Star Atlas community knowledge base. Uses BM25 ranking with stemming. Supports phrases (\"exact match\") and boolean operators (AND, OR). Returns ranked results with excerpts. Use scope to filter by section: 'developer', 'economy', 'game-guides', 'governance', 'lore'."
    )]
    pub(crate) async fn search(
        &self,
        Parameters(params): Parameters<SearchParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let index = self.index.read().await;
        let results = self.search_engine.search(
            &index.documents,
            &params.query,
            params.scope.as_deref(),
            params.max_results,
        );

        let json = format::format_search(&params.query, &results, &index.documents);
        Ok(CallToolResult::success(vec![rmcp::model::Content::text(
            json,
        )]))
    }
}
