// 🦀 src/element/ora/header.rs

use crate::egui::{self, Color32, CursorIcon, Frame, Id, Margin, Rect, Sense, Stroke, Vec2, pos2};

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
}

#[allow(clippy::too_many_arguments)]
pub fn draw_header(
	ui: &mut egui::Ui,
	window_id: Id,
	title: &str,
	is_movable: bool,
	is_contracted: bool,
	roll_icon: OraRollIcon,
	dim_header_btn_radius: (f32, f32, f32),
	color_base: (Color32, Color32, Color32),
	color_close: (Color32, Color32, Color32),
	color_roll: (Color32, Color32, Color32),
) -> OraHeaderAction {
	let mut action = OraHeaderAction::default();
	let (header_h, btn_sz, corner_radius) = dim_header_btn_radius;
	let (header_bg, _content_bg, header_text) = color_base;

	let header_frame =
		Frame::new().fill(header_bg).inner_margin(Margin::symmetric(8, 4)).corner_radius(egui::CornerRadius {
			nw: corner_radius as u8,
			ne: corner_radius as u8,
			sw: if is_contracted { corner_radius as u8 } else { 0 },
			se: if is_contracted { corner_radius as u8 } else { 0 },
		});

	let mut left_buttons_max_x = f32::MIN;

	let header_response = header_frame
		.show(ui, |header_ui| {
			header_ui.set_min_height(header_h);
			header_ui.horizontal(|ui| {
				ui.spacing_mut().item_spacing.x = 4.0;

				// Kontrolki po lewej stronie: [✕] [— / ▲]
				let (close_clicked, close_rect) = draw_close_button(ui, btn_sz, color_close);
				if close_clicked {
					action.close_clicked = true;
				}
				left_buttons_max_x = left_buttons_max_x.max(close_rect.max.x);

				let (roll_clicked, roll_rect) = draw_roll_button(ui, btn_sz, is_contracted, roll_icon, color_roll);
				if roll_clicked {
					action.toggle_contract = true;
				}
				left_buttons_max_x = left_buttons_max_x.max(roll_rect.max.x);

				ui.add_space(4.0);

				// Tytuł okna
				ui.label(egui::RichText::new(title).color(header_text).strong());
			});
		})
		.response;

	let header_rect = header_response.rect;

	// Strefa przeciągania obejmuje wyłącznie wolną przestrzeń za przyciskami
	let drag_min_x = if left_buttons_max_x > f32::MIN { left_buttons_max_x + 2.0 } else { header_rect.min.x };

	if is_movable && header_rect.max.x > drag_min_x {
		let drag_rect =
			Rect::from_min_max(pos2(drag_min_x, header_rect.min.y), pos2(header_rect.max.x, header_rect.max.y));
		let drag_resp = ui.interact(drag_rect, window_id.with("ora_hdr_drag"), Sense::drag());

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
