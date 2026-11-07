# egui-knex

[![Crates.io](https://img.shields.io/crates/v/egui-knex.svg)](https://crates.io/crates/egui-knex)
[![Downloads](https://img.shields.io/crates/d/egui-knex.svg)](https://crates.io/crates/egui-knex)
[![GitHub stars](https://img.shields.io/github/stars/j-Cis/egui-knex.svg?style=social&label=Star)](https://github.com/j-Cis/egui-knex)
[![Docs.rs](https://docs.rs/egui-knex/badge.svg)](https://docs.rs/egui-knex)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.99.0%2B-yellow.svg)](https://www.rust-lang.org)

## 🏗️ Building blocks / Klocki

### 🏗️ English

egui-knex adds a small set of reusable building blocks to egui:

* Ora: floating windows with a roll bar (collapsible, draggable, optional fullscreen) and centred modal windows.
* Vas: a flexbox-like layout. You describe the slots and Vas divides the available rectangle among them, with redundancy modes such as compression and scrolling.
* feature::hue: Oklab and Oklch conversions for egui::Color32, following the CSS Color 4 gamut mapping for colours outside sRGB.
* Exemplars: runnable examples (exemplar01_vas, exemplar02_ora, exemplar03_hue) and a compile-time contract (assert_exemplar_scope!) that splits an app into words, theme, state and style.

### 🏗️ Polski

egui-knex dodaje do egui zestaw klocków wielokrotnego użytku:

* Ora: pływające okna z belką do zwijania (przesuwalne, z opcjonalnym pełnym ekranem) oraz wycentrowane okna modalne.
* Vas: układ w stylu flexbox. Opisujesz gniazda, a Vas dzieli między nie dostępny prostokąt, z trybami nadmiarowości, np. ściskaniem i przewijaniem.
* feature::hue: konwersje Oklab i Oklch dla egui::Color32, zgodne z mapowaniem gamutu z CSS Color 4 dla kolorów spoza sRGB.
* Exemplary: uruchamialne przykłady (exemplar01_vas, exemplar02_ora, exemplar03_hue) oraz kontrakt sprawdzany podczas kompilacji (assert_exemplar_scope!), który dzieli aplikację na słowa, motyw, stan i styl.

## 🚀 Entry points / Punkty startowe

### 🚀 English

Your app type implements `eframe::App`. Pick one of three entry points:

* `run_app::<A>()` starts `A` with `AppConfig::default()`. `A` must also implement `Default`.
* `run_app_with::<A>(config)` starts `A` with your own `AppConfig`. `A` must also implement `Default`.
* `run_app_with_creator(config, |cc| ...)` builds the app from an `eframe::CreationContext` (fonts, style, restored state), so `Default` is not needed.

`AppConfig` sets the name, the window title, the window size, the `<canvas>` id and an optional renderer. The name is also used by eframe for storage, so give each app its own. The default name is `egui-knex`. The title and size apply natively only, the canvas id applies on WASM only, and the other platform ignores them. The renderer needs the `glow` or `wgpu` feature enabled in `eframe`.

Natively the call blocks until the window closes. On WASM it returns immediately, the start is asynchronous, and errors go to the browser console.

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn config() -> egui_knex::AppConfig {
    egui_knex::AppConfig::new("my_app").title("My app").inner_size((1200.0, 800.0))
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> egui_knex::eframe::Result<()> {
    egui_knex::run_app_with::<MyState>(config())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let _ = egui_knex::run_app_with::<MyState>(config());
}
```

### 🚀 Polski

Typ aplikacji implementuje `eframe::App`. Wybierasz jeden z trzech punktów startowych:

* `run_app::<A>()` uruchamia `A` z `AppConfig::default()`. `A` musi też implementować `Default`.
* `run_app_with::<A>(config)` uruchamia `A` z własnym `AppConfig`. `A` musi też implementować `Default`.
* `run_app_with_creator(config, |cc| ...)` buduje aplikację z `eframe::CreationContext` (czcionki, styl, odczyt zapisanego stanu), więc `Default` nie jest potrzebny.

`AppConfig` ustawia nazwę, tytuł okna, rozmiar okna, id elementu `<canvas>` i opcjonalny renderer. Nazwa służy też eframe jako miejsce zapisu stanu, więc każda aplikacja powinna mieć własną. Domyślna nazwa to `egui-knex`. Tytuł i rozmiar działają tylko natywnie, id canvasu tylko w WASM, a druga platforma je ignoruje. Renderer wymaga włączonej cechy `glow` albo `wgpu` w `eframe`.

Natywnie wywołanie blokuje się do zamknięcia okna. W WASM zwraca od razu, start jest asynchroniczny, a błędy trafiają do konsoli przeglądarki.

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn config() -> egui_knex::AppConfig {
    egui_knex::AppConfig::new("my_app").title("My app").inner_size((1200.0, 800.0))
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> egui_knex::eframe::Result<()> {
    egui_knex::run_app_with::<MyState>(config())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let _ = egui_knex::run_app_with::<MyState>(config());
}
```

---
🫟
