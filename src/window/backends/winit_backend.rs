use std::{
    borrow::Borrow,
    collections::HashMap,
    cell::{ RefCell, RefMut },
    rc::{ Rc, Weak }
};

use raw_window_handle::{
    HasRawWindowHandle,
    RawWindowHandle
};

use winit::{
    dpi::{
        LogicalPosition,
        LogicalSize,
        Pixel
    },
    error::{
        OsError
    },
    event::{
        VirtualKeyCode
    },
    event_loop::{
        ControlFlow,
        EventLoop
    },
    window::WindowBuilder
};

use crate::{
    core::{
        ecs::Realm,
        GameState
    },
    events::Event,
    input::{
        ButtonState,
        InputEvent,
        InputEventListener,
        KeyboardEvent,
        KeyCode,
        KeyModifiers,
        MouseButton,
        MouseButtonEvent,
        MouseScrollDelta,
        TouchEvent,
        TouchPhase
    },
    math::{
        Size,
        Vector2
    },
    window::{
        backends::{
            BackendInterface,
            InputEventsHandler,
            InputEventsIndirectHandler,
            WindowEventsHandler
        },
        WindowEvent,
        WindowEventListener
    }
};


pub struct WinitBackend {
    event_loop: Option<EventLoop<()>>,
    winit_window: winit::window::Window,
    //window_events: Vec<Event<WindowEvent>>,

    // input
    //input_events: Vec<Event<InputEvent>>
}

unsafe impl HasRawWindowHandle for WinitBackend {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.winit_window.raw_window_handle()
    }
}

