#![allow(clippy::manual_clamp)]

#[cfg(windows)]
extern crate windows_sys as windows;

pub mod about;
pub mod alert;
pub mod app;
pub mod button;
pub mod completion;
pub mod diff;
pub mod dropdown;
pub mod editor;
pub mod explorer;
pub mod find;
pub mod hover;
pub mod ime;
pub mod keymap;
pub mod list;
mod logging;
pub mod message;
pub mod palette;
pub mod panel;
pub mod picker;
pub mod plugin;
pub mod problem;
pub mod scroll;
pub mod search;
pub mod settings;
pub mod signature;
pub mod source_control;
pub mod split;
pub mod status;
pub mod svg;
mod tab;
pub mod terminal;
pub mod title;
pub mod window;
