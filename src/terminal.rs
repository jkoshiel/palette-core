use std::collections::BTreeMap;

use ratatui::style::Color as RatatuiColor;

use crate::color::Color;
use crate::palette::Palette;

pub fn to_ratatui_color(color: &Color) -> RatatuiColor {
    RatatuiColor::Rgb(color.r, color.g, color.b)
}

#[derive(Debug, Clone)]
pub struct TerminalTheme {
    pub base: BTreeMap<&'static str, RatatuiColor>,
    pub semantic: BTreeMap<&'static str, RatatuiColor>,
    pub diff: BTreeMap<&'static str, RatatuiColor>,
    pub surface: BTreeMap<&'static str, RatatuiColor>,
    pub typography: BTreeMap<&'static str, RatatuiColor>,
    pub syntax: BTreeMap<&'static str, RatatuiColor>,
    pub editor: BTreeMap<&'static str, RatatuiColor>,
    pub terminal_ansi: BTreeMap<&'static str, RatatuiColor>,
}

fn convert_section<'a>(
    slots: impl Iterator<Item = (&'static str, &'a Color)>,
) -> BTreeMap<&'static str, RatatuiColor> {
    slots.map(|(name, c)| (name, to_ratatui_color(c))).collect()
}

pub fn to_terminal_theme(palette: &Palette) -> TerminalTheme {
    TerminalTheme {
        base: convert_section(palette.base.populated_slots()),
        semantic: convert_section(palette.semantic.populated_slots()),
        diff: convert_section(palette.diff.populated_slots()),
        surface: convert_section(palette.surface.populated_slots()),
        typography: convert_section(palette.typography.populated_slots()),
        syntax: convert_section(palette.syntax.populated_slots()),
        editor: convert_section(palette.editor.populated_slots()),
        terminal_ansi: convert_section(palette.terminal_ansi.populated_slots()),
    }
}
