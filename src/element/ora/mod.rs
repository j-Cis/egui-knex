// 🦀 src/element/ora/mod.rs

pub mod header;
pub mod modus;

pub use header::OraRollIcon;
use header::{OraHeaderAction, OraHeaderState, OraHeaderStyle, draw_header};
pub use modus::{OraButonStatus, OraModus};

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
	/// Czy okno jest aktualnie na pełnym ekranie.
	pub maximis: bool,
	/// Czy pokazać belkę tytułową. Bez belki okno nie ma uchwytu do
	/// przeciągania, więc nie da się go przesunąć.
	pub titulus_visibilis: bool,
	/// Stan przycisków belki: `(zamknij ✕, zwiń —, pełny ekran ⛶)`.
	pub butones: (OraButonStatus, OraButonStatus, OraButonStatus),
	/// Czy okno można przesuwać.
	pub mobilis: bool,
	/// Czy okno można zmieniać: `(szerokość, wysokość)`.
	pub mutabilis: (bool, bool),
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

	/// Krotka kolorów przycisku pełnego ekranu: `(normal_bg, hover_bg,
	/// symbol_stroke)`
	pub color_maximum: (Color32, Color32, Color32),

	/// Prostokąt okna sprzed pełnego ekranu (do przywrócenia).
	rect_ante_maximum: Option<Rect>,
	/// Ostatni znany prostokąt okna w trybie zwykłym.
	rect_ultimus: Option<Rect>,
	/// Jednoklatkowe wymuszenie prostokąta przy przywracaniu.
	restaurandum: Option<Rect>,
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
			maximis: false,
			titulus_visibilis: true,
			butones: (OraButonStatus::Visibilis, OraButonStatus::Visibilis, OraButonStatus::Occultus),
			mobilis: true,
			mutabilis: (true, true),
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
			color_maximum: (
				Color32::from_rgba_unmultiplied(80, 160, 255, 45),
				Color32::from_rgba_unmultiplied(80, 160, 255, 120),
				Color32::from_rgb(150, 200, 255),
			),
			rect_ante_maximum: None,
			rect_ultimus: None,
			restaurandum: None,
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

	/// Czy pokazać belkę tytułową (domyślnie tak).
	#[must_use]
	pub const fn titulus_visibilis(mut self, visibilis: bool) -> Self {
		self.titulus_visibilis = visibilis;
		self
	}

	/// Stan przycisków belki: `(zamknij ✕, zwiń —, pełny ekran ⛶)`.
	#[must_use]
	pub const fn butones(mut self, butones: (OraButonStatus, OraButonStatus, OraButonStatus)) -> Self {
		self.butones = butones;
		self
	}

	#[must_use]
	pub const fn mobilis(mut self, mobilis: bool) -> Self {
		self.mobilis = mobilis;
		self
	}

	/// Zmiana rozmiaru: `(szerokość, wysokość)`.
	#[must_use]
	pub const fn mutabilis(mut self, mutabilis: (bool, bool)) -> Self {
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

	/// Kolory przycisku pełnego ekranu: `(normal_bg, hover_bg, symbol_stroke)`.
	#[must_use]
	pub const fn colores_maximi(mut self, maximum: (Color32, Color32, Color32)) -> Self {
		self.color_maximum = maximum;
		self
	}

	pub fn aperi(&mut self) { self.apertum = true; }
	pub fn claude(&mut self) { self.apertum = false; }
	pub fn commuta(&mut self) { self.apertum = !self.apertum; }

	/// Przełącza pełny ekran: włącza, zapamiętując poprzedni prostokąt,
	/// albo przywraca zapamiętany stan.
	pub fn commuta_maximum(&mut self) {
		if self.maximis {
			self.maximis = false;
			self.restaurandum = self.rect_ante_maximum.take();
		} else {
			self.rect_ante_maximum = self.rect_ultimus;
			self.maximis = true;
		}
	}

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

		// Mgła wokół okna także wtedy, gdy okno jest maksymalizowane
		// z zablokowanym rozmiarem (obie osie zablokowane).
		let size_locked = !self.mutabilis.0 && !self.mutabilis.1;
		let fog = self.maximis && size_locked;

		match self.modus {
			OraModus::Fluctuans if !fog => self.render_fluctuans(ui, bounds, false, add_contents),
			OraModus::Fluctuans | OraModus::FluctuansObvelatum => {
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

		// Pełny ekran obejmuje cały ekran aplikacji, a nie tylko obszar
		// canvasu.
		let screen = ui.ctx().input(|i| i.raw.screen_rect.unwrap_or(Rect::EVERYTHING));
		let limit = if self.maximis { screen } else { bounds };

		// Zmiana rozmiaru nie działa w zwinięciu ani na pełnym ekranie.
		let can_resize = !self.contractum && !self.maximis;
		// Zmiana wysokości działa tylko poza zwinięciem i pełnym ekranem.
		let resize_h = self.mutabilis.1 && can_resize;

		let mut win = egui::Window::new(&self.titulus)
			.id(self.id)
			.constrain_to(limit)
			.title_bar(false)
			.resizable(egui::Vec2b::new(self.mutabilis.0 && can_resize, self.mutabilis.1 && can_resize))
			.movable(false)
			.frame(Frame::window(ui.style()).fill(self.color_base.1).inner_margin(0))
			.min_size(min_size);

		if is_elevated {
			win = win.order(egui::Order::Tooltip);
		}

		if self.maximis {
			// Pełny ekran: cały ekran w obu osiach (w zwinięciu tylko belka).
			let h = if self.contractum { self.dim_header_btn_radius.0 } else { screen.height() };
			win = win.fixed_rect(Rect::from_min_size(screen.min, vec2(screen.width(), h)));
		} else if let Some(rect) = self.restaurandum.take() {
			// Przywracanie: jednorazowe wymuszenie zapamiętanego prostokąta.
			current_pos = rect.min;
			let h = if self.contractum { self.dim_header_btn_radius.0 } else { rect.height() };
			win = win.fixed_rect(Rect::from_min_size(rect.min, vec2(rect.width(), h)));
		} else if let Some(sz) = self.magnitudo_default {
			win = win.default_size(sz);
		}

		let mut drag_delta = Vec2::ZERO;
		let mut content_result = None;
		let mut toggle_maximum = false;
		let style = self.header_style();

		let response = win.current_pos(current_pos).show(ui.ctx(), |win_ui| {
			// Wysokość wymuszana minimalnym rozmiarem treści (samo fixed_rect
			// nie rozciągało pionowo). Przy zwinięciu zostaje sama belka.
			if self.maximis {
				let h = if self.contractum { self.dim_header_btn_radius.0 } else { screen.height() };
				win_ui.set_min_size(vec2(screen.width(), h));
			}
			// egui bierze wysokość okna z treści; zmiana wysokości z
			// przeciągania zapisuje się tylko w pamięci `Resize`.
			// Minimalna wysokość równa dostępnej (czyli
			// zapamiętanej) pozwala przeciąganiu zadziałać.
			if resize_h {
				win_ui.set_min_height(win_ui.available_height());
			}
			let action = if self.titulus_visibilis {
				draw_header(
					win_ui,
					OraHeaderState {
						window_id: self.id,
						title: &self.titulus,
						is_movable: self.mobilis && !self.maximis,
						is_contracted: self.contractum,
						is_maximized: self.maximis,
						butones: self.butones,
						roll_icon: self.roll_icon,
					},
					style,
				)
			} else {
				OraHeaderAction::default()
			};

			if action.close_clicked {
				self.apertum = false;
			}
			if action.toggle_contract {
				self.contractum = !self.contractum;
			}
			if action.toggle_maximize {
				toggle_maximum = true;
			}
			// Zwinięcie w pełnym ekranie wychodzi z pełnego ekranu, żeby belka
			// miała zwykły rozmiar i dało się ją przesuwać jak inne belki.
			if action.toggle_contract && self.contractum && self.maximis {
				toggle_maximum = true;
			}
			drag_delta = action.drag_delta;

			if !self.contractum {
				win_ui.add_space(2.0);
				Frame::new().inner_margin(8).show(win_ui, |content_ui| {
					content_result = Some(add_contents(content_ui));
				});
			}
		});

		if let Some(inner) = &response
			&& !self.maximis
		{
			self.rect_ultimus = Some(inner.response.rect);
		}

		if toggle_maximum {
			self.commuta_maximum();
		}

		current_pos += drag_delta;
		ui.data_mut(|d| d.insert_temp(pos_id, current_pos));
		content_result
	}

	fn render_centrale<R, F>(&mut self, ui: &mut egui::Ui, screen_rect: Rect, add_contents: F) -> Option<R>
	where F: FnOnce(&mut egui::Ui) -> R {
		let size = self.magnitudo_default.unwrap_or(vec2(420.0, 260.0));
		let maximum = self.maximis;
		let style = self.header_style();
		let mut content_result = None;
		let mut toggle_maximum = false;

		let (pivot, pos) = if maximum {
			(Align2::LEFT_TOP, screen_rect.min)
		} else {
			(Align2::CENTER_CENTER, screen_rect.center())
		};

		egui::Area::new(self.id.with("centrale_body")).order(egui::Order::Tooltip).pivot(pivot).fixed_pos(pos).show(
			ui.ctx(),
			|area_ui| {
				if maximum {
					area_ui.set_min_size(screen_rect.size());
				} else {
					area_ui.set_width(size.x);
				}
				Frame::window(area_ui.style()).fill(self.color_base.1).inner_margin(0).show(area_ui, |modal_ui| {
					if maximum && !self.contractum {
						modal_ui.set_min_size(screen_rect.size());
					}

					let action = if self.titulus_visibilis {
						draw_header(
							modal_ui,
							OraHeaderState {
								window_id: self.id,
								title: &self.titulus,
								is_movable: false,
								is_contracted: self.contractum,
								is_maximized: maximum,
								butones: self.butones,
								roll_icon: self.roll_icon,
							},
							style,
						)
					} else {
						OraHeaderAction::default()
					};

					if action.close_clicked {
						self.apertum = false;
					}
					if action.toggle_contract {
						self.contractum = !self.contractum;
					}
					if action.toggle_maximize {
						toggle_maximum = true;
					}

					if action.toggle_contract && self.contractum && maximum {
						toggle_maximum = true;
					}

					if !self.contractum {
						modal_ui.add_space(2.0);
						Frame::new().inner_margin(12).show(modal_ui, |content_ui| {
							content_result = Some(add_contents(content_ui));
						});
					}
				});
			},
		);

		if toggle_maximum {
			self.commuta_maximum();
		}

		content_result
	}

	fn header_style(&self) -> OraHeaderStyle {
		OraHeaderStyle {
			dim_header_btn_radius: self.dim_header_btn_radius,
			color_base: self.color_base,
			color_close: self.color_close,
			color_roll: self.color_roll,
			color_maximum: self.color_maximum,
		}
	}
}
