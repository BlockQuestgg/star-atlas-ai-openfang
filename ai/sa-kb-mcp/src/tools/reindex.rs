use rmcp::model::CallToolResult;

use crate::server::SaKbMcpServer;

pub(crate) fn router() -> rmcp::handler::server::router::tool::ToolRouter<SaKbMcpServer> {
    SaKbMcpServer::reindex_router()
}

#[rmcp::tool_router(router = reindex_router)]
impl SaKbMcpServer {
    #[rmcp::tool(
        name = "reindex",
        description = "Rebuild the search index from the community vault on disk. Use this after adding or editing documents mid-session to make new content searchable."
    )]
    pub(crate) async fn reindex(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let new_index = self.search_engine.reindex(&self.vault_path);
        let new_sections = new_index.sections.clone();
        let doc_count = new_index.documents.len();

        {
            let mut index = self.index.write().await;
            *index = new_index;
        }

        let msg = format!(
            "Reindexed {} documents across {} sections",
            doc_count,
            new_sections.len()
        );
        Ok(CallToolResult::success(vec![rmcp::model::Content::text(
            msg,
        )]))
    }
}
