// 🦀 src/element/ora/mod.rs

pub mod header;
pub mod modus;

pub use header::OraRollIcon;
use header::draw_header;
pub use modus::OraModus;

use crate::egui::{self, Align2, Color32, Frame, Id, Pos2, Rect, Sense, Vec2, pos2, vec2};

/// Trait konwersji typów na identyfikator okna `Id`.
pub trait IntoOraId {
	fn into_ora_id(self) -> Id;
}

impl IntoOraId for Id {
	#[inline]
	fn into_ora_id(self) -> Id { self }
}

impl IntoOraId for &str {
	#[inline]
	fn into_ora_id(self) -> Id { Id::unique(self) }
}

impl IntoOraId for &String {
	#[inline]
	fn into_ora_id(self) -> Id { Id::unique(self) }
}

impl IntoOraId for String {
	#[inline]
	fn into_ora_id(self) -> Id { Id::unique(self) }
}

/// Samodzielny klocek okna pływającego i modalnego (`Ora`).
#[derive(Debug, Clone, PartialEq)]
pub struct Ora {
	pub id: Id,
	pub titulus: String,
	pub modus: OraModus,
	pub apertum: bool,
	pub contractum: bool,
	pub mobilis: bool,
	pub mutabilis: bool,
	pub positio_default: Option<Pos2>,
	pub magnitudo_default: Option<Vec2>,
	pub magnitudo_min: Vec2,
	pub roll_icon: OraRollIcon,

	/// Krotka geometrii nagłówka: `(header_height, button_size, corner_radius)`
	pub dim_header_btn_radius: (f32, f32, f32),

	/// Krotka kolorów bazowych: `(header_bg, content_bg, header_text)`
	pub color_base: (Color32, Color32, Color32),

	/// Krotka kolorów przycisku zamykania: `(normal_bg, hover_bg,
	/// symbol_stroke)`
	pub color_close: (Color32, Color32, Color32),

	/// Krotka kolorów przycisku zwijania: `(normal_bg, hover_bg,
	/// symbol_stroke)`
	pub color_roll: (Color32, Color32, Color32),
}

impl Ora {
	/// Tworzy nową instancję okna o wskazanym identyfikatorze i tytule.
	#[must_use]
	pub fn nova(id: impl IntoOraId, titulus: impl Into<String>) -> Self {
		Self {
			id: id.into_ora_id(),
			titulus: titulus.into(),
			modus: OraModus::Fluctuans,
			apertum: true,
			contractum: false,
			mobilis: true,
			mutabilis: true,
			positio_default: None,
			magnitudo_default: Some(vec2(360.0, 240.0)),
			magnitudo_min: vec2(200.0, 80.0),
			roll_icon: OraRollIcon::Linea,
			dim_header_btn_radius: (26.0, 16.0, 6.0),
			color_base: (
				Color32::from_rgb(32, 34, 38),
				Color32::from_rgb(24, 25, 28),
				Color32::from_rgb(230, 230, 230),
			),
			color_close: (
				Color32::from_rgba_unmultiplied(237, 28, 36, 45),
				Color32::from_rgba_unmultiplied(237, 28, 36, 120),
				Color32::from_rgb(255, 120, 120),
			),
			color_roll: (
				Color32::from_rgba_unmultiplied(255, 200, 0, 45),
				Color32::from_rgba_unmultiplied(255, 200, 0, 120),
				Color32::from_rgb(255, 220, 100),
			),
		}
	}

	#[must_use]
	pub const fn modus(mut self, modus: OraModus) -> Self {
		self.modus = modus;
		self
	}

	#[must_use]
	pub const fn roll_icon(mut self, icon: OraRollIcon) -> Self {
		self.roll_icon = icon;
		self
	}

	#[must_use]
	pub const fn positio(mut self, pos: (f32, f32)) -> Self {
		self.positio_default = Some(pos2(pos.0, pos.1));
		self
	}

	#[must_use]
	pub const fn magnitudo(mut self, size: (f32, f32)) -> Self {
		self.magnitudo_default = Some(vec2(size.0, size.1));
		self
	}

	#[must_use]
	pub const fn mobilis(mut self, mobilis: bool) -> Self {
		self.mobilis = mobilis;
		self
	}

	#[must_use]
	pub const fn mutabilis(mut self, mutabilis: bool) -> Self {
		self.mutabilis = mutabilis;
		self
	}

	#[must_use]
	pub const fn geometria(mut self, dim_header_btn_radius: (f32, f32, f32)) -> Self {
		self.dim_header_btn_radius = dim_header_btn_radius;
		self
	}

	#[must_use]
	pub const fn colores(
		mut self,
		base: (Color32, Color32, Color32),
		close: (Color32, Color32, Color32),
		roll: (Color32, Color32, Color32),
	) -> Self {
		self.color_base = base;
		self.color_close = close;
		self.color_roll = roll;
		self
	}

	pub fn aperi(&mut self) { self.apertum = true; }
	pub fn claude(&mut self) { self.apertum = false; }
	pub fn commuta(&mut self) { self.apertum = !self.apertum; }

