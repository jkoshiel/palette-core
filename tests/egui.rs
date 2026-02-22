#![cfg(feature = "egui")]

use egui::Color32;

use palette_core::color::Color;
use palette_core::egui::{to_color32, to_egui_visuals};
use palette_core::manifest::PaletteManifest;
use palette_core::palette::Palette;

fn load_preset(name: &str) -> PaletteManifest {
    let path = format!("presets/{name}.toml");
    let content = std::fs::read_to_string(&path).unwrap();
    PaletteManifest::from_toml(&content).unwrap()
}

fn tokyonight_visuals() -> ::egui::Visuals {
    let manifest = load_preset("tokyonight");
    let palette = Palette::from_manifest(&manifest).unwrap();
    to_egui_visuals(&palette)
}

#[test]
fn single_color_converts_to_color32() {
    let color = Color { r: 26, g: 27, b: 42 };
    assert_eq!(to_color32(&color), Color32::from_rgb(26, 27, 42));
}

#[test]
fn panel_fill_matches_background() {
    let v = tokyonight_visuals();
    assert_eq!(v.panel_fill, Color32::from_rgb(26, 27, 42));
}

#[test]
fn window_fill_matches_background() {
    let v = tokyonight_visuals();
    assert_eq!(v.window_fill, v.panel_fill);
}

#[test]
fn error_fg_maps_semantic_error() {
    let v = tokyonight_visuals();
    assert_eq!(v.error_fg_color, Color32::from_rgb(219, 75, 75));
}

#[test]
fn selection_bg_maps_surface_selection() {
    let v = tokyonight_visuals();
    assert_eq!(v.selection.bg_fill, Color32::from_rgb(40, 52, 87));
}
