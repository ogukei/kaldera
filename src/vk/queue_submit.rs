
use std::sync::{Arc, Mutex};

use crate::ffi::vk::*;

use super::error::Result;
use super::{Queue, CommandBuffer};

pub struct QueueSubmit {
    queue: Arc<Queue>,
    state: Mutex<QueueSubmitState>,
}

impl QueueSubmit {
    pub fn new(queue: &Arc<Queue>) -> Arc<Self> {
        let this = Self {
            queue: Arc::clone(queue),
            state: Mutex::new(QueueSubmitState::new())
        };
        Arc::new(this)
    }

    pub fn defer_submit(&self, command_buffer: &Arc<CommandBuffer>, wait_mask: VkPipelineStageFlags) {
        let command_buffer = Arc::clone(command_buffer);
        let task = QueueSubmitTask::new(command_buffer, wait_mask);
        let mut state = self.state.lock().unwrap();
        state.tasks.push(task);
    }

    pub fn execute(&self) -> Result<()> {
        unsafe {
            let device = self.queue.device();
            let state = self.state.lock().unwrap();
            let fences: Vec<VkFence> = state.tasks.iter()
                .map(|v| v.fence())
                .collect();
            vkResetFences(device.handle(), fences.len() as u32, fences.as_ptr())
                .into_result()?;
            for task in state.tasks.iter() {
                task.submit(&self.queue);
            }
            vkWaitForFences(device.handle(), fences.len() as u32, fences.as_ptr(), VK_TRUE, crate::vk::DEFAULT_TIMEOUT)
                .into_result()?;
        }
        Ok(())
    }
}

struct QueueSubmitState {
    tasks: Vec<QueueSubmitTask>,
}

impl QueueSubmitState {
    fn new() -> Self {
        Self {
            tasks: vec![],
        }
    }
}

struct QueueSubmitTask {
    command_buffer: Arc<CommandBuffer>,
    wait_mask: VkPipelineStageFlags,
}

impl QueueSubmitTask {
    fn new(command_buffer: Arc<CommandBuffer>, wait_mask: VkPipelineStageFlags) -> Self {
        Self { command_buffer, wait_mask }
    }

    unsafe fn submit(&self, queue: &Arc<Queue>) {
        let command_buffer_handle = self.command_buffer.handle();
        let fence = self.command_buffer.fence();
        let submit_info = VkSubmitInfo::with_command_buffer_wait(
            1,
            &command_buffer_handle,
            &self.wait_mask);
        vkQueueSubmit(queue.handle(), 1, &submit_info, fence);
    }

    fn fence(&self) -> VkFence {
        self.command_buffer.fence()
    }
}
