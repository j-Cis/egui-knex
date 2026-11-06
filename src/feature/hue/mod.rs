// 🦀 src/feature/hue/mod.rs

//! Przestrzenie barw Oklab i Oklch (Björn Ottosson, 2020) dla `egui::Color32`.
//!
//! Dwie warstwy:
//! - funkcje `const` ([`oklab`], [`oklch`]) — użyteczne w stałych i w `const fn`,
//! - rozszerzenie [`Color32OklabExt`] — składnia `Color32::from_oklch(..)` oraz
//!   konwersja odwrotna (`to_oklab`, `to_oklch`).
//!
//! Wynik zawsze jest w sRGB z 8 bitami na kanał, bo tym jest `Color32`.
//! Kolory spoza gamutu sRGB są sprowadzane algorytmem „CSS gamut mapping”
//! (CSS Color 4): redukcja chromy przy stałych `L` i odcieniu, aż różnica
//! `ΔEok` między kolorem a jego obcięciem spadnie poniżej progu 0.02.

#![allow(clippy::excessive_precision)]

use std::f32::consts::{FRAC_PI_2, PI};

use crate::egui::Color32;

/// Próg „nie do odróżnienia” w `ΔEok` (CSS Color 4: `JND = 0.02`).
const JND: f32 = 0.02;

/// Dokładność bisekcji chromy (CSS Color 4: `ε = 0.0001`).
const CHROMA_EPS: f32 = 0.0001;

/// Tolerancja błędu zmiennoprzecinkowego na granicach gamutu sRGB.
const GAMUT_EPS: f32 = 1e-5;

/// Liczba kroków bisekcji dla pierwiastków (dokładność f32).
const ROOT_ITERS: u32 = 40;

/// Ogranicza wartość do przedziału `[0, 1]`.
const fn clamp01(v: f32) -> f32 {
	if v < 0.0 {
		0.0
	} else if v > 1.0 {
		1.0
	} else {
		v
	}
}

/// `base^degree` dla małej, dodatniej potęgi całkowitej.
const fn pow_uint(base: f32, degree: u32) -> f32 {
	let mut acc = 1.0_f32;
	let mut i = 0;
	while i < degree {
		acc *= base;
		i += 1;
	}
	acc
}

/// Pierwiastek `degree`-tego stopnia z `x ≥ 0`, przez bisekcję (działa w `const`).
const fn root_of(x: f32, degree: u32) -> f32 {
	if x <= 0.0 {
		return 0.0;
	}
	let mut lo = 0.0_f32;
	let mut hi = if x > 1.0 { x } else { 1.0 };
	let mut i = 0;
	while i < ROOT_ITERS {
		let mid = (lo + hi) * 0.5;
		if pow_uint(mid, degree) < x {
			lo = mid;
		} else {
			hi = mid;
		}
		i += 1;
	}
	(lo + hi) * 0.5
}

/// `x^(5/12)` (czyli `x^(1/2.4)` z krzywej gamma sRGB), przez bisekcję dla `y^12 = x`.
const fn pow_five_twelfths(x: f32) -> f32 {
	let y = root_of(x, 12);
	let y2 = y * y;
	let y4 = y2 * y2;
	y4 * y
}

/// Konwersja Oklab → liniowe sRGB, bez obcinania.
const fn oklab_to_linear_srgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
	let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
	let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
	let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

	let ll = l_ * l_ * l_;
	let mm = m_ * m_ * m_;
	let ss = s_ * s_ * s_;

	let r = 4.0767416621 * ll - 3.3077115913 * mm + 0.2309699292 * ss;
	let g = -1.2684380046 * ll + 2.6097574011 * mm - 0.3413193965 * ss;
	let b = -0.0041960863 * ll - 0.7034186147 * mm + 1.7076147010 * ss;

	(r, g, b)
}

/// Konwersja liniowe sRGB → Oklab (dla wartości w `[0, 1]`).
const fn linear_srgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
	let l = root_of(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b, 3);
	let m = root_of(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b, 3);
	let s = root_of(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b, 3);

	(
		0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
		1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
		0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
	)
}

/// Czy liniowe składowe mieszczą się w gamucie sRGB (z tolerancją `GAMUT_EPS`).
const fn is_in_srgb_gamut(r: f32, g: f32, b: f32) -> bool {
	let lo = -GAMUT_EPS;
	let hi = 1.0 + GAMUT_EPS;
	r >= lo && r <= hi && g >= lo && g <= hi && b >= lo && b <= hi
}

