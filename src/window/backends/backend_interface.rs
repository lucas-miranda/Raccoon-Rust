use crate::{
    core::{
        GameLoopInterface
    },
    window::backends::{
        BackendWindow,
        BackendEventLoop
    }
};

pub trait BackendInterface<L: GameLoopInterface> {
    type Window: BackendWindow;
    type EventLoop: BackendEventLoop<L>;

    fn window(&self) -> &Self::Window;
    fn window_mut(&mut self) -> &mut Self::Window;
    fn event_loop(&mut self) -> Self::EventLoop;
}
