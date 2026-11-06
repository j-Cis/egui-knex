// 🦀 examples/ora_exemplar02.rs

#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use main_scope::{app_state, app_style, app_theme, app_words};

egui_knex::assert_exemplar_scope!(main_scope);

fn config() -> egui_knex::AppConfig {
	egui_knex::AppConfig::new("ora_exemplar02").title(app_words::Gui::APP_TITLE).inner_size((1200.0, 800.0))
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
			pub const APP_TITLE: &'static str = "Demonstrator Ora (Okna Vektorowe) w egui-knex";
			pub const WIN1_TITLE: &'static str = "Narzędzia geometrii (Roll: Linea)";
			pub const WIN2_TITLE: &'static str = "Właściwości rzutu (Roll: Triangulum)";
			pub const MODAL_TITLE: &'static str = "Okno Ostrzeżenia (CentraleObvelatum)";
			pub const OPEN_MODAL_BTN: &'static str = "Otwórz okno modalne";
			pub const CLOSE_MODAL_BTN: &'static str = "Rozumiem, zamknij";
			pub const NEW_CAMERA_BTN: &'static str = "📷 + Nowy wizjer osoby";
		}

		pub struct Persons;
		impl Persons {
			pub const LIST: &'static [(&'static str, &'static str, &'static str)] = &[
				("Jan Kowalski", "Warszawa · 1885", "Linia męska"),
				("Maria Wiśniewska", "Kraków · 1910", "Do sprawdzenia"),
				("Piotr Nowak", "Poznań · 1902", "Zweryfikowany"),
				("Zofia Zielińska", "Gdańsk · 1925", "Akt urodzenia"),
				("Stanisław Kamiński", "Lwów · 1879", "Archiwum AGAD"),
			];
		}
	}

	pub mod app_theme {
		use egui_knex::Color32;

		pub struct ThemeTokens;

		pub const CANVAS_BG: Color32 = Color32::from_rgb(18, 19, 21);
		pub const GRID_LINE: Color32 = Color32::from_rgb(32, 35, 40);

		pub const HEADER_BG: Color32 = Color32::from_rgb(34, 37, 43);
		pub const CONTENT_BG: Color32 = Color32::from_rgb(26, 28, 32);
		pub const TEXT_DEFAULT: Color32 = Color32::from_rgb(235, 235, 235);
	}

	pub mod app_state {
		use egui_knex::{
			egui,
			element::{Ora, OraModus, OraRollIcon},
		};

		/// Samodzielna instancja dynamicznego okna podglądu (np. kamera na
		/// osobę)
		pub struct PersonViewer {
			pub id_num: usize,
			pub ora: Ora,
			pub selected_person_idx: usize,
		}

		pub struct AppState {
			pub win_tools: Ora,
			pub win_props: Ora,
			pub modal_alert: Ora,
			pub viewers: Vec<PersonViewer>,
			pub next_viewer_id: usize,
			pub counter: usize,
		}

		impl Default for AppState {
			fn default() -> Self {
				let win_tools = Ora::nova("win_tools", super::app_words::Gui::WIN1_TITLE)
					.positio((50.0, 60.0))
					.magnitudo((320.0, 220.0))
					.roll_icon(OraRollIcon::Linea);

				let win_props = Ora::nova("win_props", super::app_words::Gui::WIN2_TITLE)
					.positio((420.0, 100.0))
					.magnitudo((340.0, 260.0))
					.roll_icon(OraRollIcon::Triangulum);

				let mut modal_alert = Ora::nova("modal_alert", super::app_words::Gui::MODAL_TITLE)
					.modus(OraModus::CentraleObvelatum)
					.magnitudo((380.0, 160.0));
				modal_alert.claude();

				Self { win_tools, win_props, modal_alert, viewers: Vec::new(), next_viewer_id: 1, counter: 0 }
			}
		}

		impl egui_knex::eframe::App for AppState {
			fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut egui_knex::eframe::Frame) {
				let style = super::app_style::AppStyle;
				style.draw_canvas_background(ui);

				let persons = super::app_words::Persons::LIST;

				// Pasek górny: 4 przyciski
				ui.horizontal(|ui| {
					if ui.button("Pokaż okno narzędzi").clicked() {
						self.win_tools.aperi();
					}
					if ui.button("Pokaż okno właściwości").clicked() {
						self.win_props.aperi();
					}
					if ui.button(super::app_words::Gui::OPEN_MODAL_BTN).clicked() {
						self.modal_alert.aperi();
					}

					ui.separator();

					// 4. GUZIK: Tworzy nową instancję kamery / wizjera
					if ui.button(super::app_words::Gui::NEW_CAMERA_BTN).clicked() {
						let id = self.next_viewer_id;
						self.next_viewer_id += 1;

						// Kaskadowe przesunięcie pozycji każdego nowego okna
						let cascade = ((id - 1) % 8) as f32 * 25.0;
						let titulus = format!("Wizjer #{id}");

						let win = Ora::nova(format!("person_cam_{id}"), titulus)
							.positio((200.0 + cascade, 140.0 + cascade))
							.magnitudo((290.0, 175.0))
							.roll_icon(OraRollIcon::Triangulum);

						self.viewers.push(PersonViewer {
							id_num: id,
							ora: win,
							selected_person_idx: (id - 1) % persons.len(),
						});
					}
				});

				// Okno 1: Narzędzia
				self.win_tools.show(ui, |content_ui| {
					content_ui.label("Przeciągaj za wolne miejsce belki obok przycisków.");
					content_ui.add_space(8.0);
					if content_ui.button("Inkrementuj licznik").clicked() {
						self.counter += 1;
					}
					content_ui.label(format!("Wartość: {}", self.counter));
					content_ui.separator();
					content_ui.label("Sterowanie oknem właściwości:");
					content_ui.checkbox(&mut self.win_props.mobilis, "Właściwości: mobilność");
					content_ui.checkbox(&mut self.win_props.mutabilis, "Właściwości: skalowalność");
				});

				// Okno 2: Właściwości
				self.win_props.show(ui, |content_ui| {
					content_ui.label("To okno posiada trójkątny wskaźnik zwijania ▲ / ▼.");
					content_ui.add_space(8.0);
					content_ui.label(format!("Aktywne kamery podglądu: {}", self.viewers.len()));
				});

				// Okno 3: Centralny modal
				let should_close = self.modal_alert.show(ui, |modal_ui| {
					modal_ui.label("To okno wycentrowało się na ekranie i zablokowało tło.");
					modal_ui.add_space(12.0);
					modal_ui.button(super::app_words::Gui::CLOSE_MODAL_BTN).clicked()
				});

				if should_close == Some(true) {
					self.modal_alert.claude();
				}

				// WIELE OKIEN WIZJERA (Dynamiczne instancje)
				for viewer in &mut self.viewers {
					viewer.ora.show(ui, |cam_ui| {
						cam_ui.horizontal(|ui| {
							ui.label(egui::RichText::new(format!("Kamera #{}", viewer.id_num)).strong());
							ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |r_ui| {
								if r_ui.small_button("▶").on_hover_text("Następna osoba").clicked() {
									viewer.selected_person_idx = (viewer.selected_person_idx + 1) % persons.len();
								}
								if r_ui.small_button("◀").on_hover_text("Poprzednia osoba").clicked() {
									viewer.selected_person_idx =
										viewer.selected_person_idx.checked_sub(1).unwrap_or(persons.len() - 1);
								}
							});
						});

						cam_ui.separator();

						let (name, birth, tag) = persons[viewer.selected_person_idx];

						// Karta podglądu wybranej osoby
						cam_ui.vertical(|ui| {
							ui.label(egui::RichText::new(format!("👤 {name}")).size(15.0).strong());
							ui.label(egui::RichText::new(format!("📍 {birth}")).weak().size(12.0));
							ui.add_space(4.0);
							ui.label(
								egui::RichText::new(format!("🏷️ {tag}")).color(egui::Color32::from_rgb(0, 180, 255)),
							);
						});

						cam_ui.add_space(6.0);

						// Rozwijany ComboBox do bezpośredniego wyboru z listy
						egui::ComboBox::from_id_salt(format!("combo_cam_{}", viewer.id_num))
							.selected_text(name)
							.width(cam_ui.available_width() - 8.0)
							.show_ui(cam_ui, |ui| {
								for (idx, (p_name, _, _)) in persons.iter().enumerate() {
									ui.selectable_value(&mut viewer.selected_person_idx, idx, *p_name);
								}
							});
					});
				}

				// Automatyczne czyszczenie okien zamkniętych krzyżykiem [✕]
				self.viewers.retain(|v| v.ora.apertum);
			}
		}
	}

	pub mod app_style {
		use egui_knex::{egui, pos2};

		pub struct AppStyle;

		impl AppStyle {
			pub fn draw_canvas_background(&self, ui: &mut egui::Ui) {
				let rect = ui.available_rect_before_wrap();
				ui.painter().rect_filled(rect, 0.0, super::app_theme::CANVAS_BG);

				let step = 32.0;
				let stroke = egui::Stroke::new(1.0, super::app_theme::GRID_LINE);

				let mut x = rect.left();
				while x <= rect.right() {
					ui.painter().line_segment([pos2(x, rect.top()), pos2(x, rect.bottom())], stroke);
					x += step;
				}

				let mut y = rect.top();
				while y <= rect.bottom() {
					ui.painter().line_segment([pos2(rect.left(), y), pos2(rect.right(), y)], stroke);
					y += step;
				}
			}
		}
	}
}
