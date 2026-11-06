// 🦀 examples/exemplar03_hue.rs

#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use main_scope::{app_state, app_style, app_theme, app_words};

egui_knex::assert_exemplar_scope!(main_scope);

fn config() -> egui_knex::AppConfig {
	egui_knex::AppConfig::new("hue_exemplar03").title(app_words::Gui::APP_TITLE).inner_size((1100.0, 720.0))
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> egui_knex::eframe::Result<()> { egui_knex::run_app_with::<self::app_state::AppState>(config()) }

#[cfg(target_arch = "wasm32")]
fn main() { let _ = egui_knex::run_app_with::<self::app_state::AppState>(config()); }

mod main_scope {
	pub struct MainScopeBundle;
	impl egui_knex::ExemplarScope for MainScopeBundle {
		type Words = app_words::Gui;
		type Theme = app_theme::ThemeTokens;
		type State = app_state::AppState;
		type Style = app_style::AppStyle;
	}

	pub mod app_words {
		pub struct Gui;
		impl Gui {
			pub const APP_TITLE: &'static str = "Demonstrator Oklab / Oklch (feature::hue)";
			pub const LEG_OKLCH: &'static str = "OKLCH";
			pub const LEG_OKLAB: &'static str = "OKLab";
			pub const LBL_L: &'static str = "L";
			pub const LBL_C: &'static str = "C";
			pub const LBL_H: &'static str = "h (°)";
			pub const LBL_A: &'static str = "a";
			pub const LBL_B: &'static str = "b";
			pub const SEC_PALETTE: &'static str = "Zabawka: obrót odcienia";
			pub const BTN_ROTATE: &'static str = "Obróć odcień o +30°";
			pub const OUT_OF_GAMUT: &'static str = "Poza sRGB: chroma sprowadzona do gamutu";
		}
	}

	pub mod app_theme {
		use egui_knex::Color32;

		pub struct ThemeTokens;

		pub const BORDER: Color32 = Color32::from_rgb(70, 74, 82);
		pub const LEGEND: Color32 = Color32::from_rgb(200, 200, 200);
		pub const WARNING: Color32 = Color32::from_rgb(255, 180, 60);
	}

	pub mod app_state {
		use egui_knex::{Color32, egui, feature::hue::Color32OklabExt};

		use super::{app_style::AppStyle, app_words::Gui};

		/// Szerokość lewej kolumny (kwadrat, HEX, ostrzeżenie).
		const LEFT_COL_W: f32 = 220.0;

		/// Stan demonstracji. Źródłem prawdy jest Oklch (`l`, `c`, `h`);
		/// wartości Oklab i HEX są z niego wyliczane.
		pub struct AppState {
			pub l: f32,
			pub c: f32,
			pub h: f32,
			pub hex_input: String,
			pub hex_focused: bool,
		}

		impl Default for AppState {
			fn default() -> Self { Self { l: 0.70, c: 0.12, h: 250.0, hex_input: String::new(), hex_focused: false } }
		}

		impl AppState {
			/// Ustawia Oklch. Przy szarości (`c` ≈ 0) odcień zostaje poprzedni.
			fn set_lch(&mut self, l: f32, c: f32, h: f32) {
				self.l = l;
				self.c = c;
				if c > 1e-4 {
					self.h = h;
				}
			}
		}

		fn hex_string(color: Color32) -> String { format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b()) }

		/// `#rrggbb`, `rrggbb`, `#rgb` lub `rgb`; wielkość liter bez znaczenia.
		fn parse_hex(text: &str) -> Option<Color32> {
			let digits = text.trim().trim_start_matches('#');
			let expanded: String = match digits.len() {
				3 => digits.chars().flat_map(|c| [c, c]).collect(),
				6 => digits.to_owned(),
				_ => return None,
			};
			if !expanded.chars().all(|c| c.is_ascii_hexdigit()) {
				return None;
			}
			let v = u32::from_str_radix(&expanded, 16).ok()?;
			Some(Color32::from_rgb(((v >> 16) & 0xFF) as u8, ((v >> 8) & 0xFF) as u8, (v & 0xFF) as u8))
		}

		/// Odcień w stopniach z przedziału `[0, 360)` dla składowych `a`, `b`.
		fn hue_of(a: f32, b: f32) -> f32 {
			let h = b.atan2(a).to_degrees();
			let h = if h < 0.0 { h + 360.0 } else { h };
			if h >= 360.0 { h - 360.0 } else { h }
		}

		impl egui_knex::eframe::App for AppState {
			fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut egui_knex::eframe::Frame) {
				let style = AppStyle;
				let current = Color32::from_oklch(self.l, self.c, self.h);
				let (_, current_chroma, _) = current.to_oklch();
				let out_of_gamut = self.c - current_chroma > 0.01;

				// Pole HEX pokazuje aktualny kolor, dopóki użytkownik go nie
				// edytuje
				if !self.hex_focused {
					self.hex_input = hex_string(current);
				}

				// Szerokość kafelka wspólna dla obu wierszy, dzięki temu oba
				// mają tę samą długość
				let tile_w = ((ui.available_width() - 27.0 * 2.0) / 28.0).max(8.0);

				// 1a. Kolo odcieni: 28 kafelków w jednym wierszu
				ui.horizontal(|ui| {
					ui.spacing_mut().item_spacing.x = 2.0;
					for k in 0..28_u8 {
						let h = f32::from(k) * 360.0 / 28.0;
						style
							.swatch(ui, Color32::from_oklch(self.l, self.c, h), egui::vec2(tile_w, 40.0))
							.on_hover_text(format!("h = {h:.1}°"));
					}
				});

				// 1b. Aktualny kolor zrzutowany na szarość: ten sam L, C = 0.
				// Jeden klocek na długość całego rzędu 28 kafelków, o połowę
				// ich wysokości.
				let row_w = 28.0 * tile_w + 27.0 * 2.0;
				let gray = Color32::from_oklch(self.l, 0.0, 0.0);
				style.swatch(ui, gray, egui::vec2(row_w, 20.0)).on_hover_text(format!("L = {:.2}, C = 0", self.l));

				ui.add_space(12.0);

				// 2. Kolor z HEX | ramka OKLCH | ramka OKLab
				ui.horizontal_top(|ui| {
					ui.spacing_mut().slider_width = 180.0;

					ui.vertical(|ui| {
						// Stała szerokość kolumny: napis nie może jej
						// poszerzać, bo przesuwa suwaki
						ui.set_width(LEFT_COL_W);

						let (rect, _) =
							ui.allocate_exact_size(egui::vec2(LEFT_COL_W, LEFT_COL_W), egui::Sense::hover());
						ui.painter().rect_filled(rect, 10.0, current);
						ui.painter().rect_stroke(
							rect,
							10.0,
							egui::Stroke::new(1.0, super::app_theme::BORDER),
							egui::StrokeKind::Inside,
						);

						let resp = ui.add(
							egui::TextEdit::singleline(&mut self.hex_input)
								.desired_width(LEFT_COL_W)
								.font(egui::TextStyle::Monospace),
						);
						self.hex_focused = resp.has_focus();
						if resp.changed() {
							if let Some(color) = parse_hex(&self.hex_input) {
								let (l, c, h) = color.to_oklch();
								self.set_lch(l, c, h);
							}
						}

						// Miejsce na ostrzeżenie jest zarezerwowane zawsze (2
						// linie), więc layout nie skacze
						let warning = if out_of_gamut { Gui::OUT_OF_GAMUT } else { "" };
						let warning_h = ui.text_style_height(&egui::TextStyle::Body) * 2.0;
						ui.add_sized(
							[LEFT_COL_W, warning_h],
							egui::Label::new(egui::RichText::new(warning).color(super::app_theme::WARNING)),
						);
					});

					ui.add_space(16.0);

					style.fieldset(ui, Gui::LEG_OKLCH, |ui| {
						ui.vertical(|ui| {
							ui.add(egui::Slider::new(&mut self.l, 0.0..=1.0).text(Gui::LBL_L));
							ui.add(egui::Slider::new(&mut self.c, 0.0..=0.4).text(Gui::LBL_C));
							ui.add(egui::Slider::new(&mut self.h, 0.0..=360.0).text(Gui::LBL_H));
						});
					});

					ui.add_space(16.0);

					style.fieldset(ui, Gui::LEG_OKLAB, |ui| {
						ui.vertical(|ui| {
							let rad = self.h.to_radians();
							let mut l = self.l;
							let mut a = self.c * rad.cos();
							let mut b = self.c * rad.sin();

							let mut changed = ui.add(egui::Slider::new(&mut l, 0.0..=1.0).text(Gui::LBL_L)).changed();
							changed |= ui.add(egui::Slider::new(&mut a, -0.4..=0.4).text(Gui::LBL_A)).changed();
							changed |= ui.add(egui::Slider::new(&mut b, -0.4..=0.4).text(Gui::LBL_B)).changed();

							if changed {
								self.set_lch(l, a.hypot(b), hue_of(a, b));
							}
						});
					});
				});

				ui.add_space(16.0);

				// 3. Zabawka: obrót odcienia
				style.section_title(ui, Gui::SEC_PALETTE);
				if ui.button(Gui::BTN_ROTATE).clicked() {
					self.h = (self.h + 30.0).rem_euclid(360.0);
				}
				ui.horizontal(|ui| {
					for k in -2_i8..=2 {
						let h = (self.h + 30.0 * f32::from(k)).rem_euclid(360.0);
						style.swatch(ui, Color32::from_oklch(self.l, self.c, h), egui::vec2(56.0, 56.0));
					}
				});
			}
		}
	}

	pub mod app_style {
		use egui_knex::{Color32, egui};

		pub struct AppStyle;

		impl AppStyle {
			pub fn section_title(&self, ui: &mut egui::Ui, text: &str) {
				ui.label(egui::RichText::new(text).strong().size(15.0));
				ui.add_space(4.0);
			}

			pub fn swatch(&self, ui: &mut egui::Ui, color: Color32, size: egui::Vec2) -> egui::Response {
				let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::hover());
				ui.painter().rect_filled(rect, 6.0, color);
				resp
			}

			/// Ramka w stylu HTML `<fieldset>` z podpisem `legend` na górnej
			/// krawędzi.
			pub fn fieldset(&self, ui: &mut egui::Ui, legend: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
				let frame = egui::Frame::new()
					.stroke(egui::Stroke::new(1.0, super::app_theme::BORDER))
					.corner_radius(6.0)
					.inner_margin(egui::Margin { left: 12, right: 12, top: 14, bottom: 10 });
				let rect = frame.show(ui, add_contents).response.rect;

				// Podpis wycinający w ramce: tło paneli zasłania linię pod
				// tekstem
				let galley = ui.painter().layout_no_wrap(
					legend.to_owned(),
					egui::FontId::proportional(13.0),
					super::app_theme::LEGEND,
				);
				let text_size = galley.size();
				let pos = egui::pos2(rect.left() + 10.0, rect.top() - text_size.y * 0.5);
				let gap = egui::Rect::from_min_size(pos - egui::vec2(4.0, 0.0), text_size + egui::vec2(8.0, 0.0));
				ui.painter().rect_filled(gap, 0.0, ui.visuals().panel_fill);
				ui.painter().galley(pos, galley, super::app_theme::LEGEND);
			}
		}
	}
}
