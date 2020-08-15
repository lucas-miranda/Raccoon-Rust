use std::path::PathBuf;

use crate::{
    math::{
        Size,
        Vector2
    }
};

#[derive(PartialEq, Debug)]
pub enum WindowEvent {
    Resized(Size<u32>),
    Moved(Vector2<i32>),
    CloseRequested,
    Destroyed,
    DroppedFile(PathBuf),
    HoveredFile(PathBuf),
    HoveredFileCancelled,
    Focused(bool),
    Refresh,
    HiDpiFactorChanged(f64)
}
