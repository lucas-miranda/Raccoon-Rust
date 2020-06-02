use std::time::{
    Duration,
    Instant
};

/// Provides info about Game underlying System.
pub struct System {
    _running: bool,
    _timer: Duration,
    _last_update_timer_checkpoint: Option<Instant>
}

impl System {
    pub fn is_running(&self) -> bool {
        self._running
    }

    pub fn get_timer(&self) -> &Duration {
        &self._timer
    }

    pub fn close_game(&mut self) {
        self._running = false;
    }

    pub(super) fn new() -> System {
        System {
            _running: false,
            _timer: Duration::new(0, 0),
            _last_update_timer_checkpoint: None
        }
    }

    pub(super) fn initialize(&mut self) {
        self._last_update_timer_checkpoint = Some(Instant::now());
    }

    pub(super) fn start(&mut self) {
        self._running = true;
    }

    pub(super) fn step_timer(&mut self) -> Duration {
        let last_update_timer = match self._last_update_timer_checkpoint {
            Some(t) => t,
            None => panic!("System was not initialized, can't step timer.")
        };

        let delta_time = Instant::now().duration_since(last_update_timer);
        self._timer += delta_time;
        self._last_update_timer_checkpoint = Some(Instant::now());
        delta_time
    }
}
