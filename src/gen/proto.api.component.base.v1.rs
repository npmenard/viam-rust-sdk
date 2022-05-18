// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStraightRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Desired travel distance in millimeters
    #[prost(int64, tag="2")]
    pub distance_mm: i64,
    /// Desired travel velocity in millimeters/second
    #[prost(double, tag="3")]
    pub mm_per_sec: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStraightResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveArcRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Desired travel distance in millimeters
    #[prost(int64, tag="2")]
    pub distance_mm: i64,
    /// Desired speed in millimeters per second 
    #[prost(double, tag="3")]
    pub mm_per_sec: f64,
    /// Desired angle in degrees
    #[prost(double, tag="4")]
    pub angle_deg: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveArcResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpinRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Desired angle
    #[prost(double, tag="2")]
    pub angle_deg: f64,
    /// Desired angular velocity
    #[prost(double, tag="3")]
    pub degs_per_sec: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpinResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPowerRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Desired linear power percentage as -1 -> 1 
    #[prost(message, optional, tag="2")]
    pub linear: ::core::option::Option<super::super::super::common::v1::Vector3>,
    /// Desired angular power percentage % as -1 -> 1 
    #[prost(message, optional, tag="3")]
    pub angular: ::core::option::Option<super::super::super::common::v1::Vector3>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPowerResponse {
}
/// Encoded file descriptor set for the `proto.api.component.base.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x98, 0x22, 0x0a, 0x26, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f, 0x76, 0x31,
    0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1b, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
    0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x61, 0x70,
    0x69, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d,
    0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x68, 0x0a, 0x13, 0x4d, 0x6f, 0x76, 0x65,
    0x53, 0x74, 0x72, 0x61, 0x69, 0x67, 0x68, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x5f,
    0x6d, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0a, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e,
    0x63, 0x65, 0x4d, 0x6d, 0x12, 0x1c, 0x0a, 0x0a, 0x6d, 0x6d, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x73,
    0x65, 0x63, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x6d, 0x6d, 0x50, 0x65, 0x72, 0x53,
    0x65, 0x63, 0x22, 0x16, 0x0a, 0x14, 0x4d, 0x6f, 0x76, 0x65, 0x53, 0x74, 0x72, 0x61, 0x69, 0x67,
    0x68, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x80, 0x01, 0x0a, 0x0e, 0x4d,
    0x6f, 0x76, 0x65, 0x41, 0x72, 0x63, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x5f, 0x6d, 0x6d,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0a, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65,
    0x4d, 0x6d, 0x12, 0x1c, 0x0a, 0x0a, 0x6d, 0x6d, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x73, 0x65, 0x63,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x6d, 0x6d, 0x50, 0x65, 0x72, 0x53, 0x65, 0x63,
    0x12, 0x1b, 0x0a, 0x09, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x64, 0x65, 0x67, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x01, 0x52, 0x08, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x44, 0x65, 0x67, 0x22, 0x11, 0x0a,
    0x0f, 0x4d, 0x6f, 0x76, 0x65, 0x41, 0x72, 0x63, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x22, 0x60, 0x0a, 0x0b, 0x53, 0x70, 0x69, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x64, 0x65, 0x67,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x44, 0x65, 0x67,
    0x12, 0x20, 0x0a, 0x0c, 0x64, 0x65, 0x67, 0x73, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x73, 0x65, 0x63,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0a, 0x64, 0x65, 0x67, 0x73, 0x50, 0x65, 0x72, 0x53,
    0x65, 0x63, 0x22, 0x0e, 0x0a, 0x0c, 0x53, 0x70, 0x69, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x22, 0x21, 0x0a, 0x0b, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x0e, 0x0a, 0x0c, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x93, 0x01, 0x0a, 0x0f, 0x53, 0x65, 0x74, 0x50, 0x6f, 0x77,
    0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x34, 0x0a,
    0x06, 0x6c, 0x69, 0x6e, 0x65, 0x61, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e,
    0x2e, 0x76, 0x31, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x52, 0x06, 0x6c, 0x69, 0x6e,
    0x65, 0x61, 0x72, 0x12, 0x36, 0x0a, 0x07, 0x61, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69,
    0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x33, 0x52, 0x07, 0x61, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x22, 0x12, 0x0a, 0x10, 0x53,
    0x65, 0x74, 0x50, 0x6f, 0x77, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x32,
    0x97, 0x06, 0x0a, 0x0b, 0x42, 0x61, 0x73, 0x65, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12,
    0xad, 0x01, 0x0a, 0x0c, 0x4d, 0x6f, 0x76, 0x65, 0x53, 0x74, 0x72, 0x61, 0x69, 0x67, 0x68, 0x74,
    0x12, 0x30, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4d,
    0x6f, 0x76, 0x65, 0x53, 0x74, 0x72, 0x61, 0x69, 0x67, 0x68, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x31, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31,
    0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x53, 0x74, 0x72, 0x61, 0x69, 0x67, 0x68, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x38, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x32, 0x22, 0x30, 0x2f,
    0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70,
    0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65,
    0x7d, 0x2f, 0x6d, 0x6f, 0x76, 0x65, 0x5f, 0x73, 0x74, 0x72, 0x61, 0x69, 0x67, 0x68, 0x74, 0x12,
    0x99, 0x01, 0x0a, 0x07, 0x4d, 0x6f, 0x76, 0x65, 0x41, 0x72, 0x63, 0x12, 0x2b, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e,
    0x74, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x41, 0x72,
    0x63, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x62,
    0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x41, 0x72, 0x63, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x33, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2d, 0x22, 0x2b,
    0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f, 0x7b, 0x6e, 0x61, 0x6d,
    0x65, 0x7d, 0x2f, 0x6d, 0x6f, 0x76, 0x65, 0x5f, 0x61, 0x72, 0x63, 0x12, 0x8c, 0x01, 0x0a, 0x04,
    0x53, 0x70, 0x69, 0x6e, 0x12, 0x28, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69,
    0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e,
    0x76, 0x31, 0x2e, 0x53, 0x70, 0x69, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x29,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
    0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x70, 0x69,
    0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x2f, 0x82, 0xd3, 0xe4, 0x93, 0x02,
    0x29, 0x22, 0x27, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f, 0x7b,
    0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x73, 0x70, 0x69, 0x6e, 0x12, 0x9d, 0x01, 0x0a, 0x08, 0x53,
    0x65, 0x74, 0x50, 0x6f, 0x77, 0x65, 0x72, 0x12, 0x2c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x62, 0x61,
    0x73, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x65, 0x74, 0x50, 0x6f, 0x77, 0x65, 0x72, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70,
    0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x62, 0x61, 0x73, 0x65,
    0x2e, 0x76, 0x31, 0x2e, 0x53, 0x65, 0x74, 0x50, 0x6f, 0x77, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x22, 0x34, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2e, 0x22, 0x2c, 0x2f, 0x76,
    0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
    0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d,
    0x2f, 0x73, 0x65, 0x74, 0x5f, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x12, 0x8c, 0x01, 0x0a, 0x04, 0x53,
    0x74, 0x6f, 0x70, 0x12, 0x28, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76,
    0x31, 0x2e, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x29, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e,
    0x65, 0x6e, 0x74, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x6f, 0x70,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x2f, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x29,
    0x22, 0x27, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f, 0x7b, 0x6e,
    0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x73, 0x74, 0x6f, 0x70, 0x42, 0x57, 0x0a, 0x28, 0x63, 0x6f, 0x6d,
    0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x72, 0x64, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x62, 0x61,
    0x73, 0x65, 0x2e, 0x76, 0x31, 0x5a, 0x2b, 0x67, 0x6f, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63,
    0x6f, 0x6d, 0x2f, 0x72, 0x64, 0x6b, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x61, 0x70, 0x69,
    0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f,
    0x76, 0x31, 0x4a, 0xae, 0x15, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x67, 0x1a, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x01, 0x00,
    0x42, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x01, 0x00, 0x42, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x02, 0x00, 0x41, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x02, 0x00,
    0x41, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x04, 0x00, 0x24, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x06, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x00,
    0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x09, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x13, 0x0a, 0xdc, 0x01, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x00, 0x12, 0x04, 0x0d, 0x02, 0x11, 0x03, 0x1a, 0xcd, 0x01, 0x20, 0x4d, 0x6f, 0x76, 0x65,
    0x53, 0x74, 0x72, 0x61, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6d, 0x6f, 0x76, 0x65, 0x73, 0x20, 0x61,
    0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27, 0x73, 0x20, 0x62, 0x61, 0x73, 0x65, 0x20, 0x69, 0x6e,
    0x20, 0x61, 0x20, 0x73, 0x74, 0x72, 0x61, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6c, 0x69, 0x6e, 0x65,
    0x20, 0x62, 0x79, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x64, 0x69, 0x73, 0x74,
    0x61, 0x6e, 0x63, 0x65, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x20, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x0a, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x73, 0x70, 0x65, 0x65,
    0x64, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x20, 0x70, 0x65, 0x72, 0x20,
    0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c,
    0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x63, 0x61,
    0x6e, 0x63, 0x65, 0x6c, 0x6c, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0d, 0x06, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x0d, 0x13, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d,
    0x31, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0e, 0x04, 0x10,
    0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04,
    0x0e, 0x04, 0x10, 0x06, 0x0a, 0x81, 0x02, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x04, 0x16,
    0x02, 0x1a, 0x03, 0x1a, 0xf2, 0x01, 0x20, 0x4d, 0x6f, 0x76, 0x65, 0x41, 0x72, 0x63, 0x20, 0x6d,
    0x6f, 0x76, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27, 0x73,
    0x20, 0x62, 0x61, 0x73, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x72, 0x63, 0x20,
    0x62, 0x79, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x64, 0x69, 0x73, 0x74, 0x61,
    0x6e, 0x63, 0x65, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x20, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2c, 0x0a, 0x20,
    0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x73, 0x70, 0x65, 0x65, 0x64, 0x2c, 0x20, 0x65,
    0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x69, 0x6c, 0x6c,
    0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x20, 0x70, 0x65, 0x72, 0x20, 0x73, 0x65, 0x63, 0x6f,
    0x6e, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x6f, 0x76, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2c, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x61, 0x6e, 0x67, 0x6c,
    0x65, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x64,
    0x65, 0x67, 0x72, 0x65, 0x65, 0x73, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c,
    0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x63, 0x61,
    0x6e, 0x63, 0x65, 0x6c, 0x6c, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x16, 0x06, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x16, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x16,
    0x27, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x17, 0x04, 0x19,
    0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x01, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04,
    0x17, 0x04, 0x19, 0x06, 0x0a, 0xc1, 0x01, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x04, 0x1f,
    0x02, 0x23, 0x03, 0x1a, 0xb2, 0x01, 0x20, 0x53, 0x70, 0x69, 0x6e, 0x20, 0x73, 0x70, 0x69, 0x6e,
    0x73, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27, 0x73, 0x20, 0x62, 0x61, 0x73, 0x65,
    0x20, 0x62, 0x79, 0x20, 0x61, 0x6e, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x61, 0x6e, 0x67,
    0x6c, 0x65, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e,
    0x20, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65, 0x73, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x20,
    0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x0a, 0x20, 0x61, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x20,
    0x73, 0x70, 0x65, 0x65, 0x64, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65, 0x73, 0x20, 0x70, 0x65, 0x72, 0x20,
    0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c,
    0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x63, 0x61,
    0x6e, 0x63, 0x65, 0x6c, 0x6c, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x1f, 0x06, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x1f, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1f,
    0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x20, 0x04, 0x22,
    0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x02, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04,
    0x20, 0x04, 0x22, 0x06, 0x0a, 0x47, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x04, 0x26, 0x02,
    0x2a, 0x03, 0x1a, 0x39, 0x20, 0x53, 0x65, 0x74, 0x50, 0x6f, 0x77, 0x65, 0x72, 0x20, 0x73, 0x65,
    0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x61, 0x72, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x61, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x20, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69,
    0x74, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x62, 0x61, 0x73, 0x65, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x26, 0x06, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x26, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x26, 0x29, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x27, 0x04, 0x29, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x03, 0x04, 0xb0,
    0xca, 0xbc, 0x22, 0x12, 0x04, 0x27, 0x04, 0x29, 0x06, 0x0a, 0x2a, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x04, 0x12, 0x04, 0x2d, 0x02, 0x31, 0x03, 0x1a, 0x1c, 0x20, 0x53, 0x74, 0x6f, 0x70, 0x20, 0x73,
    0x74, 0x6f, 0x70, 0x73, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27, 0x73, 0x20, 0x62,
    0x61, 0x73, 0x65, 0x20, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x2d, 0x06, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x2d, 0x0b,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2d, 0x21, 0x2d, 0x0a,
    0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2e, 0x04, 0x30, 0x06, 0x0a, 0x11,
    0x0a, 0x09, 0x06, 0x00, 0x02, 0x04, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x2e, 0x04, 0x30,
    0x06, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x34, 0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x34, 0x08, 0x1b, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x36, 0x02, 0x12, 0x1a, 0x10, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x61, 0x20, 0x62, 0x61, 0x73, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x36, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x36, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36,
    0x10, 0x11, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x38, 0x02, 0x18, 0x1a,
    0x28, 0x20, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x20, 0x74, 0x72, 0x61, 0x76, 0x65, 0x6c,
    0x20, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x69, 0x6c,
    0x6c, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x38, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x38, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x38, 0x16, 0x17, 0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x3a, 0x02, 0x18,
    0x1a, 0x2f, 0x20, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x20, 0x74, 0x72, 0x61, 0x76, 0x65,
    0x6c, 0x20, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x69,
    0x6c, 0x6c, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3a, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x09, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3a, 0x16, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x3d, 0x00, 0x1f, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x3d,
    0x08, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x3f, 0x00, 0x48, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x16, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x41, 0x02, 0x12, 0x1a, 0x10, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x62, 0x61, 0x73, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x41, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x41, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x41, 0x10, 0x11, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x43, 0x02, 0x18,
    0x1a, 0x28, 0x20, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x20, 0x74, 0x72, 0x61, 0x76, 0x65,
    0x6c, 0x20, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x69,
    0x6c, 0x6c, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x43, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x43, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x43, 0x16, 0x17, 0x0a, 0x37, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x45, 0x02,
    0x18, 0x1a, 0x2a, 0x20, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x20, 0x73, 0x70, 0x65, 0x65,
    0x64, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73,
    0x20, 0x70, 0x65, 0x72, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x20, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x45, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x45, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x45, 0x16, 0x17, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x47, 0x02, 0x17, 0x1a, 0x1a, 0x20, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x20, 0x61,
    0x6e, 0x67, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65, 0x73, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x47, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x47, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x47, 0x15, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x4a, 0x00, 0x1a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x4a, 0x08,
    0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x4c, 0x00, 0x53, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x13, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x4e, 0x02, 0x12, 0x1a, 0x10, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x61, 0x20, 0x62, 0x61, 0x73, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x4e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4e, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e,
    0x10, 0x11, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x50, 0x02, 0x17, 0x1a,
    0x0f, 0x20, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x50, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x50, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x50, 0x15, 0x16, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x02, 0x12, 0x03, 0x52, 0x02, 0x1a, 0x1a, 0x1a, 0x20, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65,
    0x64, 0x20, 0x61, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x20, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69,
    0x74, 0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x52, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x52, 0x09, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x52, 0x18, 0x19, 0x0a, 0x09, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x55, 0x00, 0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x55, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x57, 0x00, 0x5a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x57, 0x08, 0x13, 0x0a, 0x1d, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x59, 0x02, 0x12, 0x1a, 0x10, 0x20, 0x4e, 0x61, 0x6d, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x62, 0x61, 0x73, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x59, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x59, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x59, 0x10, 0x11, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x03, 0x5c, 0x00, 0x17,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x5c, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x08, 0x12, 0x04, 0x5e, 0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12,
    0x03, 0x5e, 0x08, 0x17, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x60, 0x02,
    0x12, 0x1a, 0x10, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x62, 0x61,
    0x73, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x60, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x60, 0x09, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60, 0x10, 0x11, 0x0a, 0x3a, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x62, 0x02, 0x1f, 0x1a, 0x2d, 0x20, 0x44, 0x65, 0x73,
    0x69, 0x72, 0x65, 0x64, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x61, 0x72, 0x20, 0x70, 0x6f, 0x77, 0x65,
    0x72, 0x20, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x20, 0x61, 0x73, 0x20,
    0x2d, 0x31, 0x20, 0x2d, 0x3e, 0x20, 0x31, 0x20, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x62, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x62, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x62, 0x1d, 0x1e, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x64, 0x02, 0x20,
    0x1a, 0x30, 0x20, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x67, 0x75, 0x6c,
    0x61, 0x72, 0x20, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x20, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74,
    0x61, 0x67, 0x65, 0x20, 0x25, 0x20, 0x61, 0x73, 0x20, 0x2d, 0x31, 0x20, 0x2d, 0x3e, 0x20, 0x31,
    0x20, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x06, 0x12, 0x03, 0x64, 0x02, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x64, 0x14, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x64, 0x1e, 0x1f, 0x0a, 0x09, 0x0a, 0x02,
    0x04, 0x09, 0x12, 0x03, 0x67, 0x00, 0x1a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03,
    0x67, 0x08, 0x18, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("proto.api.component.base.v1.tonic.rs");
// @@protoc_insertion_point(module)