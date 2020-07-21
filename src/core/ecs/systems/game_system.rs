use std:: {
    any::Any,
    time::{
        Duration,
        Instant
    }
};

use crate::core::ecs::{
    System
};

/// Provides info about Game underlying System.
pub struct GameSystem {
    is_running: bool,

    // time
    timer: Duration,
    last_update_timer_checkpoint: Option<Instant>,
    last_update_delta_time: Duration
}

impl System for GameSystem {
    type DataType = ();

    fn run(&mut self, nothing: ()) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl GameSystem {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn get_timer(&self) -> &Duration {
        &self.timer
    }

    pub fn get_update_delta_time(&self) -> &Duration {
        &self.last_update_delta_time
    }

    pub fn close_game(&mut self) {
        self.is_running = false;
    }

    pub(in crate::core) fn initialize(&mut self) {
        self.last_update_timer_checkpoint = Some(Instant::now());
    }

    pub(in crate::core) fn start(&mut self) {
        self.is_running = true;
    }

    pub(in crate::core) fn step_timer(&mut self) {
        let last_update_timer = match self.last_update_timer_checkpoint {
            Some(t) => t,
            None => panic!("System was not initialized, can't step timer.")
        };

        let delta_time = Instant::now().duration_since(last_update_timer);
        self.timer += delta_time;
        self.last_update_timer_checkpoint = Some(Instant::now());
        self.last_update_delta_time = delta_time;
    }
}
