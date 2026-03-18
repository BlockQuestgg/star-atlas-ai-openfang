use rmcp::model::CallToolResult;

use crate::format;
use crate::server::SaKbMcpServer;

pub(crate) fn router() -> rmcp::handler::server::router::tool::ToolRouter<SaKbMcpServer> {
    SaKbMcpServer::sections_router()
}

#[rmcp::tool_router(router = sections_router)]
impl SaKbMcpServer {
    #[rmcp::tool(
        name = "list_sections",
        description = "List all community vault sections with document counts and descriptions. Use this to discover what knowledge areas are available: developer, economy, game-guides, governance, lore."
    )]
    pub(crate) async fn list_sections(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let index = self.index.read().await;
        let json = format::format_sections(&index.sections);
        Ok(CallToolResult::success(vec![rmcp::model::Content::text(
            json,
        )]))
    }
}
