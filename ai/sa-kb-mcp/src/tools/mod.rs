pub(crate) mod documents;
pub(crate) mod reindex;
pub(crate) mod search;
pub(crate) mod sections;

use crate::server::SaKbMcpServer;
use rmcp::handler::server::router::tool::ToolRouter;

pub(crate) fn combined_router() -> ToolRouter<SaKbMcpServer> {
    sections::router() + documents::router() + search::router() + reindex::router()
}
