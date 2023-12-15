use crate::{numeric_types::u10, util::Truncate};
mod ump_stream_group;

pub mod device_identity;
pub mod end_of_clip;
pub mod endpoint_discovery;
pub mod endpoint_info;
pub mod endpoint_name;
pub mod function_block_discovery;
pub mod function_block_info;
pub mod function_block_name;
pub mod product_instance_id;
pub mod start_of_clip;
pub mod stream_configuration_notification;
pub mod stream_configuration_request;

const TYPE_CODE: u32 = 0xF;

fn status_from_buffer(buffer: &[u32]) -> u10 {
    (buffer[0] >> 16).truncate()
}
