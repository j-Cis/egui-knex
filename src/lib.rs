// 🦀 src\lib.rs

//! `egui-knex` — reużywalne klocki dla projektów opartych o `egui` / `eframe`.
//!
//! # Re-eksporty (zgodność typów z projektem nadrzędnym)
//!
//! Typy `egui` (np. [`Ui`], [`Rect`], [`Color32`]) są re-eksportowane stąd,
//! więc projekt nadrzędny może importować je z tego crate'a i ma gwarancję, że
//! są to dokładnie te same typy, których używa biblioteka — niezależnie od
//! tego, jaką wersję `egui` ma we własnym `Cargo.toml`:
//!
//! ```ignore
//! use egui_knex::{Color32, Rect, Ui};    // wybrane typy
//! use egui_knex::egui;                   // cały crate `egui`
//! use egui_knex::eframe;                 // cały crate `eframe`
//! use egui_knex::prelude::*;             // typowy zestaw + klocki biblioteki
//! ```

pub use eframe::{
	self,
	egui,
	egui::{Color32, Pos2, Rect, Stroke, Ui, Vec2, pos2, vec2},
};

pub mod element;
pub mod feature;
mod include {
	mod egui_run_app;
	pub mod exemplar;
	pub use egui_run_app::{AppConfig, run_app, run_app_with, run_app_with_creator};
	pub use exemplar::ExemplarScope;
}
pub use include::{AppConfig, ExemplarScope, run_app, run_app_with, run_app_with_creator};

/// Typowy zestaw importów: najczęściej używane typy `egui` oraz klocki
/// biblioteki.
pub mod prelude {
	pub use crate::{
		AppConfig,
		egui::{Color32, Context, Pos2, Rect, Stroke, Ui, Vec2, pos2, vec2},
		element::{Vas, VasDispositio, VasFovea, VasRedundantia, VasSpatium},
		run_app,
		run_app_with,
		run_app_with_creator,
	};
}
