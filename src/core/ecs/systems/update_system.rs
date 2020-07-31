use std:: {
    any::Any,
    time::{
        Duration,
        Instant
    }
};

use crate::{
    core::{
        ecs::{
            components::{
                EmptyComponent
            },
            Component,
            SimpleDataContainer,
            System
        },
        GameController
    }
};

pub struct UpdateSystem {
    timer: Duration,
    last_update_timer_checkpoint: Option<Instant>,
    last_update_delta_time: Duration
}

type ComponentCollection = Vec<Box<dyn Component>>;
impl System for UpdateSystem {
    type DataType = (
        SimpleDataContainer<EmptyComponent>,
        SimpleDataContainer<EmptyComponent>
    );

    fn setup(&mut self, game_controller: &mut GameController) {
        self.last_update_timer_checkpoint = Some(Instant::now());
    }

    fn run(&mut self, game_controller: &mut GameController) {
        self.step_timer();
        println!("dt: {:?}, et: {:?}", self.get_update_delta_time(), self.get_timer());

        if self.timer.as_secs() >= 3 {
            println!("Timer test has ended!");
            game_controller.close_game();
        }
    }

    fn handle(&mut self, (a, b): &Self::DataType, game_controller: &mut GameController) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl UpdateSystem {
    pub fn new() -> UpdateSystem {
        UpdateSystem {
            timer: Duration::new(0, 0),
            last_update_timer_checkpoint: None,
            last_update_delta_time: Duration::new(0, 0)
        }
    }

    pub fn get_timer(&self) -> &Duration {
        &self.timer
    }

    pub fn get_update_delta_time(&self) -> &Duration {
        &self.last_update_delta_time
    }

    fn step_timer(&mut self) {
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
