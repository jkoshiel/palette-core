use crate::color::Color;
use crate::palette::Palette;

pub fn to_color32(color: &Color) -> ::egui::Color32 {
    ::egui::Color32::from_rgb(color.r, color.g, color.b)
}

macro_rules! apply_color {
    ($field:expr => $($target:expr),+) => {
        match $field {
            Some(c) => {
                let color = to_color32(c);
                $($target = color;)+
            }
            None => {}
        }
    };
}

pub fn to_egui_visuals(palette: &Palette) -> ::egui::Visuals {
    let mut v = ::egui::Visuals::dark();

    apply_color!(&palette.base.background =>
        v.panel_fill, v.window_fill, v.faint_bg_color, v.extreme_bg_color);

    if let Some(fg) = &palette.base.foreground {
        v.override_text_color = Some(to_color32(fg));
    }

    apply_color!(&palette.semantic.error => v.error_fg_color);
    apply_color!(&palette.semantic.warning => v.warn_fg_color);
    apply_color!(&palette.semantic.info => v.hyperlink_color);

    apply_color!(&palette.surface.selection => v.selection.bg_fill);
    apply_color!(&palette.surface.highlight =>
        v.widgets.hovered.bg_fill, v.widgets.active.bg_fill);

    v
}