impl BackendInterface for WinitBackend {
    fn run(&mut self, game_state: Weak<RefCell<GameState>>, realm: Realm) {
        let event_loop = self.event_loop.take()
                                        .expect("Expecting winit event loop.");

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match game_state.upgrade() {
                Some(ref mut game_state_strong_ref) => {
                    let mut state = <_ as Borrow<RefCell<GameState>>>::borrow(game_state_strong_ref)
                                                                      .borrow_mut();
                    match event {
                        winit::event::Event::WindowEvent { window_id: _,  event } => {
                            match event {
                                winit::event::WindowEvent::CloseRequested => {
                                    println!("Close was requested by system.");
                                    state.close_game();
                                },
                                _ => ()
                            }
                        },
                        _ => ()
                    }

                    if !state.is_running() {
                        if let ControlFlow::Exit = *control_flow {
                        } else {
                            println!("Close was requested by user.");
                        }

                        *control_flow = ControlFlow::Exit;
                    }
                },
                None => {
                    eprintln!("Failed retrieving game state from winit backend.");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            };
        });
    }

    /*
    fn poll_events(&mut self) {
        let window_events = &mut self.window_events;
        let input_events = &mut self.input_events;

        self.events_loop.poll_events(|e| {
            match e {
                winit::Event::WindowEvent { window_id: _, event } => {
                    match event {
                        // window events

                        winit::WindowEvent::Resized(new_size) => {
                            window_events.push(Event::new(WindowEvent::Resized(new_size.into())));
                        },
                        winit::WindowEvent::Moved(new_pos) => {
                            window_events.push(Event::new(WindowEvent::Moved(new_pos.into())));
                        },
                        winit::WindowEvent::CloseRequested => {
                            window_events.push(Event::new(WindowEvent::CloseRequested));
                        },
                        winit::WindowEvent::Destroyed => {
                            window_events.push(Event::new(WindowEvent::Destroyed));
                        },
                        winit::WindowEvent::DroppedFile(file_path) => {
                            window_events.push(Event::new(WindowEvent::DroppedFile(file_path)));
                        },
                        winit::WindowEvent::HoveredFile(file_path) => {
                            window_events.push(Event::new(WindowEvent::HoveredFile(file_path)));
                        },
                        winit::WindowEvent::HoveredFileCancelled => {
                            window_events.push(Event::new(WindowEvent::HoveredFileCancelled));
                        },
                        winit::WindowEvent::Focused(received_focus) => {
                            window_events.push(Event::new(WindowEvent::Focused(received_focus)));
                        },
                        winit::WindowEvent::Refresh => {
                            window_events.push(Event::new(WindowEvent::Refresh));
                        },
                        winit::WindowEvent::HiDpiFactorChanged(dpi) => {
                            window_events.push(Event::new(WindowEvent::HiDpiFactorChanged(dpi)));
                        },

                        // input events
                        winit::WindowEvent::KeyboardInput { device_id: _, input } => {
                            input_events.push(Event::new(InputEvent::Keyboard(
                                KeyboardEvent::new(
                                    input.scancode,
                                    match input.state {
                                        winit::ElementState::Pressed => ButtonState::Pressed,
                                        winit::ElementState::Released => ButtonState::Released,
                                    },
                                    match input.virtual_keycode {
                                        Some(winit_keycode) => Some(winit_keycode.into()),
                                        None => None
                                    },
                                    input.modifiers.into()
                                )
                            )));
                        },

                        winit::WindowEvent::ReceivedCharacter(c) => {
                            input_events.push(Event::new(InputEvent::ReceivedChar(c)));
                        },

                        winit::WindowEvent::MouseInput { device_id: _, state, button, modifiers } => {
                            input_events.push(Event::new(InputEvent::MouseButton(
                                MouseButtonEvent::new(
                                    match state {
                                        winit::ElementState::Pressed => ButtonState::Pressed,
                                        winit::ElementState::Released => ButtonState::Released
                                    },
                                    match button {
                                        winit::MouseButton::Left => MouseButton::Left,
                                        winit::MouseButton::Right => MouseButton::Right,
                                        winit::MouseButton::Middle => MouseButton::Middle,
                                        winit::MouseButton::Other(id) => MouseButton::Other(id)
                                    },
                                    modifiers.into()
                                )
                            )));
                        },

                        winit::WindowEvent::MouseWheel { device_id: _, delta, phase, modifiers } => {
                            input_events.push(Event::new(
                                InputEvent::MouseWheel {
                                    delta: match delta {
                                        winit::MouseScrollDelta::LineDelta(x, y) => MouseScrollDelta::Line { horizontal: x, vertical: y },
                                        winit::MouseScrollDelta::PixelDelta(value) => {
                                            MouseScrollDelta::Pixel(value.into())
                                        }
                                    },
                                    phase: phase.into(), 
                                    modifiers: modifiers.into()
                                }
                            ));
                        },

                        winit::WindowEvent::CursorEntered { device_id: _ } => {
                            input_events.push(Event::new(
                                InputEvent::CursorEntered
                            ));
                        },

                        winit::WindowEvent::CursorLeft { device_id: _ } => {
                            input_events.push(Event::new(
                                InputEvent::CursorLeft
                            ));
                        },

                        winit::WindowEvent::CursorMoved { device_id: _, position, modifiers  } => {
                            input_events.push(Event::new(
                                InputEvent::CursorMoved {
                                    position: position.into(),
                                    modifiers: modifiers.into()
                                },
                            ));
                        },

                        winit::WindowEvent::TouchpadPressure { device_id: _, pressure, stage } => {
                            input_events.push(Event::new(
                                InputEvent::TouchpadPressure {
                                    pressure,
                                    stage
                                }
                            ));
                        },

                        winit::WindowEvent::Touch(touch) => {
                            input_events.push(Event::new(
                                InputEvent::Touch(TouchEvent::new(
                                    touch.phase.into(),
                                    touch.location.into(),
                                    touch.id
                                ))
                            ));
                        },

                        winit::WindowEvent::AxisMotion { device_id: _, axis, value } => {
                            input_events.push(Event::new(
                                InputEvent::AxisMotion {
                                    axis, 
                                    value
                                }
                            ));
                        }
                    }
                },
                _ => ()
            }
        });
    }
    */

    /*
    fn redirect_input_events<T, H: InputEventsIndirectHandler<T>>(&mut self, handler: &mut H, listeners: Vec<T>) {
        handler.handle_multiple(listeners, &mut self.input_events);
        self.input_events.clear();
    }
    */
}

