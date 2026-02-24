use std::fmt::Write;

use crate::color::Color;
use crate::palette::Palette;

fn write_section<'a>(
    out: &mut String,
    prefix: &str,
    section: &str,
    slots: impl Iterator<Item = (&'static str, &'a Color)>,
) {
    for (slot, color) in slots {
        let slot_css = slot.replace('_', "-");
        // String::write_fmt is infallible
        let _ = writeln!(
            out,
            "  --{prefix}-{section}-{slot_css}: {color};",
        );
    }
}

impl Palette {
    pub fn to_css(&self, prefix: &str) -> String {
        to_css_custom_properties(self, prefix)
    }
}

pub fn to_css_custom_properties(palette: &Palette, prefix: &str) -> String {
    let mut out = String::new();
    write_section(&mut out, prefix, "base", palette.base.populated_slots());
    write_section(&mut out, prefix, "semantic", palette.semantic.populated_slots());
    write_section(&mut out, prefix, "diff", palette.diff.populated_slots());
    write_section(&mut out, prefix, "surface", palette.surface.populated_slots());
    write_section(&mut out, prefix, "typography", palette.typography.populated_slots());
    write_section(&mut out, prefix, "syntax", palette.syntax.populated_slots());
    write_section(&mut out, prefix, "editor", palette.editor.populated_slots());
    write_section(&mut out, prefix, "terminal", palette.terminal_ansi.populated_slots());
    out
}
