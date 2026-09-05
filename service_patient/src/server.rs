pub mod application;
mod infrastructure;
pub mod module;
pub mod schema;

use crate::application::patient_service::{
    grpc::patient_service_server::PatientServiceServer, grpc::FILE_DESCRIPTOR_SET, Service,
};
use crate::infrastructure::database::setup_database_with_migration;
use crate::infrastructure::queue::setup_queue;
use dotenvy::dotenv;
use prost::Message;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("Starting server...");
    let (pool, channel) = tokio::join!(setup_database_with_migration(), setup_queue());

    let patient_service = Service::new(pool, channel);

    let addr = "[::1]:50051".parse()?;
    println!("Server started at {}", addr);

    let descriptor_set = prost_types::FileDescriptorSet::decode(FILE_DESCRIPTOR_SET)?;
    let reflection_service = Builder::configure()
        .register_file_descriptor_set(descriptor_set)
        .build_v1()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(PatientServiceServer::new(patient_service))
        .serve(addr)
        .await?;

    Ok(())
}
