use crate::color::Color;
use crate::palette::Palette;

pub fn to_color32(color: &Color) -> ::egui::Color32 {
    ::egui::Color32::from_rgb(color.r, color.g, color.b)
}

pub fn to_egui_visuals(palette: &Palette) -> ::egui::Visuals {
    let mut v = ::egui::Visuals::dark();

    if let Some(bg) = &palette.base.background {
        let c = to_color32(bg);
        v.panel_fill = c;
        v.window_fill = c;
        v.faint_bg_color = c;
        v.extreme_bg_color = c;
    }
    if let Some(fg) = &palette.base.foreground {
        v.override_text_color = Some(to_color32(fg));
    }

    if let Some(err) = &palette.semantic.error {
        v.error_fg_color = to_color32(err);
    }
    if let Some(warn) = &palette.semantic.warning {
        v.warn_fg_color = to_color32(warn);
    }
    if let Some(info) = &palette.semantic.info {
        v.hyperlink_color = to_color32(info);
    }

    if let Some(sel) = &palette.surface.selection {
        v.selection.bg_fill = to_color32(sel);
    }
    if let Some(hl) = &palette.surface.highlight {
        let c = to_color32(hl);
        v.widgets.hovered.bg_fill = c;
        v.widgets.active.bg_fill = c;
    }

    v
}
