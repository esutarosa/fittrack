#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fittrack::server::run().await
}
