mod api;
use tokio::io;
use tonic::transport::Server;
use crate::api::analyzer::AnalyzerImpl;
use crate::api::v1::analyzer_service_server::AnalyzerServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = "8080";
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    Server::builder().add_service(AnalyzerServiceServer::new(AnalyzerImpl{})).serve(addr).await;
    Ok(())
}
