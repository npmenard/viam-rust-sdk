// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncResponse {
}
/// Encoded file descriptor set for the `viam.service.datamanager.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe5, 0x06, 0x0a, 0x29, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x64, 0x61, 0x74,
    0x61, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x64, 0x61, 0x74, 0x61,
    0x5f, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1b,
    0x76, 0x69, 0x61, 0x6d, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x64, 0x61, 0x74,
    0x61, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x21, 0x0a, 0x0b, 0x53, 0x79, 0x6e,
    0x63, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x0e, 0x0a, 0x0c,
    0x53, 0x79, 0x6e, 0x63, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x32, 0xac, 0x01, 0x0a,
    0x12, 0x44, 0x61, 0x74, 0x61, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x53, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x12, 0x95, 0x01, 0x0a, 0x04, 0x53, 0x79, 0x6e, 0x63, 0x12, 0x28, 0x2e, 0x76,
    0x69, 0x61, 0x6d, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x64, 0x61, 0x74, 0x61,
    0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x29, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65,
    0x72, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x79, 0x6e, 0x63, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0x38, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x32, 0x22, 0x30, 0x2f, 0x76, 0x69, 0x61, 0x6d,
    0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f,
    0x64, 0x61, 0x74, 0x61, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2f, 0x7b, 0x6e, 0x61, 0x6d,
    0x65, 0x7d, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x73, 0x79, 0x6e, 0x63, 0x42, 0x49, 0x0a, 0x1f, 0x63,
    0x6f, 0x6d, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x64, 0x61, 0x74, 0x61, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x5a, 0x26,
    0x67, 0x6f, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x6d, 0x61, 0x6e, 0x61,
    0x67, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x4a, 0xc7, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x17,
    0x17, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x00, 0x24, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x26,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x3d, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b,
    0x12, 0x03, 0x06, 0x00, 0x3d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x38, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x07, 0x00, 0x38, 0x0a, 0x58, 0x0a, 0x02, 0x06, 0x00,
    0x12, 0x04, 0x0a, 0x00, 0x11, 0x01, 0x1a, 0x4c, 0x20, 0x41, 0x20, 0x44, 0x61, 0x74, 0x61, 0x4d,
    0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x73, 0x20, 0x64, 0x61,
    0x74, 0x61, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x6f, 0x62, 0x6f, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x6f,
    0x75, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x1a,
    0x0a, 0x5c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x0c, 0x02, 0x10, 0x03, 0x1a, 0x4e,
    0x20, 0x53, 0x79, 0x6e, 0x63, 0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x73, 0x20, 0x61,
    0x20, 0x73, 0x79, 0x6e, 0x63, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x6e,
    0x2d, 0x73, 0x79, 0x6e, 0x63, 0x65, 0x64, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2c, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x06, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0c, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x0d, 0x04, 0x0f, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x00, 0x04,
    0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x0d, 0x04, 0x0f, 0x06, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x13, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x13,
    0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x02, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x10, 0x11, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x17, 0x00, 0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x17, 0x08, 0x14,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("viam.service.datamanager.v1.tonic.rs");
// @@protoc_insertion_point(module)