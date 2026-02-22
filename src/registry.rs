use std::sync::Arc;

use crate::error::PaletteError;
use crate::manifest::PaletteManifest;
use crate::merge::merge_manifests;
use crate::palette::Palette;

fn preset_toml(id: &str) -> Option<&'static str> {
    match id {
        "catppuccin" => Some(include_str!("../presets/catppuccin.toml")),
        "catppuccin_frappe" => Some(include_str!("../presets/catppuccin_frappe.toml")),
        "catppuccin_latte" => Some(include_str!("../presets/catppuccin_latte.toml")),
        "catppuccin_macchiato" => Some(include_str!("../presets/catppuccin_macchiato.toml")),
        "github_dark" => Some(include_str!("../presets/github_dark.toml")),
        "github_light" => Some(include_str!("../presets/github_light.toml")),
        "tokyonight" => Some(include_str!("../presets/tokyonight.toml")),
        "tokyonight_day" => Some(include_str!("../presets/tokyonight_day.toml")),
        "tokyonight_moon" => Some(include_str!("../presets/tokyonight_moon.toml")),
        "tokyonight_storm" => Some(include_str!("../presets/tokyonight_storm.toml")),
        _ => None,
    }
}

pub fn preset_ids() -> &'static [&'static str] {
    &[
        "catppuccin",
        "catppuccin_frappe",
        "catppuccin_latte",
        "catppuccin_macchiato",
        "github_dark",
        "github_light",
        "tokyonight",
        "tokyonight_day",
        "tokyonight_moon",
        "tokyonight_storm",
    ]
}

pub fn load_preset(id: &str) -> Result<Palette, PaletteError> {
    let toml = preset_toml(id).ok_or_else(|| PaletteError::UnknownPreset(Arc::from(id)))?;
    let manifest = PaletteManifest::from_toml(toml)?;

    let resolved = match manifest.inherits_from() {
        None => manifest,
        Some(parent_id) => {
            let parent_toml = preset_toml(parent_id)
                .ok_or_else(|| PaletteError::UnknownPreset(Arc::from(parent_id)))?;
            let parent = PaletteManifest::from_toml(parent_toml)?;
            merge_manifests(&manifest, &parent)
        }
    };

    Palette::from_manifest(&resolved)
}