	/// Renderuje okno wraz z przekazanym domknięciem zawartości.
	/// Zwraca `Some(R)` z wynikiem domknięcia lub `None`, gdy okno jest
	/// zamknięte bądź zwinięte.
	pub fn show<R, F>(&mut self, ui: &mut egui::Ui, add_contents: F) -> Option<R>
	where F: FnOnce(&mut egui::Ui) -> R {
		if !self.apertum {
			return None;
		}

		let bounds = ui.max_rect();
		let screen_rect = ui.ctx().input(|i| i.raw.screen_rect.unwrap_or(Rect::EVERYTHING));

		match self.modus {
			OraModus::Fluctuans => self.render_fluctuans(ui, bounds, false, add_contents),
			OraModus::FluctuansObvelatum => {
				egui::Area::new(self.id.with("obvelatum_shield"))
					.order(egui::Order::Foreground)
					.fixed_pos(screen_rect.min)
					.show(ui.ctx(), |shield_ui| {
						shield_ui.allocate_exact_size(screen_rect.size(), Sense::click_and_drag());
						shield_ui.painter().rect_filled(screen_rect, 0.0, Color32::from_black_alpha(70));
					});

				self.render_fluctuans(ui, bounds, true, add_contents)
			}
			OraModus::CentraleObvelatum => {
				egui::Area::new(self.id.with("centrale_curtain"))
					.order(egui::Order::Foreground)
					.fixed_pos(screen_rect.min)
					.show(ui.ctx(), |curtain_ui| {
						curtain_ui.allocate_exact_size(screen_rect.size(), Sense::click_and_drag());
						curtain_ui.painter().rect_filled(screen_rect, 0.0, Color32::from_black_alpha(175));
					});

				self.render_centrale(ui, screen_rect, add_contents)
			}
		}
	}

	fn render_fluctuans<R, F>(
		&mut self,
		ui: &mut egui::Ui,
		bounds: Rect,
		is_elevated: bool,
		add_contents: F,
	) -> Option<R>
	where
		F: FnOnce(&mut egui::Ui) -> R,
	{
		let pos_id = self.id.with("pos");
		let mut current_pos = ui
			.data(|d| d.get_temp::<Pos2>(pos_id))
			.unwrap_or_else(|| self.positio_default.unwrap_or_else(|| bounds.center() - vec2(180.0, 120.0)));

		let min_size = if self.contractum {
			vec2(self.magnitudo_min.x, self.dim_header_btn_radius.0)
		} else {
			self.magnitudo_min
		};

		let mut win = egui::Window::new(&self.titulus)
			.id(self.id)
			.constrain_to(bounds)
			.title_bar(false)
			.resizable(self.mutabilis && !self.contractum)
			.movable(false)
			.frame(Frame::window(ui.style()).fill(self.color_base.1).inner_margin(0))
			.min_size(min_size);

		if is_elevated {
			win = win.order(egui::Order::Tooltip);
		}
		if let Some(sz) = self.magnitudo_default {
			win = win.default_size(sz);
		}

		let mut drag_delta = Vec2::ZERO;
		let mut content_result = None;

		win.current_pos(current_pos).show(ui.ctx(), |win_ui| {
			let action = draw_header(
				win_ui,
				self.id,
				&self.titulus,
				self.mobilis,
				self.contractum,
				self.roll_icon,
				self.dim_header_btn_radius,
				self.color_base,
				self.color_close,
				self.color_roll,
			);

			if action.close_clicked {
				self.apertum = false;
			}
			if action.toggle_contract {
				self.contractum = !self.contractum;
			}
			drag_delta = action.drag_delta;

			if !self.contractum {
				win_ui.add_space(2.0);
				Frame::new().inner_margin(8).show(win_ui, |content_ui| {
					content_result = Some(add_contents(content_ui));
				});
			}
		});

		current_pos += drag_delta;
		ui.data_mut(|d| d.insert_temp(pos_id, current_pos));
		content_result
	}

	fn render_centrale<R, F>(&mut self, ui: &mut egui::Ui, screen_rect: Rect, add_contents: F) -> Option<R>
	where F: FnOnce(&mut egui::Ui) -> R {
		let size = self.magnitudo_default.unwrap_or(vec2(420.0, 260.0));
		let mut content_result = None;

		egui::Area::new(self.id.with("centrale_body"))
			.order(egui::Order::Tooltip)
			.pivot(Align2::CENTER_CENTER)
			.fixed_pos(screen_rect.center())
			.show(ui.ctx(), |area_ui| {
				area_ui.set_width(size.x);
				Frame::window(area_ui.style()).fill(self.color_base.1).inner_margin(0).show(area_ui, |modal_ui| {
					let action = draw_header(
						modal_ui,
						self.id,
						&self.titulus,
						false,
						self.contractum,
						self.roll_icon,
						self.dim_header_btn_radius,
						self.color_base,
						self.color_close,
						self.color_roll,
					);

					if action.close_clicked {
						self.apertum = false;
					}
					if action.toggle_contract {
						self.contractum = !self.contractum;
					}

					if !self.contractum {
						modal_ui.add_space(2.0);
						Frame::new().inner_margin(12).show(modal_ui, |content_ui| {
							content_result = Some(add_contents(content_ui));
						});
					}
				});
			});

		content_result
	}
}
