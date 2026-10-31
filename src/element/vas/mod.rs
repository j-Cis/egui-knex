// 🦀 src\element\vas\mod.rs

use eframe::egui;

/// Pomocniczy enum do precyzyjnego opisu rozpiętości (piksele lub procenty)
#[derive(Debug, Clone, Default)]
pub enum VasSpatium {
	Pixel(f32),
	Percent(f32),
	#[default]
	Auto,
}

#[derive(Debug, Clone)]
pub struct VasFovea {
	/// Deklarowana rozpiętość (np. w procentach lub pikselach)
	pub spatium: VasSpatium,

	/// Numer w kolejności (indeks gniazda na osi)
	pub numerus: usize,

	/// Może się ukrywać (flaga widoczności / toggle)
	pub latens: bool,

	/// Może się kurczyć (współczynnik kurczenia / shrink)
	pub contractio: f32,

	/// Może się rozszerzać (współczynnik rośnięcia / grow)
	pub amplificatio: f32,

	/// Ma wypełnić pozostałą przestrzeń (flaga typu flex-fill /
	/// auto-stretch)
	pub repletio: bool,
}

impl Default for VasFovea {
	fn default() -> Self {
		Self {
			spatium: VasSpatium::Auto,
			numerus: 0,
			latens: false,
			contractio: 0.0,
			amplificatio: 0.0,
			repletio: false,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VasRedundantia {
	/// Elementy wewnątrz ramy zmniejszają swój wymiar poprzeczny,
	/// aby cała zawartość idealnie upchnęła się w dostępnej przestrzeni
	/// (Ściskanie / Shrink).
	Compressio,

	/// Elementy zachowują oryginalny rozmiar i wykraczają poza krawędź
	/// ramy, umożliwiając użytkownikowi płynne przesuwanie widoku
	/// wzdłuż osi (Przewijanie / Scroll).
	Volutatio,

	/// Zawartość, która nie mieści się w granicach kontenera,
	/// zostaje twardo obcięta dokładnie na linii jego krawędzi (Ucięcie /
	/// Clip).
	Truncatio,

	/// Strumień elementów po napotkaniu krawędzi kontenera załamuje się,
	/// przenosząc nadmiarowe gniazda do nowo utworzonego rzędu lub linii
	/// (Zawijanie / Wrap).
	Flexio,

	/// Nadmiarowa zawartość zostaje pogrupowana w osobne, pełnowymiarowe
	/// ekrany, umożliwiając przełączanie się między nimi jak między
	/// kartami w przeglądarce (Stronicowanie / Tabs).
	Paginatio,
}

/// Wspólny rdzeń geometryczny i strukturalny dla każdego kontenera.
pub struct Vas {
	/// Orientacja osi głównej: true = pozioma, false = pionowa.
	pub axis_maior_horizontalis: bool,

	/// Wektor ruchu na osi głównej: false = Directus, true = Reversus.
	pub axis_maior_reversus: bool,

	/// Strategia obsługi przepełnienia (overflow) na osi głównej.
	pub redundantia_maior: VasRedundantia,

	/// Pakiet wartości geometrycznych layoutu (.0: p_am, .1: p_cm, .2:
	/// p_at, .3: p_ct, .4: gap)
	pub padding_am_cm_at_ct_gap: (f32, f32, f32, f32, f32),

	/// Zablokowana sekwencja gniazd ułożonych wzdłuż osi głównej.
	pub foveae: Box<[VasFovea]>,
}

impl Vas {
	/// Oblicza całkowitą wymaganą rozpiętość zawartości na osi głównej
	pub fn total_content_length(&self, rect: egui::Rect) -> f32 {
		let (am, cm, _at, _ct, gap) = self.padding_am_cm_at_ct_gap;
		let visible_foveae: Vec<&VasFovea> = self.foveae.iter().filter(|f| !f.latens).collect();
		if visible_foveae.is_empty() {
			return 0.0;
		}
		let total_gap = gap * (visible_foveae.len() - 1) as f32;
		let main_length = if self.axis_maior_horizontalis {
			(rect.width() - am - cm).max(0.0)
		} else {
			(rect.height() - am - cm).max(0.0)
		};
		let available_length = (main_length - total_gap).max(0.0);

		let mut allocated = 0.0;
		let mut auto_count = 0;
		for fovea in &visible_foveae {
			match fovea.spatium {
				VasSpatium::Pixel(px) => allocated += px,
				VasSpatium::Percent(pct) => allocated += available_length * (pct / 100.0),
				VasSpatium::Auto => auto_count += 1,
			}
		}

		if auto_count > 0 {
			// W Volutatio minimalny rozmiar gniazda Auto to 115 px (miejsce
			// na ikonę i tekst bez łamania wiersza)
			let min_auto = if self.redundantia_maior == VasRedundantia::Volutatio { 115.0 } else { 0.0 };
			let remaining = (available_length - allocated).max(0.0);
			let auto_size = (remaining / auto_count as f32).max(min_auto);
			allocated += auto_size * auto_count as f32;
		}

		allocated + total_gap + am + cm
	}

	pub fn resolve(&self, rect: egui::Rect) -> Vec<(usize, egui::Rect)> { self.resolve_scroll(rect, 0.0) }

	pub fn resolve_scroll(&self, rect: egui::Rect, scroll_offset: f32) -> Vec<(usize, egui::Rect)> {
		let (am, cm, at, ct, gap) = self.padding_am_cm_at_ct_gap;

		let mut inner_rect = rect;
		if self.axis_maior_horizontalis {
			inner_rect.min.x += am;
			inner_rect.max.x -= cm;
			inner_rect.min.y += at;
			inner_rect.max.y -= ct;
		} else {
			inner_rect.min.y += am;
			inner_rect.max.y -= cm;
			inner_rect.min.x += at;
			inner_rect.max.x -= ct;
		}

		let visible_foveae: Vec<&VasFovea> = self.foveae.iter().filter(|f| !f.latens).collect();
		if visible_foveae.is_empty() {
			return vec![];
		}

		let total_gap = gap * (visible_foveae.len() - 1) as f32;
		let main_length = if self.axis_maior_horizontalis { inner_rect.width() } else { inner_rect.height() };
		let cross_length = if self.axis_maior_horizontalis { inner_rect.height() } else { inner_rect.width() };

		let available_length = (main_length - total_gap).max(0.0);

		let mut sizes = vec![0.0; visible_foveae.len()];
		let mut allocated_length = 0.0;
		let mut auto_count = 0;

		for (i, fovea) in visible_foveae.iter().enumerate() {
			match fovea.spatium {
				VasSpatium::Pixel(px) => {
					sizes[i] = px;
					allocated_length += px;
				}
				VasSpatium::Percent(pct) => {
					let s = available_length * (pct / 100.0);
					sizes[i] = s;
					allocated_length += s;
				}
				VasSpatium::Auto => {
					auto_count += 1;
				}
			}
		}

		let remaining_length = (available_length - allocated_length).max(0.0);
		if auto_count > 0 {
			// W Volutatio gniazda zachowują oryginalny rozmiar i NIE MOGĄ
			// skurczyć się poniżej wymaganych 115 px
			let min_auto = if self.redundantia_maior == VasRedundantia::Volutatio { 115.0 } else { 0.0 };
			let auto_size = (remaining_length / auto_count as f32).max(min_auto);
			for (i, fovea) in visible_foveae.iter().enumerate() {
				if matches!(fovea.spatium, VasSpatium::Auto) {
					sizes[i] = auto_size;
					allocated_length += auto_size;
				}
			}
		}

		// WYŁĄCZNIE Compressio ma prawo zmniejszać wymiary, aby upchnąć
		// zawartość
		if allocated_length > available_length && self.redundantia_maior == VasRedundantia::Compressio {
			let scale = available_length / allocated_length;
			for size in &mut sizes {
				*size *= scale;
			}
		}

		let mut results = Vec::new();

		// W Volutatio punkt startowy jest przesuwany o scroll_offset
		let mut current_pos = if self.axis_maior_reversus {
			if self.axis_maior_horizontalis {
				inner_rect.max.x + scroll_offset
			} else {
				inner_rect.max.y + scroll_offset
			}
		} else {
			if self.axis_maior_horizontalis {
				inner_rect.min.x - scroll_offset
			} else {
				inner_rect.min.y - scroll_offset
			}
		};

		for (i, &size) in sizes.iter().enumerate() {
			let fovea = visible_foveae[i];

			let slot_rect = if self.axis_maior_horizontalis {
				let (min_x, max_x) = if self.axis_maior_reversus {
					(current_pos - size, current_pos)
				} else {
					(current_pos, current_pos + size)
				};
				egui::Rect::from_min_max(
					egui::pos2(min_x, inner_rect.min.y),
					egui::pos2(max_x, inner_rect.min.y + cross_length),
				)
			} else {
				let (min_y, max_y) = if self.axis_maior_reversus {
					(current_pos - size, current_pos)
				} else {
					(current_pos, current_pos + size)
				};
				egui::Rect::from_min_max(
					egui::pos2(inner_rect.min.x, min_y),
					egui::pos2(inner_rect.min.x + cross_length, max_y),
				)
			};

			results.push((fovea.numerus, slot_rect));

			if self.axis_maior_reversus {
				current_pos -= size + gap;
			} else {
				current_pos += size + gap;
			}
		}

		results
	}
}
