mod api;
use crate::api::analyzer::AnalyzerImpl;
use crate::api::v1::analyzer_service_server::AnalyzerServiceServer;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = "8080";
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();

    let sv = tonic_reflection::server::Builder::configure()
        .with_service_name("derp")
        .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!("rust-analyzer"))
        .build()
        .unwrap();

    Server::builder()
        .add_service(AnalyzerServiceServer::new(AnalyzerImpl {}))
        .add_service(sv)
        .serve(addr)
        .await
        .unwrap();
    Ok(())
}
