// 🦀 src/element/ora/header.rs

use crate::egui::{self, Color32, CursorIcon, Frame, Id, Margin, Rect, Sense, Stroke, StrokeKind, Vec2, pos2, vec2};

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

/// Parametry wyglądu i zachowania nagłówka, przekazywane z `Ora`.
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
	pub maximabilis: bool,
	pub roll_icon: OraRollIcon,
}

pub fn draw_header(ui: &mut egui::Ui, state: OraHeaderState<'_>, style: OraHeaderStyle) -> OraHeaderAction {
	let mut action = OraHeaderAction::default();
	let (header_h, btn_sz, corner_radius) = style.dim_header_btn_radius;
	let (header_bg, _content_bg, header_text) = style.color_base;

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
				let (close_clicked, close_rect) = draw_close_button(ui, btn_sz, style.color_close);
				if close_clicked {
					action.close_clicked = true;
				}
				left_buttons_max_x = left_buttons_max_x.max(close_rect.max.x);

				let (roll_clicked, roll_rect) =
					draw_roll_button(ui, btn_sz, state.is_contracted, state.roll_icon, style.color_roll);
				if roll_clicked {
					action.toggle_contract = true;
				}
				left_buttons_max_x = left_buttons_max_x.max(roll_rect.max.x);

				if state.maximabilis {
					let (max_clicked, max_rect) =
						draw_maximize_button(ui, btn_sz, state.is_maximized, style.color_maximum);
					if max_clicked {
						action.toggle_maximize = true;
					}
					left_buttons_max_x = left_buttons_max_x.max(max_rect.max.x);
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

fn draw_close_button(ui: &mut egui::Ui, btn_size: f32, colors: (Color32, Color32, Color32)) -> (bool, Rect) {
	let (normal_bg, hover_bg, stroke_col) = colors;
	let (rect, resp) = ui.allocate_exact_size(Vec2::splat(btn_size), Sense::click());

	let bg = if resp.hovered() { hover_bg } else { normal_bg };
	ui.painter().rect_filled(rect, 4.0, bg);

	let pad = 3.5;
	ui.painter().line_segment([rect.min + Vec2::splat(pad), rect.max - Vec2::splat(pad)], Stroke::new(1.4, stroke_col));
	ui.painter().line_segment(
		[pos2(rect.right() - pad, rect.top() + pad), pos2(rect.left() + pad, rect.bottom() - pad)],
		Stroke::new(1.4, stroke_col),
	);

	(resp.on_hover_text("Zamknij").clicked(), rect)
}

fn draw_roll_button(
	ui: &mut egui::Ui,
	btn_size: f32,
	is_contracted: bool,
	icon: OraRollIcon,
	colors: (Color32, Color32, Color32),
) -> (bool, Rect) {
	let (normal_bg, hover_bg, stroke_col) = colors;
	let (rect, resp) = ui.allocate_exact_size(Vec2::splat(btn_size), Sense::click());

	let bg = if resp.hovered() { hover_bg } else { normal_bg };
	ui.painter().rect_filled(rect, 4.0, bg);

	match icon {
		OraRollIcon::Linea => {
			let y = rect.center().y;
			ui.painter()
				.line_segment([pos2(rect.left() + 3.5, y), pos2(rect.right() - 3.5, y)], Stroke::new(1.4, stroke_col));
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
			ui.painter().add(egui::Shape::convex_polygon(points.to_vec(), stroke_col, Stroke::NONE));
		}
	}

	let tooltip = if is_contracted { "Rozwiń" } else { "Zwiń" };
	(resp.on_hover_text(tooltip).clicked(), rect)
}

/// Przycisk pełnego ekranu / przywracania. Ikona: kwadrat, a w trybie
/// przywracania dwa nachodzące kwadraty.
fn draw_maximize_button(
	ui: &mut egui::Ui,
	btn_size: f32,
	is_maximized: bool,
	colors: (Color32, Color32, Color32),
) -> (bool, Rect) {
	let (normal_bg, hover_bg, stroke_col) = colors;
	let (rect, resp) = ui.allocate_exact_size(Vec2::splat(btn_size), Sense::click());

	let bg = if resp.hovered() { hover_bg } else { normal_bg };
	ui.painter().rect_filled(rect, 4.0, bg);

	let stroke = Stroke::new(1.2, stroke_col);
	let inner = rect.shrink(3.5);

	if is_maximized {
		let off = 2.5;
		let size = inner.size() - vec2(off, off);
		let back = Rect::from_min_size(inner.min + vec2(off, 0.0), size);
		let front = Rect::from_min_size(inner.min + vec2(0.0, off), size);
		ui.painter().rect_stroke(back, 0.0, stroke, StrokeKind::Inside);
		ui.painter().rect_stroke(front, 0.0, stroke, StrokeKind::Inside);
	} else {
		ui.painter().rect_stroke(inner, 0.0, stroke, StrokeKind::Inside);
	}

	let tooltip = if is_maximized { "Przywróć rozmiar" } else { "Pełny ekran" };
	(resp.on_hover_text(tooltip).clicked(), rect)
}
