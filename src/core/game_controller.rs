use crate::{
    input::{
        Input
    },
    tools::{
        log::Logger
    }
};

pub struct GameController {
    pub input: Input,
    pub logger: Logger, 

    is_running: bool
}

impl GameController {
    pub fn new() -> GameController {
        GameController {
            input: Input::new(),
            logger: Logger::new(),
            is_running: false
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn close_game(&mut self) {
        self.is_running = false;
    }

    pub(in crate::core) fn start(&mut self) {
        self.is_running = true;
    }
}
