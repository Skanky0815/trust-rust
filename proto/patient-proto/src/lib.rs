pub mod patient_grpc {
    tonic::include_proto!("de.trustrust.grpc.patients");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("patients_descriptor");
}
