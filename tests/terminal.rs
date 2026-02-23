#![cfg(feature = "terminal")]

use std::collections::BTreeMap;
use std::sync::Arc;

use ratatui::style::Color as RatatuiColor;

use palette_core::color::Color;
use palette_core::palette::Palette;
use palette_core::terminal::{to_ratatui_color, to_terminal_theme};

mod common;

#[test]
fn single_color_converts_rgb() {
    let color = Color { r: 26, g: 27, b: 42 };
    assert_eq!(to_ratatui_color(&color), RatatuiColor::Rgb(26, 27, 42));
}

#[test]
fn all_populated_slots_present() {
    let manifest = common::load_preset("tokyonight");
    let palette = Palette::from_manifest(&manifest).unwrap();
    let theme = to_terminal_theme(&palette);

    let expected = palette.base.populated_slots().count()
        + palette.semantic.populated_slots().count()
        + palette.diff.populated_slots().count()
        + palette.surface.populated_slots().count()
        + palette.typography.populated_slots().count()
        + palette.syntax.populated_slots().count()
        + palette.editor.populated_slots().count()
        + palette.terminal_ansi.populated_slots().count();

    let actual = theme.base.len()
        + theme.semantic.len()
        + theme.diff.len()
        + theme.surface.len()
        + theme.typography.len()
        + theme.syntax.len()
        + theme.editor.len()
        + theme.terminal_ansi.len();

    assert_eq!(actual, expected);
}

#[test]
fn rgb_values_match_source() {
    let manifest = common::load_preset("tokyonight");
    let palette = Palette::from_manifest(&manifest).unwrap();
    let theme = to_terminal_theme(&palette);

    assert_eq!(
        theme.base["background"],
        RatatuiColor::Rgb(26, 27, 42),
    );
}

#[test]
fn empty_sections_produce_empty_maps() {
    let manifest = common::manifest_with_base(
        BTreeMap::from([(Arc::from("background"), Arc::from("#000000"))]),
    );
    let palette = Palette::from_manifest(&manifest).unwrap();
    let theme = to_terminal_theme(&palette);

    assert_eq!(theme.base.len(), 1);
    assert!(theme.semantic.is_empty());
    assert!(theme.diff.is_empty());
    assert!(theme.surface.is_empty());
    assert!(theme.typography.is_empty());
    assert!(theme.syntax.is_empty());
    assert!(theme.editor.is_empty());
    assert!(theme.terminal_ansi.is_empty());
}

#[test]
fn terminal_ansi_maps_all_16_colors() {
    let manifest = common::load_preset("tokyonight");
    let palette = Palette::from_manifest(&manifest).unwrap();
    let theme = to_terminal_theme(&palette);

    assert_eq!(theme.terminal_ansi.len(), 16);
    assert_eq!(
        theme.terminal_ansi["black"],
        RatatuiColor::Rgb(21, 22, 30),
    );
}
