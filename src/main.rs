use winit::{
    dpi::LogicalSize,
    CreationError,
    Event,
    EventsLoop,
    Window,
    WindowBuilder,
    WindowEvent
};

pub const WINDOW_NAME: &str = "Hello Winit";

#[derive(Debug)]
pub struct WinitState {
    pub events_loop: EventsLoop,
    pub window: Window
}

impl WinitState{
    pub fn new<T: Into<String>>(title: T, size: LogicalSize) -> Result<Self, CreationError> {
        let events_loop = EventsLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(size)
            .build(&events_loop);

        output.map(|window| Self {
            events_loop,
            window
        })
    }
}

impl Default for WinitState {
    fn default() -> Self {
        Self::new(
            WINDOW_NAME,
            LogicalSize {
                width: 800.0,
                height: 600.0
            }
        )
        .expect("Could not create a window!")
    }
}

fn main() {
    let mut winit_state = WinitState::default();
    let mut running = true;

    while running {
        winit_state.events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => running = false,
            _ => ()
        });
    }
}
