// 🦀 src/element/ora/header.rs

use super::modus::OraButonStatus;
use crate::egui::{
	self, Color32, CursorIcon, Frame, Id, Margin, Painter, Rect, Sense, Stroke, StrokeKind, Vec2, pos2, vec2,
};

/// Wariant graficzny kontrolki zwijania / minimalizacji okna.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OraRollIcon {
	/// Klasyczna pozioma kreska `—`.
	#[default]
	Linea,

	/// Wektorowy trójkąt kierunkowy (`▲` gdy otwarte, `▼` gdy zwinięte).
	Triangulum,
}

/// Wynik interakcji z belką tytułową okna.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct OraHeaderAction {
	pub drag_delta: Vec2,
	pub close_clicked: bool,
	pub toggle_contract: bool,
	pub toggle_maximize: bool,
}

/// Parametry wyglądu nagłówka, przekazywane z `Ora`.
#[derive(Debug, Clone, Copy)]
pub struct OraHeaderStyle {
	pub dim_header_btn_radius: (f32, f32, f32),
	pub color_base: (Color32, Color32, Color32),
	pub color_close: (Color32, Color32, Color32),
	pub color_roll: (Color32, Color32, Color32),
	pub color_maximum: (Color32, Color32, Color32),
}

/// Stan okna potrzebny do narysowania belki tytułowej.
#[derive(Debug, Clone, Copy)]
pub struct OraHeaderState<'a> {
	pub window_id: Id,
	pub title: &'a str,
	pub is_movable: bool,
	pub is_contracted: bool,
	pub is_maximized: bool,
	/// Stan przycisków: `(zamknij, zwiń, pełny ekran)`.
	pub butones: (OraButonStatus, OraButonStatus, OraButonStatus),
	pub roll_icon: OraRollIcon,
}

pub fn draw_header(ui: &mut egui::Ui, state: OraHeaderState<'_>, style: OraHeaderStyle) -> OraHeaderAction {
	let mut action = OraHeaderAction::default();
	let (header_h, btn_sz, corner_radius) = style.dim_header_btn_radius;
	let (header_bg, _content_bg, header_text) = style.color_base;
	let (close_status, roll_status, maximum_status) = state.butones;

	// Dostępna szerokość liczona PRZED ramką: ramka sama kurczy się do treści,
	// dlatego tło paska trzeba rozciągnąć ręcznie (poniżej).
	let full_w = ui.available_width();
	let margin_x = 8.0;

	let header_frame = Frame::new().fill(header_bg).inner_margin(Margin::symmetric(margin_x as i8, 4)).corner_radius(
		egui::CornerRadius {
			nw: corner_radius as u8,
			ne: corner_radius as u8,
			sw: if state.is_contracted { corner_radius as u8 } else { 0 },
			se: if state.is_contracted { corner_radius as u8 } else { 0 },
		},
	);

	let mut left_buttons_max_x = f32::MIN;

	let header_response = header_frame
		.show(ui, |header_ui| {
			// Wypełnij całą szerokość okna (minus marginesy ramki).
			header_ui.set_min_width(full_w - 2.0 * margin_x);
			header_ui.set_min_height(header_h);
			header_ui.horizontal(|ui| {
				ui.spacing_mut().item_spacing.x = 4.0;

				// Kontrolki po lewej stronie: [✕] [— / ▲] [⛶]
				if close_status != OraButonStatus::Occultus {
					let (clicked, rect) =
						draw_button(ui, btn_sz, close_status, style.color_close, "Zamknij", paint_close);
					action.close_clicked = clicked;
					left_buttons_max_x = left_buttons_max_x.max(rect.max.x);
				}

				if roll_status != OraButonStatus::Occultus {
					let tooltip = if state.is_contracted { "Rozwiń" } else { "Zwiń" };
					let (clicked, rect) = draw_button(ui, btn_sz, roll_status, style.color_roll, tooltip, |p, r, c| {
						paint_roll(p, r, c, state.is_contracted, state.roll_icon);
					});
					action.toggle_contract = clicked;
					left_buttons_max_x = left_buttons_max_x.max(rect.max.x);
				}

				if maximum_status != OraButonStatus::Occultus {
					let tooltip = if state.is_maximized { "Przywróć rozmiar" } else { "Pełny ekran" };
					let (clicked, rect) =
						draw_button(ui, btn_sz, maximum_status, style.color_maximum, tooltip, |p, r, c| {
							paint_maximize(p, r, c, state.is_maximized);
						});
					action.toggle_maximize = clicked;
					left_buttons_max_x = left_buttons_max_x.max(rect.max.x);
				}

				ui.add_space(4.0);

				// Tytuł okna
				// Tytuł nie jest zaznaczalny: belka służy wyłącznie do
				// przeciągania.
				ui.add(
					egui::Label::new(egui::RichText::new(state.title).color(header_text).strong()).selectable(false),
				);
			});
		})
		.response;

	let header_rect = header_response.rect;

	// Strefa przeciągania obejmuje wyłącznie wolną przestrzeń za przyciskami
	let drag_min_x = if left_buttons_max_x > f32::MIN { left_buttons_max_x + 2.0 } else { header_rect.min.x };

	if state.is_movable && header_rect.max.x > drag_min_x {
		let drag_rect =
			Rect::from_min_max(pos2(drag_min_x, header_rect.min.y), pos2(header_rect.max.x, header_rect.max.y));
		let drag_resp = ui.interact(drag_rect, state.window_id.with("ora_hdr_drag"), Sense::drag());

		if drag_resp.dragged() || drag_resp.is_pointer_button_down_on() {
			ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
		} else if drag_resp.hovered() {
			ui.ctx().set_cursor_icon(CursorIcon::Grab);
		}

		action.drag_delta = drag_resp.drag_delta();
	}

	action
}

