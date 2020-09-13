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
    rendering::Renderer
};

pub struct GraphicRendererComponent {
    graphics: Vec<Box<dyn Graphic>>
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

impl Drop for GraphicRendererComponent {
    fn drop(&mut self) {
    }
}

impl GraphicRendererComponent {
    pub fn new() -> GraphicRendererComponent {
        GraphicRendererComponent {
            graphics: Vec::new()
        }
    }

    pub fn register(&mut self, graphic: Box<dyn Graphic>) {
        self.graphics.push(graphic);
    }
}
