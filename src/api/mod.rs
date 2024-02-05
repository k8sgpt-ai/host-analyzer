pub mod analyzer;

use tonic_reflection::server::{Error, ServerReflection, ServerReflectionServer};

pub mod v1 {
    // handy macro includes generated proto stubs in this module.
    tonic::include_proto!("schema.v1");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("rust-analyzer");
}