/// Wspólny szkielet przycisku belki. `Clausus` rysuje przycisk wyszarzony
/// i nie reaguje na klik; `Occultus` nie jest tu wywoływany.
fn draw_button(
	ui: &mut egui::Ui,
	btn_size: f32,
	status: OraButonStatus,
	colors: (Color32, Color32, Color32),
	tooltip: &str,
	paint_symbol: impl FnOnce(&Painter, Rect, Color32),
) -> (bool, Rect) {
	let (normal_bg, hover_bg, stroke_col) = colors;
	let locked = status == OraButonStatus::Clausus;
	let sense = if locked { Sense::hover() } else { Sense::click() };
	let (rect, resp) = ui.allocate_exact_size(Vec2::splat(btn_size), sense);

	let (bg, symbol_col) = if locked {
		(normal_bg.gamma_multiply(0.35), stroke_col.gamma_multiply(0.35))
	} else if resp.hovered() {
		(hover_bg, stroke_col)
	} else {
		(normal_bg, stroke_col)
	};

	ui.painter().rect_filled(rect, 4.0, bg);
	paint_symbol(ui.painter(), rect, symbol_col);

	let resp = if locked {
		resp.on_hover_text(format!("{tooltip} (zablokowane)"))
	} else {
		resp.on_hover_text(tooltip)
	};
	(resp.clicked(), rect)
}

fn paint_close(painter: &Painter, rect: Rect, col: Color32) {
	let pad = 3.5;
	let stroke = Stroke::new(1.4, col);
	painter.line_segment([rect.min + Vec2::splat(pad), rect.max - Vec2::splat(pad)], stroke);
	painter.line_segment(
		[pos2(rect.right() - pad, rect.top() + pad), pos2(rect.left() + pad, rect.bottom() - pad)],
		stroke,
	);
}

fn paint_roll(painter: &Painter, rect: Rect, col: Color32, is_contracted: bool, icon: OraRollIcon) {
	match icon {
		OraRollIcon::Linea => {
			let y = rect.center().y;
			painter.line_segment([pos2(rect.left() + 3.5, y), pos2(rect.right() - 3.5, y)], Stroke::new(1.4, col));
		}
		OraRollIcon::Triangulum => {
			let center = rect.center();
			let r = 3.8;
			let points = if is_contracted {
				// Skierowany w dół (gotowy do rozwinięcia)
				[
					pos2(center.x - r, center.y - r * 0.5),
					pos2(center.x + r, center.y - r * 0.5),
					pos2(center.x, center.y + r * 0.8),
				]
			} else {
				// Skierowany w górę (gotowy do zwinięcia)
				[
					pos2(center.x - r, center.y + r * 0.5),
					pos2(center.x + r, center.y + r * 0.5),
					pos2(center.x, center.y - r * 0.8),
				]
			};
			painter.add(egui::Shape::convex_polygon(points.to_vec(), col, Stroke::NONE));
		}
	}
}

/// Ikona: kwadrat, a w trybie przywracania dwa nachodzące kwadraty.
fn paint_maximize(painter: &Painter, rect: Rect, col: Color32, is_maximized: bool) {
	let stroke = Stroke::new(1.2, col);
	let inner = rect.shrink(3.5);

	if is_maximized {
		let off = 2.5;
		let size = inner.size() - vec2(off, off);
		let back = Rect::from_min_size(inner.min + vec2(off, 0.0), size);
		let front = Rect::from_min_size(inner.min + vec2(0.0, off), size);
		painter.rect_stroke(back, 0.0, stroke, StrokeKind::Inside);
		painter.rect_stroke(front, 0.0, stroke, StrokeKind::Inside);
	} else {
		painter.rect_stroke(inner, 0.0, stroke, StrokeKind::Inside);
	}
}
