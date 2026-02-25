//! TOML-defined theme system with inheritance and multi-target export.

pub mod color;
pub mod error;
pub mod manifest;
pub mod merge;
pub mod palette;
pub mod registry;

pub mod contrast;
pub mod css;
pub mod manipulation;

pub use color::Color;
pub use contrast::ContrastLevel;
pub use error::PaletteError;
pub use palette::{Palette, PaletteMeta};
pub use registry::{load_preset, load_preset_file, preset, preset_ids, Registry, ThemeInfo};

#[cfg(feature = "terminal")]
pub mod terminal;

#[cfg(feature = "platform")]
pub mod platform;

#[cfg(feature = "snapshot")]
pub mod snapshot;

#[cfg(feature = "egui")]
pub mod egui;

#[cfg(feature = "wasm")]
pub mod wasm;
