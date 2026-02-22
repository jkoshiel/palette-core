//! TOML-defined theme system with inheritance and multi-target export.

pub mod color;
pub mod error;
pub mod manifest;
pub mod merge;
pub mod palette;
pub mod registry;

pub mod css;

#[cfg(feature = "terminal")]
pub mod terminal;

#[cfg(feature = "egui")]
pub mod egui;
