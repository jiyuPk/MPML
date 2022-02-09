#![cfg(target_os = "windows")]
pub use self::{
    window::Window,
    window::Property
};

#[macro_use]
mod window;