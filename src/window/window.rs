use winit::{
    dpi::{
        LogicalSize
    },
    CreationError,
    EventsLoop
};

pub struct Window {
    pub events_loop: EventsLoop,
    pub winit_window: winit::Window,
    _title: String
}

impl Window {
    pub fn new<T: Into<String>>(title: T, size: LogicalSize) -> Result<Self, CreationError> {
        let window_title = title.into();
        let events_loop = EventsLoop::new();
        let output = winit::WindowBuilder::new()
            .with_title(&window_title)
            .with_dimensions(size)
            .build(&events_loop);

        output.map(|winit_window| Self {
            events_loop,
            winit_window,
            _title: window_title
        })
    }

    pub fn get_title(&self) -> &String {
        &self._title
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new(
            "Default Window",
            LogicalSize {
                width: 480.0,
                height: 320.0
            }
        )
        .expect("Could not create a window!")
    }
}
