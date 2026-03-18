use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::format;
use crate::server::SaKbMcpServer;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocumentParams {
    /// Path to the document (e.g. "game-guides/sage-overview.md") or document title
    pub path: String,
    /// Whether to include frontmatter in the output (default: false)
    #[serde(default)]
    pub include_frontmatter: bool,
}

pub(crate) fn router() -> rmcp::handler::server::router::tool::ToolRouter<SaKbMcpServer> {
    SaKbMcpServer::documents_router()
}

#[rmcp::tool_router(router = documents_router)]
impl SaKbMcpServer {
    #[rmcp::tool(
        name = "get_document",
        description = "Retrieve a community vault document by path or title. Returns the full document content. Use list_sections or search to discover available documents first."
    )]
    pub(crate) async fn get_document(
        &self,
        Parameters(params): Parameters<GetDocumentParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let index = self.index.read().await;
        let doc = index
            .find_by_path(&params.path)
            .or_else(|| index.find_by_title(&params.path));

        match doc {
            Some(doc) => {
                let _ = params.include_frontmatter; // reserved for future use
                let json = format::format_document(doc, true);
                Ok(CallToolResult::success(vec![rmcp::model::Content::text(
                    json,
                )]))
            }
            None => Ok(CallToolResult::error(vec![rmcp::model::Content::text(
                format!("Document not found: {}", params.path),
            )])),
        }
    }
}
