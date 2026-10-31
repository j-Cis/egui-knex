// 🦀 examples/vas_exemplar01.rs

#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use main_scope::{app_state, app_style, app_theme, app_words};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> { egui_knex::run_app::<self::app_state::AppState>() }

#[cfg(target_arch = "wasm32")]
fn main() { let _ = egui_knex::run_app::<self::app_state::AppState>(); }

mod main_scope {
	pub mod app_words {
		pub struct Gui;
		impl Gui {
			pub const APP_TITLE: &'static str = "Aplikacja wystartowała poprawnie w egui 0.37!";
			pub const TAB_01_BTN: &'static str = "(Tab01)";
			pub const TAB_02_BTN: &'static str = "(Tab02)";
			pub const TAB_01_PANEL: &'static str = "⚙️ Tab01...";
			pub const TAB_02_PANEL: &'static str = "🎬 Tab02...";
			pub const SCREEN_01: &'static str = "📔 Widok Tab01";
			pub const SCREEN_02: &'static str = "📘 Widok Tab02";
		}

		pub struct Log;
		impl Log {
			pub const ERR_NAV_SLOT: &'static str = "Błąd systemu layoutu: brak gniazda belki nawigacyjnej";
			pub const ERR_CANVAS_SLOT: &'static str = "Błąd systemu layoutu: brak gniazda kanwy";
		}
	}

	pub mod app_theme {
		use eframe::egui::Color32;

		pub const BOTTOM_BAR_HEIGHT: f32 = 40.0;
		pub const PANEL_HEIGHT: f32 = 150.0;

		pub const BOTTOM_BAR_BG: Color32 = Color32::from_rgb(30, 30, 30);
		pub const PANEL_BG: Color32 = Color32::from_rgb(45, 45, 45);
		pub const CANVAS_BG: Color32 = Color32::from_rgb(20, 20, 20);
		pub const SEPARATOR_COLOR: Color32 = Color32::from_rgb(70, 70, 70);

		pub const BTN_ACTIVE_BG: Color32 = Color32::from_rgb(0, 122, 255);
		pub const TEXT_DEFAULT: Color32 = Color32::from_rgb(240, 240, 240);
		pub const INPUT_BG: Color32 = Color32::from_rgb(25, 25, 25);

		pub const SLIDER_INPUT_BG: Color32 = Color32::from_rgb(35, 35, 35);
		pub const SLIDER_RAIL_COLOR: Color32 = Color32::from_rgb(50, 50, 50);
		pub const SLIDER_HANDLE_COLOR: Color32 = Color32::from_rgb(0, 122, 255);
		pub const SLIDER_HANDLE_BORDER: Color32 = Color32::from_rgb(255, 255, 255);

		pub const FONT_SIZE_LABEL: f32 = 14.0;
		pub const FONT_SIZE_BTN: f32 = 14.0;
		pub const FONT_SIZE_SECTION_TITLE: f32 = 18.0;

		pub const INPUT_HEIGHT: f32 = 24.0;
	}

	pub mod app_state {
		use eframe::egui;

		// PRAWIDŁOWO: Dodano Eq do zestawu traitów dla enuma bez danych
		#[derive(PartialEq, Eq, Clone, Copy)]
		pub enum AppMode {
			Tab01,
			Tab02,
		}

		pub struct AppState {
			pub mode: AppMode,
		}

		impl Default for AppState {
			fn default() -> Self { Self { mode: AppMode::Tab01 } }
		}

		impl eframe::App for AppState {
			fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
				// 1. Tworzymy konfigurację layoutu na bazie aktualnego stanu
				//    aplikacji
				let style = super::app_style::AppStyle {
					show_panel: true, // Chcemy widzieć dolny panel opcji
					mode: self.mode,  // Przekazujemy aktualnie wybraną kartę
				};

				// 2. Tniemy okno na sloty (belka nawigacyjna, panel opcji,
				//    główna kanwa)
				let mut frame = style.begin(ui);

