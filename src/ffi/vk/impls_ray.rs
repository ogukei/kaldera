

use super::types::*;
use super::types_ray::*;

impl Default for VkStridedBufferRegionKHR {
    fn default() -> Self {
        VkStridedBufferRegionKHR {
            buffer: std::ptr::null_mut(),
            offset: 0,
            stride: 0,
            size: 0,
        }
    }
}
