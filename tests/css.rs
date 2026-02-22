use std::collections::BTreeMap;
use std::sync::Arc;

use palette_core::css::to_css_custom_properties;
use palette_core::manifest::PaletteManifest;
use palette_core::palette::Palette;

fn load_preset(name: &str) -> PaletteManifest {
    let path = format!("presets/{name}.toml");
    let content = std::fs::read_to_string(&path).unwrap();
    PaletteManifest::from_toml(&content).unwrap()
}

#[test]
fn contains_expected_variable_format() {
    let manifest = load_preset("tokyonight");
    let palette = Palette::from_manifest(&manifest).unwrap();
    let css = to_css_custom_properties(&palette, "mx");

    assert!(
        css.contains("--mx-base-background: #"),
        "expected CSS variable pattern, got:\n{css}",
    );
}

#[test]
fn all_populated_slots_present() {
    let manifest = load_preset("tokyonight");
    let palette = Palette::from_manifest(&manifest).unwrap();
    let css = to_css_custom_properties(&palette, "mx");

    let populated_count = palette.base.populated_slots().count()
        + palette.semantic.populated_slots().count()
        + palette.diff.populated_slots().count()
        + palette.surface.populated_slots().count()
        + palette.typography.populated_slots().count()
        + palette.syntax.populated_slots().count()
        + palette.editor.populated_slots().count()
        + palette.terminal_ansi.populated_slots().count();

    let css_line_count = css.lines().filter(|l| l.contains("--mx-")).count();
    assert_eq!(css_line_count, populated_count);
}

#[test]
fn none_slots_absent() {
    let manifest = PaletteManifest {
        meta: None,
        base: BTreeMap::from([(Arc::from("background"), Arc::from("#000000"))]),
        semantic: BTreeMap::new(),
        diff: BTreeMap::new(),
        surface: BTreeMap::new(),
        typography: BTreeMap::new(),
        syntax: BTreeMap::new(),
        editor: BTreeMap::new(),
        terminal: BTreeMap::new(),
    };
    let palette = Palette::from_manifest(&manifest).unwrap();
    let css = to_css_custom_properties(&palette, "mx");

    assert!(css.contains("--mx-base-background:"));
    assert!(!css.contains("foreground"));
    assert!(!css.contains("--mx-semantic-"));
    assert!(!css.contains("--mx-terminal-"));
}

#[test]
fn underscore_to_hyphen_conversion() {
    let manifest = PaletteManifest {
        meta: None,
        base: BTreeMap::from([(Arc::from("background_dark"), Arc::from("#111111"))]),
        semantic: BTreeMap::new(),
        diff: BTreeMap::new(),
        surface: BTreeMap::new(),
        typography: BTreeMap::new(),
        syntax: BTreeMap::new(),
        editor: BTreeMap::new(),
        terminal: BTreeMap::new(),
    };
    let palette = Palette::from_manifest(&manifest).unwrap();
    let css = to_css_custom_properties(&palette, "mx");

    assert!(
        css.contains("--mx-base-background-dark:"),
        "underscores should become hyphens, got:\n{css}",
    );
    assert!(!css.contains("background_dark"), "raw underscores should not appear in CSS output");
}