				// 3. OŻYWIANIE BELKI NAWIGACYJNEJ: Rysujemy przełączniki w
				//    przygotowanych slotach
				if let Some(slot_tab1) = frame.nav_slots.get_mut(0)
					&& slot_tab1
						.selectable_label(self.mode == AppMode::Tab01, super::app_words::Gui::TAB_01_BTN)
						.clicked()
				{
					self.mode = AppMode::Tab01;
				}

				if let Some(slot_tab2) = frame.nav_slots.get_mut(1)
					&& slot_tab2
						.selectable_label(self.mode == AppMode::Tab02, super::app_words::Gui::TAB_02_BTN)
						.clicked()
				{
					self.mode = AppMode::Tab02;
				}

				// 4. OŻYWIANIE PANELU OPCJI: Wyświetlamy inną zawartość w
				//    zależności od trybu
				if let Some(ref mut slots) = frame.panel_slots {
					if self.mode == AppMode::Tab01 {
						if let Some(slot_content) = slots.get_mut(0) {
							slot_content.label(super::app_words::Gui::TAB_01_PANEL);
						}
					} else if let Some(slot_content) = slots.get_mut(0) {
						slot_content.label(super::app_words::Gui::TAB_02_PANEL);
					}
				}

				// 5. OŻYWIANIE GŁÓWNEJ KANWY
				frame.canvas_ui.centered_and_justified(|ui| {
					ui.heading(match self.mode {
						AppMode::Tab01 => super::app_words::Gui::SCREEN_01,
						AppMode::Tab02 => super::app_words::Gui::SCREEN_02,
					});
				});
			}
		}
	}

	pub mod app_style {
		use eframe::egui;
		use egui_knex::element::{Vas, VasFovea, VasRedundantia, VasSpatium};

		use super::app_state::AppMode;

		pub struct LayoutFrame {
			pub canvas_ui: egui::Ui,
			pub panel_slots: Option<Vec<egui::Ui>>,
			pub nav_slots: Vec<egui::Ui>,
		}

		pub struct AppStyle {
			pub show_panel: bool,
			pub mode: AppMode,
		}

		impl AppStyle {
			pub fn apply_theme(&self, ctx: &egui::Context) {
				let themes = [egui::Theme::Dark, egui::Theme::Light];
				for theme in themes {
					let mut style = (*ctx.style_of(theme)).clone();

					style.visuals.selection.bg_fill = super::app_theme::BTN_ACTIVE_BG;
					style.visuals.selection.stroke = egui::Stroke::new(1.0, super::app_theme::BTN_ACTIVE_BG);
					style.visuals.override_text_color = Some(super::app_theme::TEXT_DEFAULT);
					style.visuals.widgets.noninteractive.bg_stroke =
						egui::Stroke::new(1.0, super::app_theme::SEPARATOR_COLOR);
					style.visuals.extreme_bg_color = super::app_theme::INPUT_BG;

					style.visuals.widgets.noninteractive.weak_bg_fill = super::app_theme::SLIDER_INPUT_BG;
					style.visuals.widgets.inactive.weak_bg_fill = super::app_theme::SLIDER_INPUT_BG;
					style.visuals.widgets.hovered.weak_bg_fill = super::app_theme::SLIDER_INPUT_BG;
					style.visuals.widgets.active.weak_bg_fill = super::app_theme::SLIDER_INPUT_BG;

					style.visuals.widgets.inactive.bg_fill = super::app_theme::SLIDER_RAIL_COLOR;
					style.visuals.widgets.hovered.bg_fill = super::app_theme::SLIDER_RAIL_COLOR;
					style.visuals.widgets.active.bg_fill = super::app_theme::SLIDER_HANDLE_COLOR;
					style.visuals.widgets.active.fg_stroke =
						egui::Stroke::new(1.5, super::app_theme::SLIDER_HANDLE_BORDER);

					style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(4);
					style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(4);
					style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(4);
					style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(4);

					style
						.text_styles
						.insert(egui::TextStyle::Body, egui::FontId::proportional(super::app_theme::FONT_SIZE_LABEL));
					style
						.text_styles
						.insert(egui::TextStyle::Button, egui::FontId::proportional(super::app_theme::FONT_SIZE_BTN));
					style.text_styles.insert(
						egui::TextStyle::Heading,
						egui::FontId::proportional(super::app_theme::FONT_SIZE_SECTION_TITLE),
					);

					style.spacing.interact_size.y = super::app_theme::INPUT_HEIGHT;

					ctx.set_style_of(theme, style);
				}
			}

			/// # Panics
			/// Funkcja panikuje, jeśli silnik layoutu `Vas` nie zwróci
			/// oczekiwanych indeksów foveae
			/// (wymagane identyfikatory gniazd: 1 oraz 3).
			#[allow(clippy::too_many_lines)] // Dozwolone w monolitycznych plikach demonstracyjnych layoutu
			pub fn begin(&self, ui: &mut egui::Ui) -> LayoutFrame {
				self.apply_theme(ui.ctx());
				let full_rect = ui.available_rect_before_wrap();

				// =========================================================
				// A) "VAS" główny root (Compressio - ściskanie pionowe)
				// =========================================================
				let vas_root = Vas {
					axis_maior_horizontalis: false,
					axis_maior_reversus: true,
					redundantia_maior: VasRedundantia::Compressio,
					padding_am_cm_at_ct_gap: (0.0, 0.0, 0.0, 0.0, 0.0),
					foveae: Box::new([
						VasFovea {
							numerus: 1,
							spatium: VasSpatium::Pixel(super::app_theme::BOTTOM_BAR_HEIGHT),
							contractio: 0.0,
							amplificatio: 0.0,
							repletio: false,
							latens: false,
						},
						VasFovea {
							numerus: 2,
							spatium: VasSpatium::Pixel(super::app_theme::PANEL_HEIGHT),
							contractio: 1.0,
							amplificatio: 0.0,
							repletio: false,
							latens: !self.show_panel,
						},
						VasFovea {
							numerus: 3,
							spatium: VasSpatium::Auto,
							contractio: 1.0,
							amplificatio: 1.0,
							repletio: true,
							latens: false,
						},
					]),
				};

				let root_rects = vas_root.resolve(full_rect);

				// Zamieniono unwrap() na expect() spełniając wymogi
				// clippy::missing_panics_doc
				let rect_nav = root_rects
					.iter()
					.find(|(n, _)| *n == 1)
					.expect("Błąd systemu layoutu: brak gniazda belki nawigacyjnej")
					.1;
				let rect_panel = root_rects.iter().find(|(n, _)| *n == 2).map(|(_, r)| *r);
				let rect_canvas =
					root_rects.iter().find(|(n, _)| *n == 3).expect("Błąd systemu layoutu: brak gniazda kanwy").1;

				// =========================================================
				// B) "VAS" Belka pozioma (Volutatio - zachowanie rozmiarów i
				// scroll)
				// =========================================================
				ui.painter().rect_filled(rect_nav, 0.0, super::app_theme::BOTTOM_BAR_BG);
				ui.painter().line_segment(
					[rect_nav.left_top(), rect_nav.right_top()],
					egui::Stroke::new(1.0, super::app_theme::SEPARATOR_COLOR),
				);

				let vas_nav = Vas {
					axis_maior_horizontalis: true,
					axis_maior_reversus: false,
					redundantia_maior: VasRedundantia::Volutatio,
					padding_am_cm_at_ct_gap: (10.0, 10.0, 4.0, 4.0, 8.0),
					foveae: Box::new([
						VasFovea { numerus: 0, spatium: VasSpatium::Auto, ..Default::default() },
						VasFovea { numerus: 1, spatium: VasSpatium::Auto, ..Default::default() },
						VasFovea { numerus: 2, spatium: VasSpatium::Auto, ..Default::default() },
						VasFovea { numerus: 3, spatium: VasSpatium::Auto, ..Default::default() },
						VasFovea { numerus: 4, spatium: VasSpatium::Pixel(30.0), ..Default::default() },
						VasFovea { numerus: 5, spatium: VasSpatium::Pixel(30.0), ..Default::default() },
						VasFovea { numerus: 6, spatium: VasSpatium::Pixel(20.0), ..Default::default() },
					]),
				};

				let total_nav_width = vas_nav.total_content_length(rect_nav);
				let max_scroll = (total_nav_width - rect_nav.width()).max(0.0);

				let nav_scroll_id = ui.scope_id().with("vas_nav_volutatio_offset");
				let mut scroll_x = ui.ctx().data_mut(|d| *d.get_temp_mut_or_default::<f32>(nav_scroll_id));

				let is_pointer_down = ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary));
				let pointer_pos = ui.input(|i| i.pointer.latest_pos());
				if is_pointer_down && pointer_pos.is_some_and(|pos| rect_nav.contains(pos)) {
					let delta_x = ui.input(|i| i.pointer.delta().x);
					scroll_x -= delta_x;
				}
				if pointer_pos.is_some_and(|pos| rect_nav.contains(pos)) {
					let wheel = ui.input(|i| i.smooth_scroll_delta.x + i.smooth_scroll_delta.y);
					scroll_x -= wheel;
				}
				scroll_x = scroll_x.clamp(0.0, max_scroll);
				ui.ctx().data_mut(|d| *d.get_temp_mut_or_default::<f32>(nav_scroll_id) = scroll_x);

				let mut nav_slots = Vec::new();
				for (_, r) in vas_nav.resolve_scroll(rect_nav, scroll_x) {
					let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
					child.set_clip_rect(rect_nav);
					nav_slots.push(child);
				}

				// =========================================================
				// C i D) "VAS" Panel opcji (w Gnieździe 2)
				// =========================================================
				// POPRAWKA DLA clippy::option_if_let_else: Zastąpiono if let
				// przez map_or_else
				let panel_slots = rect_panel.map_or_else(
					|| None,
					|r_panel| {
						ui.painter().rect_filled(r_panel, 0.0, super::app_theme::PANEL_BG);
						ui.painter().line_segment(
							[r_panel.left_top(), r_panel.right_top()],
							egui::Stroke::new(1.0, super::app_theme::SEPARATOR_COLOR),
						);

						let mut slots = Vec::new();

						if self.mode == AppMode::Tab01 {
							let vas_calc = Vas {
								axis_maior_horizontalis: true,
								axis_maior_reversus: false,
								redundantia_maior: VasRedundantia::Paginatio,
								padding_am_cm_at_ct_gap: (10.0, 10.0, 10.0, 10.0, 20.0),
								foveae: Box::new([
									VasFovea { numerus: 0, spatium: VasSpatium::Auto, ..Default::default() },
									VasFovea { numerus: 1, spatium: VasSpatium::Auto, ..Default::default() },
									VasFovea { numerus: 2, spatium: VasSpatium::Auto, ..Default::default() },
								]),
							};
							for (_, r) in vas_calc.resolve(r_panel) {
								let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
								child.set_clip_rect(r);
								slots.push(child);
							}
						} else {
							let vas_anim = Vas {
								axis_maior_horizontalis: true,
								axis_maior_reversus: true,
								redundantia_maior: VasRedundantia::Flexio,
								padding_am_cm_at_ct_gap: (10.0, 10.0, 10.0, 10.0, 20.0),
								foveae: Box::new([
									VasFovea { numerus: 0, spatium: VasSpatium::Auto, ..Default::default() },
									VasFovea { numerus: 1, spatium: VasSpatium::Auto, ..Default::default() },
									VasFovea { numerus: 2, spatium: VasSpatium::Auto, ..Default::default() },
									VasFovea { numerus: 3, spatium: VasSpatium::Auto, ..Default::default() },
								]),
							};
							for (_, r) in vas_anim.resolve(r_panel) {
								let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
								child.set_clip_rect(r);
								slots.push(child);
							}
						}
						Some(slots)
					},
				);
				// =========================================================
				// KANWA (w Gnieździe 3)
				// =========================================================
				ui.painter().rect_filled(rect_canvas, 0.0, super::app_theme::CANVAS_BG);
				let mut canvas_ui = ui.new_child(egui::UiBuilder::new().max_rect(rect_canvas));
				canvas_ui.set_clip_rect(rect_canvas);
				LayoutFrame { canvas_ui, panel_slots, nav_slots }
			}
		}
	}
}
