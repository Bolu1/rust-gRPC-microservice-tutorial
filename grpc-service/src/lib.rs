pub mod task_service {
    tonic::include_proto!("task_service");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("task_service_descriptor");
}