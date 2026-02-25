# Guide

This guide covers loading presets, theme switching, custom presets, and end-user theming. For quick-start examples (single preset, all targets), see the [README](../README.md). For CSS variable names, see the [CSS variables reference](css-variables.md).

## Loading built-in presets

Built-in presets are compiled into the binary. Use `preset()` to load them — it returns `Option<Palette>`, with `None` only if the ID doesn't match a builtin.

```rust
use palette_core::preset;

let palette = preset("tokyonight").expect("builtin preset");
```

This is the right choice when you're loading a known builtin ID. No error handling beyond the `Option`.

### `preset()` vs `load_preset()`

| | `preset(id)` | `load_preset(id)` |
|---|---|---|
| Returns | `Option<Palette>` | `Result<Palette, PaletteError>` |
| Unknown ID | `None` | `Err(UnknownPreset)` |
| Use when | Loading a known builtin by name | You need the error type (e.g. to propagate with `?`) |

Both resolve inheritance for variant presets (e.g. `tokyonight_storm` inherits from `tokyonight`).

For user-provided TOML files, use `load_preset_file()` or a `Registry` — those paths can genuinely fail (missing file, bad TOML, broken inheritance chain).

### WASM

```js
import { preset } from "palette-core";

const palette = preset("tokyonight");  // returns palette or undefined
```

### Fallback when loading fails

`Palette` implements `Default` — a neutral dark palette with base, semantic, and surface colors. Use it as a safe fallback when theme loading fails:

```rust
use palette_core::Palette;

let palette = reg.load(&user_choice).unwrap_or_default();
```

Or combine with `preset()` for a two-tier fallback:

```rust
use palette_core::preset;
use palette_core::Palette;

let palette = preset(&user_choice)
    .or_else(|| preset("tokyonight"))
    .unwrap_or_default();
```

The default palette covers `base`, `semantic`, and `surface` slots. Syntax, editor, terminal, and diff slots are `None` — downstream renderers should apply their own defaults for those.

## Theme switching with Registry

Use a `Registry` to expose all built-in presets. Load a default at startup. Let users pick from the list.

```rust
use palette_core::Registry;

let reg = Registry::new();

// Populate a settings menu
for info in reg.list() {
    println!("{} ({})", info.name, info.style);
}

// Load the user's choice (or fall back to a default)
let user_choice = "catppuccin";
let palette = reg.load(user_choice)?;
```

### CSS — generate all themes for live switching

```rust
use palette_core::Registry;

let reg = Registry::new();
let mut css = String::new();
for info in reg.list() {
    let palette = reg.load(&info.id)?;
    let selector = format!("[data-theme=\"{}\"]", info.id);
    css.push_str(&palette.to_css_scoped(&selector, None));
}
```

Switch themes in the browser by setting a data attribute:

```js
document.documentElement.dataset.theme = "catppuccin";
```

Reference the variables in CSS:

```css
body {
    background: var(--bg);
    color: var(--fg);
}
```

### Terminal — swap themes at runtime

```rust
use palette_core::Registry;
use palette_core::terminal::to_terminal_theme;

let reg = Registry::new();
let theme = to_terminal_theme(&reg.load("tokyonight_storm")?);
```

### WASM

```js
import { JsRegistry } from "palette-core";

const reg = new JsRegistry();
const themes = reg.list(); // [{id, name, style}, ...]

const palette = reg.load("dracula");
```

## Developer-defined custom presets

Add your own presets — either full themes or variants that inherit from a built-in.

**Variant that inherits from a built-in:**

```toml
# corporate_dark.toml
[meta]
name = "Corporate Dark"
preset_id = "corporate_dark"
schema_version = "1"
style = "dark"
kind = "preset-variant"
inherits = "tokyonight"

[semantic]
error = "#FF3333"
info = "#0099FF"
```

This theme gets all of tokyonight's colors, overriding only the semantic values.

**Full custom preset:**

```toml
# brand.toml
[meta]
name = "Brand Theme"
preset_id = "brand"
schema_version = "1"
style = "light"
kind = "preset-base"

[base]
background = "#FFFFFF"
foreground = "#1A1A1A"

[semantic]
error = "#CC0000"
success = "#008800"
```

**Register and use:**

```rust
use std::path::Path;
use palette_core::Registry;

let mut reg = Registry::new();

// Add a single file
reg.add_file(Path::new("themes/corporate_dark.toml"))?;

// Or add an entire directory of .toml files
reg.add_dir(Path::new("themes/"))?;

// Custom themes appear alongside built-ins
for info in reg.list() {
    println!("{}: {} ({})", info.id, info.name, info.style);
}

// Load like any other theme — inheritance resolves automatically
let palette = reg.load("corporate_dark")?;
```

Custom variants can inherit from built-ins or from other custom presets already in the registry.

**WASM**

```js
import { JsRegistry } from "palette-core";

const reg = new JsRegistry();
reg.addToml(corporateDarkToml);

const palette = reg.load("corporate_dark");
```

## End-user-defined presets

Let your users load their own theme files at runtime. The same registry handles built-in, developer, and user themes.

```rust
use std::path::Path;
use palette_core::Registry;

let mut reg = Registry::new();

// Developer themes ship with the app
reg.add_dir(Path::new("themes/"))?;

// End-user themes loaded from a config directory
let user_themes_dir = dirs::config_dir()
    .map(|d| d.join("myapp/themes"));

if let Some(dir) = user_themes_dir.as_deref() {
    if dir.is_dir() {
        reg.add_dir(dir)?;
    }
}

// All themes — built-in, developer, and user — are in one list
for info in reg.list() {
    println!("{}: {} ({})", info.id, info.name, info.style);
}

let palette = reg.load(&user_selected_theme_id)?;
```

A user preset with the same `preset_id` as an existing theme replaces it, so users can override built-ins or developer themes.

User presets support inheritance too — a user can write a variant that inherits from any theme already in the registry:

```toml
# ~/.config/myapp/themes/my_nord.toml
[meta]
name = "My Nord"
preset_id = "my_nord"
schema_version = "1"
style = "dark"
kind = "preset-variant"
inherits = "nord"

[base]
background = "#1a1a2e"
```

**WASM — user-supplied TOML string**

```js
const reg = new JsRegistry();
reg.addToml(userTomlString);
const palette = reg.load("my_nord");
```