/// Obcięcie liniowego sRGB do `[0, 1]` (krok „clip” z CSS).
const fn clip_rgb(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
	(clamp01(r), clamp01(g), clamp01(b))
}

/// Odległość `ΔEok` między dwoma kolorami Oklab.
const fn delta_eok(p: (f32, f32, f32), q: (f32, f32, f32)) -> f32 {
	let dl = p.0 - q.0;
	let da = p.1 - q.1;
	let db = p.2 - q.2;
	root_of(dl * dl + da * da + db * db, 2)
}

/// Mapowanie do gamutu sRGB algorytmem CSS Color 4 (bisekcja chromy w OkLCh).
///
/// Zwraca liniowe sRGB w `[0, 1]`. Gdy kolor jest już w gamucie, zwraca go bez zmian.
/// Gdy obcięcie jest nie do odróżnienia od oryginału (`ΔEok < 0.02`), zwraca obcięcie.
/// W przeciwnym razie zmniejsza chromę, zachowując `L` i odcień, do najwyższej
/// wartości, przy której kolor jest w gamucie albo różni się od obcięcia mniej niż progiem.
const fn map_to_gamut(l: f32, c: f32, cos_h: f32, sin_h: f32) -> (f32, f32, f32) {
	let (r, g, b) = oklab_to_linear_srgb(l, c * cos_h, c * sin_h);
	if is_in_srgb_gamut(r, g, b) {
		return (r, g, b);
	}

	let origin = (l, c * cos_h, c * sin_h);
	let mut clipped = clip_rgb(r, g, b);
	let e = delta_eok(origin, linear_srgb_to_oklab(clipped.0, clipped.1, clipped.2));
	if e < JND {
		return clipped;
	}

	let mut min = 0.0_f32;
	let mut max = c;
	let mut min_in_gamut = true;

	while max - min > CHROMA_EPS {
		let chroma = (min + max) * 0.5;
		let (r, g, b) = oklab_to_linear_srgb(l, chroma * cos_h, chroma * sin_h);

		if min_in_gamut && is_in_srgb_gamut(r, g, b) {
			min = chroma;
			continue;
		}

		clipped = clip_rgb(r, g, b);
		let current = (l, chroma * cos_h, chroma * sin_h);
		let e = delta_eok(current, linear_srgb_to_oklab(clipped.0, clipped.1, clipped.2));

		if e < JND {
			if JND - e < CHROMA_EPS {
				return clipped;
			}
			min_in_gamut = false;
			min = chroma;
		} else {
			max = chroma;
		}
	}

	clipped
}

/// Bajt sRGB z liniowej składowej: krzywa gamma i zaokrąglenie.
const fn channel(linear: f32) -> u8 {
	let x = clamp01(linear);
	let srgb = if x <= 0.0031308 { 12.92 * x } else { 1.055 * pow_five_twelfths(x) - 0.055 };
	(srgb * 255.0 + 0.5) as u8
}

/// `(sin, cos)` kąta w stopniach, bez `std` (dla `const`).
///
/// Redukcja do `[-π/2, π/2]`, potem szereg Taylora (wystarczający dla f32).
const fn sin_cos_degrees(degrees: f32) -> (f32, f32) {
	let mut angle = degrees % 360.0;
	if angle < 0.0 {
		angle += 360.0;
	}
	if angle > 180.0 {
		angle -= 360.0;
	}

	let mut x = angle * PI / 180.0;
	let mut cos_sign = 1.0_f32;
	if x > FRAC_PI_2 {
		x = PI - x;
		cos_sign = -1.0;
	} else if x < -FRAC_PI_2 {
		x = -PI - x;
		cos_sign = -1.0;
	}

	let x2 = x * x;
	let sin = x
		* (1.0
			+ x2 * (-1.0 / 6.0
				+ x2 * (1.0 / 120.0
					+ x2 * (-1.0 / 5040.0
						+ x2 * (1.0 / 362880.0 + x2 * (-1.0 / 39916800.0 + x2 * (1.0 / 6227020800.0)))))));
	let cos = 1.0
		+ x2 * (-1.0 / 2.0
			+ x2 * (1.0 / 24.0
				+ x2 * (-1.0 / 720.0
					+ x2 * (1.0 / 40320.0
						+ x2 * (-1.0 / 3628800.0 + x2 * (1.0 / 479001600.0 + x2 * (-1.0 / 87178291200.0)))))));

	(sin, cos * cos_sign)
}

