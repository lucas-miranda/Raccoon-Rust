use enumflags2::BitFlags;

use crate::{
    rendering::{
        //backends::HalState,
        BackendInterface,
        RenderingRequirements
    },
    window::Window
};

pub struct VulkanBackend {
    //hal_state: HalState
}

impl VulkanBackend {
    pub fn new(window: Option<&Window>) -> Result<Self, &'static str> {
        /*
        let hal_state = match window {
            Some(w) => {
                match HalState::new(w) {
                    Ok(state) => state,
                    Err(e) => return Err("Can't create hal state.")
                }
            },
            None => return Err("Missing window reference.")
        };

        Ok(Self { 
            hal_state
        })
        */

        Ok(Self {
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
}
