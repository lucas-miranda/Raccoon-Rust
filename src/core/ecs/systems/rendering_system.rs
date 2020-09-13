use std::{
    any::Any,
    borrow::Borrow,
    cell::{
        Ref,
        RefCell
    },
    rc::Weak
};

use crate::{
    core::{
        ecs::{
            components::{
                GraphicRendererComponent
            },
            containers::{
                SimpleDataContainer
            },
            System,
        },
        GameState
    },
    graphics::{
        Drawable,
        Graphic
    },
    rendering::Renderer
};

pub struct RenderingSystem {
    renderer: Weak<RefCell<Renderer>>
}

impl System for RenderingSystem {
    type DataType = SimpleDataContainer<GraphicRendererComponent>;

    fn setup(&mut self, _game_state: &mut Ref<GameState>) {
    }

    fn run(&mut self, components: &mut Self::DataType, _game_state: &mut Ref<GameState>) {
        match self.renderer.upgrade() {
            Some(renderer_strong_ref) => {
                let mut renderer = <_ as Borrow<RefCell<Renderer>>>::borrow(&renderer_strong_ref)
                                                                    .borrow_mut();
                components.components_mut()
                          .for_each(|component| {
                              component.draw(&mut renderer);
                          })
            },
            None => ()
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl RenderingSystem {
    pub fn new(renderer: Weak<RefCell<Renderer>>) -> RenderingSystem {
        RenderingSystem {
            renderer
        }
    }
}
