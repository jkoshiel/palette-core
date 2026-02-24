use ratatui::style::Color as RatatuiColor;

use crate::color::Color;
use crate::palette::Palette;

pub fn to_ratatui_color(color: &Color) -> RatatuiColor {
    RatatuiColor::Rgb(color.r, color.g, color.b)
}

macro_rules! terminal_group {
    ($color_type:ident { $($field:ident),+ $(,)? }) => {
        paste::paste! {
            #[derive(Debug, Clone)]
            pub struct [<Terminal $color_type>] {
                $(pub $field: Option<RatatuiColor>,)+
            }

            impl [<Terminal $color_type>] {
                fn from_palette(group: &crate::palette::$color_type) -> Self {
                    Self {
                        $($field: group.$field.map(|c| to_ratatui_color(&c)),)+
                    }
                }
            }
        }
    };
}

crate::palette::color_fields!(terminal_group);

#[derive(Debug, Clone)]
pub struct TerminalTheme {
    pub base: TerminalBaseColors,
    pub semantic: TerminalSemanticColors,
    pub diff: TerminalDiffColors,
    pub surface: TerminalSurfaceColors,
    pub typography: TerminalTypographyColors,
    pub syntax: TerminalSyntaxColors,
    pub editor: TerminalEditorColors,
    pub terminal_ansi: TerminalTerminalAnsiColors,
}

pub fn to_terminal_theme(palette: &Palette) -> TerminalTheme {
    TerminalTheme {
        base: TerminalBaseColors::from_palette(&palette.base),
        semantic: TerminalSemanticColors::from_palette(&palette.semantic),
        diff: TerminalDiffColors::from_palette(&palette.diff),
        surface: TerminalSurfaceColors::from_palette(&palette.surface),
        typography: TerminalTypographyColors::from_palette(&palette.typography),
        syntax: TerminalSyntaxColors::from_palette(&palette.syntax),
        editor: TerminalEditorColors::from_palette(&palette.editor),
        terminal_ansi: TerminalTerminalAnsiColors::from_palette(&palette.terminal_ansi),
    }
}
