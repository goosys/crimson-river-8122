use poem_mcpserver::{content::Text, stdio::stdio, McpServer, Tools};

struct GeneratorTool {}

#[Tools]
impl GeneratorTool {
    /// Return a Heroku-like memorable random string
    async fn haikunate(&self) -> Text<String> {
        Text(haikunator::Haikunator::default().haikunate())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    stdio(McpServer::new().tools(GeneratorTool {})).await
}
