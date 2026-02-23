use std::path::Path;
use std::sync::Arc;

use crate::error::PaletteError;
use crate::manifest::PaletteManifest;
use crate::merge::merge_manifests;
use crate::palette::Palette;

macro_rules! presets {
    ($($id:literal => $file:literal),+ $(,)?) => {
        pub(crate) fn preset_toml(id: &str) -> Option<&'static str> {
            match id {
                $($id => Some(include_str!($file)),)+
                _ => None,
            }
        }

        pub fn preset_ids() -> &'static [&'static str] {
            &[$($id),+]
        }
    };
}

presets! {
    "ayu_dark"              => "../presets/ayu_dark.toml",
    "ayu_light"             => "../presets/ayu_light.toml",
    "ayu_mirage"            => "../presets/ayu_mirage.toml",
    "catppuccin"            => "../presets/catppuccin.toml",
    "catppuccin_frappe"     => "../presets/catppuccin_frappe.toml",
    "catppuccin_latte"      => "../presets/catppuccin_latte.toml",
    "catppuccin_macchiato"  => "../presets/catppuccin_macchiato.toml",
    "dracula"               => "../presets/dracula.toml",
    "everforest_dark"       => "../presets/everforest_dark.toml",
    "everforest_light"      => "../presets/everforest_light.toml",
    "github_dark"           => "../presets/github_dark.toml",
    "github_light"          => "../presets/github_light.toml",
    "gruvbox_dark"          => "../presets/gruvbox_dark.toml",
    "gruvbox_light"         => "../presets/gruvbox_light.toml",
    "kanagawa"              => "../presets/kanagawa.toml",
    "monokai"               => "../presets/monokai.toml",
    "nord"                  => "../presets/nord.toml",
    "one_dark"              => "../presets/one_dark.toml",
    "one_light"             => "../presets/one_light.toml",
    "rose_pine"             => "../presets/rose_pine.toml",
    "rose_pine_dawn"        => "../presets/rose_pine_dawn.toml",
    "rose_pine_moon"        => "../presets/rose_pine_moon.toml",
    "solarized_dark"        => "../presets/solarized_dark.toml",
    "solarized_light"       => "../presets/solarized_light.toml",
    "tokyonight"            => "../presets/tokyonight.toml",
    "tokyonight_day"        => "../presets/tokyonight_day.toml",
    "tokyonight_moon"       => "../presets/tokyonight_moon.toml",
    "tokyonight_storm"      => "../presets/tokyonight_storm.toml",
}

pub fn load_preset_file(path: &Path) -> Result<Palette, PaletteError> {
    let path_str: Arc<str> = Arc::from(path.to_string_lossy().as_ref());
    let toml = std::fs::read_to_string(path).map_err(|source| PaletteError::Io {
        path: Arc::clone(&path_str),
        source,
    })?;
    let manifest = PaletteManifest::from_toml(&toml)?;

    let resolved = match manifest.inherits_from() {
        None => manifest,
        Some(parent_id) => {
            let parent = resolve_parent(path, parent_id)?;
            merge_manifests(&manifest, &parent)
        }
    };

    Palette::from_manifest(&resolved)
}

fn resolve_parent(child_path: &Path, parent_id: &str) -> Result<PaletteManifest, PaletteError> {
    let sibling = child_path
        .parent()
        .map(|dir| dir.join(format!("{parent_id}.toml")))
        .filter(|p| p.is_file());

    match (sibling, preset_toml(parent_id)) {
        (Some(path), _) => {
            let path_str: Arc<str> = Arc::from(path.to_string_lossy().as_ref());
            let toml = std::fs::read_to_string(&path).map_err(|source| PaletteError::Io {
                path: path_str,
                source,
            })?;
            PaletteManifest::from_toml(&toml)
        }
        (None, Some(embedded)) => PaletteManifest::from_toml(embedded),
        (None, None) => Err(PaletteError::UnknownPreset(Arc::from(parent_id))),
    }
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
