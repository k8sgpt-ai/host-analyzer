pub mod analyzer;

use tonic_reflection::server::{Error, ServerReflection, ServerReflectionServer};

pub mod v1 {
    // handy macro includes generated proto stubs in this module.
    tonic::include_proto!("schema.v1");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("rust-analyzer");
}

pub fn api_service() -> Result<ServerReflectionServer<impl ServerReflection>, Error> {
    let spec = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(v1::FILE_DESCRIPTOR_SET)
        .build()?;
    Ok(spec)
}

// RpcResult will help to make our service implementation easier to read.
pub type RpcResult<T> = std::result::Result<tonic::Response<T>, tonic::Status>;
