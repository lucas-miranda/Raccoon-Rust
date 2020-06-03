use std::{
    any::Any,
    time::Duration
};

use crate::{
    core::{
        System,
        entity::{
            Component,
            Entity
        }
    }
};

pub struct TransformComponent {
    _x: f32,
    _y: f32
}

impl Component for TransformComponent {
    fn added(&self, entity: &mut Entity) {
        println!("transform component added!");
    }

    fn removed(&self, entity: &Entity) {
    }

    fn update(&mut self, delta_time: &Duration, entity: &mut Entity, system: &System) {
    }

    fn render(&self, entity: &Entity) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        TransformComponent {
            _x: 0f32,
            _y: 0f32
        }
    }

    pub fn x(&self) -> f32 {
        self._x
    }

    pub fn y(&self) -> f32 {
        self._y
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self._x += x;
        self._y += y;
    }

    pub fn change(&mut self, x: f32, y: f32) {
        self._x += x;
        self._y += y;
    }

    pub fn change_x(&mut self, change: f32) {
        self._x += change;
    }

    pub fn change_y(&mut self, change: f32) {
        self._y += change;
    }
}