/// Kolor z postaci biegunowej Oklch (`cos_h`, `sin_h` odcienia już policzone).
const fn from_polar(l: f32, c: f32, cos_h: f32, sin_h: f32) -> Color32 {
	if l <= 0.0 {
		return Color32::BLACK;
	}
	let (r, g, b) = map_to_gamut(clamp01(l), c, cos_h, sin_h);
	Color32::from_rgb(channel(r), channel(g), channel(b))
}

/// Oklab (`L` ∈ [0, 1], `a`, `b`) → `Color32` w sRGB.
///
/// Działa w czasie kompilacji. `L ≤ 0` daje czerń, `L > 1` jest obcinane do 1.
/// Kolory spoza sRGB są sprowadzane do gamutu (patrz moduł).
#[must_use]
pub const fn oklab(l: f32, a: f32, b: f32) -> Color32 {
	let c = root_of(a * a + b * b, 2);
	if c > 0.0 { from_polar(l, c, a / c, b / c) } else { from_polar(l, 0.0, 1.0, 0.0) }
}

/// Oklch (`L` ∈ [0, 1], chroma `c`, odcień `h` w stopniach) → `Color32` w sRGB.
///
/// Działa w czasie kompilacji. Ujemna chroma traktowana jest jak zero.
#[must_use]
pub const fn oklch(l: f32, c: f32, h: f32) -> Color32 {
	if c <= 0.0 {
		return from_polar(l, 0.0, 1.0, 0.0);
	}
	let (sin_h, cos_h) = sin_cos_degrees(h);
	from_polar(l, c, cos_h, sin_h)
}

/// Konwersje Oklab / Oklch dla `egui::Color32`.
///
/// Trait nie może mieć metod `const`, więc wersje `const` są w [`oklab`] i
/// [`oklch`]. Tu jest wygodna składnia i konwersja odwrotna.
pub trait Color32OklabExt: Sized {
	/// Kolor z Oklab (`l` ∈ [0, 1]).
	fn from_oklab(l: f32, a: f32, b: f32) -> Self;

	/// Kolor z Oklch (`c` — chroma, `h` — odcień w stopniach).
	fn from_oklch(l: f32, c: f32, h: f32) -> Self;

	/// Składowe Oklab tego koloru: `(L, a, b)`.
	fn to_oklab(self) -> (f32, f32, f32);

	/// Składowe Oklch tego koloru: `(L, C, h)`, `h` w stopniach z przedziału `[0, 360)`.
	fn to_oklch(self) -> (f32, f32, f32);
}

impl Color32OklabExt for Color32 {
	fn from_oklab(l: f32, a: f32, b: f32) -> Self {
		oklab(l, a, b)
	}

	fn from_oklch(l: f32, c: f32, h: f32) -> Self {
		oklch(l, c, h)
	}

	fn to_oklab(self) -> (f32, f32, f32) {
		let r = srgb_to_linear(f32::from(self.r()) / 255.0);
		let g = srgb_to_linear(f32::from(self.g()) / 255.0);
		let b = srgb_to_linear(f32::from(self.b()) / 255.0);

		let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
		let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
		let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();

		(
			0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
			1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
			0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
		)
	}

	fn to_oklch(self) -> (f32, f32, f32) {
		let (l, a, b) = self.to_oklab();
		let c = a.hypot(b);
		let mut h = b.atan2(a).to_degrees();
		// Dodatnie przesunięcie i porównanie zamiast `rem_euclid`: dla -1e-7 `rem_euclid` daje 360.0.
		if h < 0.0 {
			h += 360.0;
		}
		if h >= 360.0 {
			h -= 360.0;
		}
		(l, c, h)
	}
}