/*
impl<T: InputEventListener> InputEventsHandler<T> for WinitBackend {
    fn handle(&mut self, listener: &mut T) {
        for event in self.input_events.iter_mut() {
            listener.handle(event);
        }

        self.input_events.clear();
    }

    fn handle_multiple(&mut self, mut listeners: Vec<T>) {
        for event in self.input_events.iter_mut() {
            for listener in listeners.iter_mut() {
                listener.handle(event);
            }
        }

        self.input_events.clear();
    }
}

impl InputEventsHandler<Box<&mut dyn InputEventListener>> for WinitBackend {
    fn handle(&mut self, listener: &mut Box<&mut dyn InputEventListener>) {
        for event in self.input_events.iter_mut() {
            listener.handle(event);
        }

        self.input_events.clear();
    }

    fn handle_multiple(&mut self, mut listeners: Vec<Box<&mut dyn InputEventListener>>) {
        for event in self.input_events.iter_mut() {
            for listener in listeners.iter_mut() {
                listener.handle(event);
            }
        }

        self.input_events.clear();
    }
}

impl<T: WindowEventListener> WindowEventsHandler<T> for WinitBackend {
    fn handle(&mut self, listener: &mut T) {
        for event in self.window_events.iter_mut() {
            listener.handle(event);
        }

        self.window_events.clear();
    }

    fn handle_multiple(&mut self, mut listeners: Vec<T>) {
        for event in self.window_events.iter_mut() {
            for listener in listeners.iter_mut() {
                listener.handle(event);
            }
        }

        self.window_events.clear();
    }
}

impl WindowEventsHandler<Box<&mut dyn WindowEventListener>> for WinitBackend {
    fn handle(&mut self, listener: &mut Box<&mut dyn WindowEventListener>) {
        for event in self.window_events.iter_mut() {
            listener.handle(event);
        }

        self.window_events.clear();
    }

    fn handle_multiple(&mut self, mut listeners: Vec<Box<&mut dyn WindowEventListener>>) {
        for event in self.window_events.iter_mut() {
            for listener in listeners.iter_mut() {
                listener.handle(event);
            }
        }

        self.window_events.clear();
    }
}
*/

impl WinitBackend {
    pub fn new<T: Into<String>>(window_title: T, size: Size<u32>) -> Result<Self, OsError> {
        let event_loop = EventLoop::new();

        WindowBuilder::new()
          .with_title(&window_title.into())
          .with_inner_size(winit::dpi::Size::Logical(size.into()))
          .build(&event_loop)
          .map(|winit_window| {
              Self {
                  event_loop: Some(event_loop),
                  winit_window
                  //window_events: Vec::new(),
                  //input_events: Vec::new()
              }
          })
    }
}

