use tonic::transport::Server;

use generated::logger::logger_server::LoggerServer;
use services::filter_service::FilterConstraintsService;
use services::logger_service::LogCollectorService;

use crate::generated::filter_constraints::filter_constraints_server::FilterConstraintsServer;

mod generated;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    println!("Grpc Server Started");
    let log_collector = LogCollectorService::default();
    let filter_constraints_service = FilterConstraintsService::default();
    Server::builder()
        .add_service(LoggerServer::new(log_collector))
        .add_service(FilterConstraintsServer::new(filter_constraints_service))
        .serve(addr)
        .await?;
    Ok(())
}