/// Odwrotność krzywej gamma sRGB (`0..1` → liniowe).
fn srgb_to_linear(v: f32) -> f32 {
	if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Porównanie kolorów z tolerancją na kanał.
	fn assert_rgb_close(actual: Color32, expected: (u8, u8, u8), tol: u8) {
		let ok = actual.r().abs_diff(expected.0) <= tol
			&& actual.g().abs_diff(expected.1) <= tol
			&& actual.b().abs_diff(expected.2) <= tol;
		assert!(ok, "wynik {actual:?}, oczekiwano {expected:?} (tol {tol})");
	}

	#[test]
	fn const_fns_work_at_compile_time() {
		const RED: Color32 = oklch(0.6280, 0.2577, 29.23);
		const GREY: Color32 = oklab(0.5, 0.0, 0.0);
		assert_rgb_close(RED, (255, 0, 0), 1);
		assert_eq!(GREY.r(), GREY.g());
		assert_eq!(GREY.g(), GREY.b());
	}

	#[test]
	fn white_black_and_clamping() {
		assert_rgb_close(oklab(1.0, 0.0, 0.0), (255, 255, 255), 0);
		assert_eq!(oklab(0.0, 0.3, 0.3), Color32::BLACK);
		assert_eq!(oklab(-0.5, 0.0, 0.0), Color32::BLACK);
		assert_rgb_close(oklab(1.2, 0.0, 0.0), (255, 255, 255), 0);
	}

	#[test]
	fn known_reference_values_for_red_and_white() {
		let (l, a, b) = Color32::from_rgb(255, 0, 0).to_oklab();
		assert!((l - 0.62796).abs() < 1e-3, "L = {l}");
		assert!((a - 0.22486).abs() < 1e-3, "a = {a}");
		assert!((b - 0.12585).abs() < 1e-3, "b = {b}");

		let (lw, aw, bw) = Color32::WHITE.to_oklab();
		assert!((lw - 1.0).abs() < 1e-3 && aw.abs() < 1e-3 && bw.abs() < 1e-3);
	}

	#[test]
	fn round_trip_srgb_grid() {
		for r in (0..=255).step_by(17) {
			for g in (0..=255).step_by(17) {
				for b in (0..=255).step_by(17) {
					let c = Color32::from_rgb(r, g, b);
					let (l, a, bb) = c.to_oklab();
					assert_rgb_close(Color32::from_oklab(l, a, bb), (r, g, b), 1);
				}
			}
		}
	}

	#[test]
	fn in_gamut_reference_teal() {
		// Wartość referencyjna z zewnętrznego pickera Oklch: oklch(33% 0.055 195) = #033e3e.
		assert_rgb_close(oklch(0.33, 0.055, 195.0), (3, 62, 62), 1);
	}

	#[test]
	fn out_of_gamut_reference_magenta() {
		// Wartość referencyjna z zewnętrznego pickera Oklch: oklch(66% 0.35 360) = #ff008b.
		assert_rgb_close(oklch(0.66, 0.35, 360.0), (255, 0, 139), 3);
	}

	#[test]
	fn oklch_is_polar_form_of_oklab() {
		let (l, c, h): (f32, f32, f32) = (0.7, 0.1, 200.0);
		let rad = h.to_radians();
		assert_eq!(oklch(l, c, h), oklab(l, c * rad.cos(), c * rad.sin()));
	}

	#[test]
	fn negative_chroma_is_grey_and_hue_wraps() {
		assert_eq!(oklch(0.5, -0.1, 45.0), oklch(0.5, 0.0, 45.0));
		assert_eq!(oklch(0.5, 0.1, 405.0), oklch(0.5, 0.1, 45.0));
		assert_eq!(oklch(0.5, 0.1, -315.0), oklch(0.5, 0.1, 45.0));
	}

	#[test]
	fn to_oklch_hue_is_in_range() {
		// Czysta czerwień ma odcień blisko 29°; wartości bliskie 0° nie mogą dać 360°.
		for c in [Color32::from_rgb(255, 0, 1), Color32::from_rgb(255, 0, 0), Color32::from_rgb(255, 1, 0)] {
			let (_, _, h) = c.to_oklch();
			assert!((0.0..360.0).contains(&h), "h = {h}");
		}
	}

	#[test]
	fn sin_cos_matches_std() {
		let mut deg = -720.0_f32;
		while deg <= 720.0 {
			let (s, c) = sin_cos_degrees(deg);
			let r = deg.to_radians();
			assert!((s - r.sin()).abs() < 1e-5, "sin({deg}) = {s}");
			assert!((c - r.cos()).abs() < 1e-5, "cos({deg}) = {c}");
			deg += 7.5;
		}
	}

	#[test]
	fn pow_five_twelfths_matches_powf() {
		let mut x = 0.0031309_f32;
		while x <= 1.0 {
			assert!((pow_five_twelfths(x) - x.powf(5.0 / 12.0)).abs() < 1e-5, "x = {x}");
			x += 0.01;
		}
	}

	#[test]
	fn out_of_gamut_keeps_lightness_and_roughly_hue() {
		// CSS: obcięcie kanałów może przesunąć odcień o kilka stopni; L jest zachowywane.
		let c = oklch(0.9, 0.4, 265.0);
		let (l, _, h) = c.to_oklch();
		assert!((l - 0.9).abs() < 0.02, "L = {l}");
		let dh = (h - 265.0 + 540.0).rem_euclid(360.0) - 180.0;
		assert!(dh.abs() < 10.0, "odcień zmieniony o {dh}°");
	}
}
