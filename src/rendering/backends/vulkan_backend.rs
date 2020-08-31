use enumflags2::BitFlags;

use gfx_backend_vulkan;

use crate::{
    core::GameLoopInterface,
    rendering::{
        backends::HalState,
        BackendInterface,
        RenderingRequirements
    },
    window::Window
};

pub struct VulkanBackend {
    hal_state: HalState<gfx_backend_vulkan::Backend>
}

impl VulkanBackend {
    pub fn new<L: 'static + GameLoopInterface>(window: Option<&Window<L>>) -> Result<Self, &'static str> {
        let hal_state = match window {
            Some(w) => {
                match HalState::new(w) {
                    Ok(state) => state,
                    Err(e) => {
                        panic!(format!("Can't create hal state.\n{}", e));
                        return Err("Can't create hal state.");
                    }
                }
            },
            None => return Err("Missing window reference.")
        };

        Ok(Self { 
            hal_state
        })
    }
}

impl BackendInterface for VulkanBackend {
    fn name() -> &'static str {
        "Vulkan"
    }

    fn has_requirements(requirements: RenderingRequirements) -> bool {
        true
    }

    fn draw_clear_frame(&mut self, color: [f32; 4]) {
        self.hal_state.draw_clear_frame(color)
    }
}