impl From<VirtualKeyCode> for KeyCode {
    fn from(key: VirtualKeyCode) -> KeyCode {
        match key {
            VirtualKeyCode::Key1                 => KeyCode::D1,
            VirtualKeyCode::Key2                 => KeyCode::D2,
            VirtualKeyCode::Key3                 => KeyCode::D3,
            VirtualKeyCode::Key4                 => KeyCode::D4,
            VirtualKeyCode::Key5                 => KeyCode::D5,
            VirtualKeyCode::Key6                 => KeyCode::D6,
            VirtualKeyCode::Key7                 => KeyCode::D7,
            VirtualKeyCode::Key8                 => KeyCode::D8,
            VirtualKeyCode::Key9                 => KeyCode::D9,
            VirtualKeyCode::Key0                 => KeyCode::D0,
            VirtualKeyCode::A                    => KeyCode::A,
            VirtualKeyCode::B                    => KeyCode::B,
            VirtualKeyCode::C                    => KeyCode::C,
            VirtualKeyCode::D                    => KeyCode::D,
            VirtualKeyCode::E                    => KeyCode::E,
            VirtualKeyCode::F                    => KeyCode::F,
            VirtualKeyCode::G                    => KeyCode::G,
            VirtualKeyCode::H                    => KeyCode::H,
            VirtualKeyCode::I                    => KeyCode::I,
            VirtualKeyCode::J                    => KeyCode::J,
            VirtualKeyCode::K                    => KeyCode::K,
            VirtualKeyCode::L                    => KeyCode::L,
            VirtualKeyCode::M                    => KeyCode::M,
            VirtualKeyCode::N                    => KeyCode::N,
            VirtualKeyCode::O                    => KeyCode::O,
            VirtualKeyCode::P                    => KeyCode::P,
            VirtualKeyCode::Q                    => KeyCode::Q,
            VirtualKeyCode::R                    => KeyCode::R,
            VirtualKeyCode::S                    => KeyCode::S,
            VirtualKeyCode::T                    => KeyCode::T,
            VirtualKeyCode::U                    => KeyCode::U,
            VirtualKeyCode::V                    => KeyCode::V,
            VirtualKeyCode::W                    => KeyCode::W,
            VirtualKeyCode::X                    => KeyCode::X,
            VirtualKeyCode::Y                    => KeyCode::Y,
            VirtualKeyCode::Z                    => KeyCode::Z,
            VirtualKeyCode::Escape               => KeyCode::Escape,
            VirtualKeyCode::F1                   => KeyCode::F1,
            VirtualKeyCode::F2                   => KeyCode::F2,
            VirtualKeyCode::F3                   => KeyCode::F3,
            VirtualKeyCode::F4                   => KeyCode::F4,
            VirtualKeyCode::F5                   => KeyCode::F5,
            VirtualKeyCode::F6                   => KeyCode::F6,
            VirtualKeyCode::F7                   => KeyCode::F7,
            VirtualKeyCode::F8                   => KeyCode::F8,
            VirtualKeyCode::F9                   => KeyCode::F9,
            VirtualKeyCode::F10                  => KeyCode::F10,
            VirtualKeyCode::F11                  => KeyCode::F11,
            VirtualKeyCode::F12                  => KeyCode::F12,
            VirtualKeyCode::F13                  => KeyCode::F13,
            VirtualKeyCode::F14                  => KeyCode::F14,
            VirtualKeyCode::F15                  => KeyCode::F15,
            VirtualKeyCode::F16                  => KeyCode::F16,
            VirtualKeyCode::F17                  => KeyCode::F17,
            VirtualKeyCode::F18                  => KeyCode::F18,
            VirtualKeyCode::F19                  => KeyCode::F19,
            VirtualKeyCode::F20                  => KeyCode::F20,
            VirtualKeyCode::F21                  => KeyCode::F21,
            VirtualKeyCode::F22                  => KeyCode::F22,
            VirtualKeyCode::F23                  => KeyCode::F23,
            VirtualKeyCode::F24                  => KeyCode::F24,
            VirtualKeyCode::Snapshot             => KeyCode::PrintScreen,
            VirtualKeyCode::Scroll               => KeyCode::ScrollLock,
            VirtualKeyCode::Pause                => KeyCode::PauseBreak,
            VirtualKeyCode::Insert               => KeyCode::Insert,
            VirtualKeyCode::Home                 => KeyCode::Home,
            VirtualKeyCode::Delete               => KeyCode::Delete,
            VirtualKeyCode::End                  => KeyCode::End,
            VirtualKeyCode::PageDown             => KeyCode::PageDown,
            VirtualKeyCode::PageUp               => KeyCode::PageUp,
            VirtualKeyCode::Left                 => KeyCode::Left,
            VirtualKeyCode::Up                   => KeyCode::Up,
            VirtualKeyCode::Right                => KeyCode::Right,
            VirtualKeyCode::Down                 => KeyCode::Down,
            VirtualKeyCode::Back                 => KeyCode::Backspace,
            VirtualKeyCode::Return               => KeyCode::Return,
            VirtualKeyCode::Space                => KeyCode::Space,
            VirtualKeyCode::Compose              => KeyCode::Compose,
            VirtualKeyCode::Caret                => KeyCode::Caret,
            VirtualKeyCode::Numlock              => KeyCode::Numlock,
            VirtualKeyCode::Numpad0              => KeyCode::Numpad0,
            VirtualKeyCode::Numpad1              => KeyCode::Numpad1,
            VirtualKeyCode::Numpad2              => KeyCode::Numpad2,
            VirtualKeyCode::Numpad3              => KeyCode::Numpad3,
            VirtualKeyCode::Numpad4              => KeyCode::Numpad4,
            VirtualKeyCode::Numpad5              => KeyCode::Numpad5,
            VirtualKeyCode::Numpad6              => KeyCode::Numpad6,
            VirtualKeyCode::Numpad7              => KeyCode::Numpad7,
            VirtualKeyCode::Numpad8              => KeyCode::Numpad8,
            VirtualKeyCode::Numpad9              => KeyCode::Numpad9,
            VirtualKeyCode::AbntC1               => KeyCode::AbntC1,
            VirtualKeyCode::AbntC2               => KeyCode::AbntC2,
            VirtualKeyCode::Add                  => KeyCode::Add,
            VirtualKeyCode::Apostrophe           => KeyCode::Apostrophe,
            VirtualKeyCode::Apps                 => KeyCode::Apps,
            VirtualKeyCode::At                   => KeyCode::At,
            VirtualKeyCode::Ax                   => KeyCode::Ax,
            VirtualKeyCode::Backslash            => KeyCode::Backslash,
            VirtualKeyCode::Calculator           => KeyCode::Calculator,
            VirtualKeyCode::Capital              => KeyCode::Capital,
            VirtualKeyCode::Colon                => KeyCode::Colon,
            VirtualKeyCode::Comma                => KeyCode::Comma,
            VirtualKeyCode::Convert              => KeyCode::Convert,
            VirtualKeyCode::Decimal              => KeyCode::Decimal,
            VirtualKeyCode::Divide               => KeyCode::Divide,
            VirtualKeyCode::Equals               => KeyCode::Equals,
            VirtualKeyCode::Grave                => KeyCode::Grave,
            VirtualKeyCode::Kana                 => KeyCode::Kana,
            VirtualKeyCode::Kanji                => KeyCode::Kanji,
            VirtualKeyCode::LAlt                 => KeyCode::LeftAlt,
            VirtualKeyCode::LBracket             => KeyCode::LeftBracket,
            VirtualKeyCode::LControl             => KeyCode::LeftControl,
            VirtualKeyCode::LShift               => KeyCode::LeftShift,
            VirtualKeyCode::LWin                 => KeyCode::LeftCommand,
            VirtualKeyCode::Mail                 => KeyCode::Mail,
            VirtualKeyCode::MediaSelect          => KeyCode::MediaSelect,
            VirtualKeyCode::MediaStop            => KeyCode::MediaStop,
            VirtualKeyCode::Minus                => KeyCode::Minus,
            VirtualKeyCode::Multiply             => KeyCode::Multiply,
            VirtualKeyCode::Mute                 => KeyCode::Mute,
            VirtualKeyCode::MyComputer           => KeyCode::MyComputer,
            VirtualKeyCode::NavigateForward      => KeyCode::NavigateForward,
            VirtualKeyCode::NavigateBackward     => KeyCode::NavigateBackward, 
            VirtualKeyCode::NextTrack            => KeyCode::NextTrack,
            VirtualKeyCode::NoConvert            => KeyCode::NoConvert,
            VirtualKeyCode::NumpadComma          => KeyCode::NumpadComma,
            VirtualKeyCode::NumpadEnter          => KeyCode::NumpadEnter,
            VirtualKeyCode::NumpadEquals         => KeyCode::NumpadEquals,
            VirtualKeyCode::OEM102               => KeyCode::OEM102,
            VirtualKeyCode::Period               => KeyCode::Period,
            VirtualKeyCode::PlayPause            => KeyCode::PlayPause,
            VirtualKeyCode::Power                => KeyCode::Power,
            VirtualKeyCode::PrevTrack            => KeyCode::PrevTrack,
            VirtualKeyCode::RAlt                 => KeyCode::RightAlt,
            VirtualKeyCode::RBracket             => KeyCode::RightBracket,
            VirtualKeyCode::RControl             => KeyCode::RightControl,
            VirtualKeyCode::RShift               => KeyCode::RightShift,
            VirtualKeyCode::RWin                 => KeyCode::RightCommand,
            VirtualKeyCode::Semicolon            => KeyCode::Semicolon,
            VirtualKeyCode::Slash                => KeyCode::Slash,
            VirtualKeyCode::Sleep                => KeyCode::Sleep,
            VirtualKeyCode::Stop                 => KeyCode::Stop,
            VirtualKeyCode::Subtract             => KeyCode::Subtract,
            VirtualKeyCode::Sysrq                => KeyCode::Sysrq,
            VirtualKeyCode::Tab                  => KeyCode::Tab,
            VirtualKeyCode::Underline            => KeyCode::Underline,
            VirtualKeyCode::Unlabeled            => KeyCode::Unlabeled,
            VirtualKeyCode::VolumeDown           => KeyCode::VolumeDown,
            VirtualKeyCode::VolumeUp             => KeyCode::VolumeUp,
            VirtualKeyCode::Wake                 => KeyCode::Wake,
            VirtualKeyCode::WebBack              => KeyCode::WebBack,
            VirtualKeyCode::WebFavorites         => KeyCode::WebFavorites,
            VirtualKeyCode::WebForward           => KeyCode::WebForward,
            VirtualKeyCode::WebHome              => KeyCode::WebHome,
            VirtualKeyCode::WebRefresh           => KeyCode::WebRefresh,
            VirtualKeyCode::WebSearch            => KeyCode::WebSearch,
            VirtualKeyCode::WebStop              => KeyCode::WebStop,
            VirtualKeyCode::Yen                  => KeyCode::Yen,
            VirtualKeyCode::Copy                 => KeyCode::Copy,
            VirtualKeyCode::Paste                => KeyCode::Paste,
            VirtualKeyCode::Cut                  => KeyCode::Cut,
        }
    }
}

