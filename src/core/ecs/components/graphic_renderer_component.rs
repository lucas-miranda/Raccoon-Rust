use std::{
    any::Any,
    borrow::BorrowMut,
    cell::RefMut,
};

use crate::{
    core::ecs::{
        components::Updatable,
        Component
    },
    graphics::{
        Drawable,
        Graphic
    },
    rendering::{
        backends::{
            GraphicsDevice,
            ResourceDisposable,
            panic_if_resource_isnt_disposed,
            panic_if_resources_isnt_disposed
        },
        Renderer
    }
};

pub struct GraphicRendererComponent {
    graphics: Vec<Box<dyn Graphic>>,
    disposed: bool
}

impl Component for GraphicRendererComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Updatable for GraphicRendererComponent {
    fn before_update(&mut self) {
    }

    fn update(&mut self) {
    }

    fn late_update(&mut self) {
    }
}

impl Drawable for GraphicRendererComponent {
    fn draw(&mut self, renderer: &mut Renderer) {
        self.graphics
            .iter_mut()
            .for_each(|graphic| graphic.draw(renderer.borrow_mut()))
    }
}

impl ResourceDisposable for GraphicRendererComponent {
    fn is_disposed(&self) -> bool {
        self.disposed
    }

    fn dispose(&mut self, device: &GraphicsDevice) {
        if self.disposed {
            return;
        }

        self.disposed = true;

        for graphic in self.graphics.iter_mut() {
            graphic.dispose(device);
        }
    }
}

impl Drop for GraphicRendererComponent {
    fn drop(&mut self) {
        panic_if_resource_isnt_disposed!(self);
        panic_if_resources_isnt_disposed!(self.graphics.iter());
    }
}

impl GraphicRendererComponent {
    pub fn new() -> GraphicRendererComponent {
        GraphicRendererComponent {
            graphics: Vec::new(),
            disposed: false
        }
    }

    pub fn register(&mut self, graphic: Box<dyn Graphic>) {
        self.graphics.push(graphic);
    }
}