/*
impl From<winit::ModifiersState> for KeyModifiers {
    fn from(modifiers: winit::ModifiersState) -> KeyModifiers {
        KeyModifiers {
            ctrl:    modifiers.ctrl,
            shift:   modifiers.shift,
            alt:     modifiers.alt,
            command: modifiers.logo
        }
    }
}
*/

/*
impl From<LogicalPosition<i32>> for Vector2<i32> {
    fn from(logical_pos: LogicalPosition<i32>) -> Vector2<i32> {
        let decomposed_position: (i32, i32) = logical_pos.into();
        Vector2::from(decomposed_position)
    }
}
*/

impl From<LogicalPosition<f64>> for Vector2<i32> {
    fn from(logical_pos: LogicalPosition<f64>) -> Vector2<i32> {
        let decomposed_position: (i32, i32) = logical_pos.into();
        Vector2::from(decomposed_position)
    }
}

impl From<LogicalSize<f64>> for Size<u32> {
    fn from(logical_size: LogicalSize<f64>) -> Size<u32> {
        let decomposed_size: (u32, u32) = logical_size.into();
        Size::from(decomposed_size)
    }
}

impl From<Size<u32>> for LogicalSize<f64> {
    fn from(size: Size<u32>) -> LogicalSize<f64> {
        LogicalSize {
            width: size.width().into(),
            height: size.height().into(),
        }
    }
}

/*
impl From<winit::TouchPhase> for TouchPhase {
    fn from(phase: winit::TouchPhase) -> TouchPhase {
        match phase {
            winit::TouchPhase::Started   => TouchPhase::Started,
            winit::TouchPhase::Moved     => TouchPhase::Moved,
            winit::TouchPhase::Ended     => TouchPhase::Ended,
            winit::TouchPhase::Cancelled => TouchPhase::Cancelled
        }
    }
}
*/
