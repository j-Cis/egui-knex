// 🦀 src\element\vas\mod.rs

use std::slice;

use crate::egui::{Rect, Ui, UiBuilder, pos2};

/// Pomocniczy enum do precyzyjnego opisu rozpiętości (piksele lub procenty)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum VasSpatium {
	Pixel(f32),
	Percent(f32),
	#[default]
	Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VasFovea {
	/// Deklarowana rozpiętość (np. w procentach lub pikselach)
	pub spatium: VasSpatium,

	/// Numer w kolejności (indeks gniazda na osi)
	pub numerus: usize,

	/// Może się ukrywać (flaga widoczności / toggle)
	pub latens: bool,

	/// Może się kurczyć (współczynnik kurczenia / shrink).
	/// Zarezerwowane: `Vas::resolve` obecnie nie uwzględnia tego pola.
	pub contractio: f32,

	/// Może się rozszerzać (współczynnik rośnięcia / grow).
	/// Zarezerwowane: `Vas::resolve` obecnie nie uwzględnia tego pola.
	pub amplificatio: f32,

	/// Ma wypełnić pozostałą przestrzeń (flaga typu flex-fill /
	/// auto-stretch). Zarezerwowane: `Vas::resolve` obecnie nie uwzględnia
	/// tego pola (wypełnianie realizuje `VasSpatium::Auto`).
	pub repletio: bool,
}

impl VasFovea {
	/// Gniazdo o numerze `numerus` i rozpiętości `spatium` (reszta domyślnie).
	#[must_use]
	pub const fn new(numerus: usize, spatium: VasSpatium) -> Self {
		Self { spatium, numerus, latens: false, contractio: 0.0, amplificatio: 0.0, repletio: false }
	}

	/// Gniazdo `Auto` (dzieli pozostałą przestrzeń).
	#[must_use]
	pub const fn auto(numerus: usize) -> Self { Self::new(numerus, VasSpatium::Auto) }

	/// Gniazdo o stałej rozpiętości w pikselach.
	#[must_use]
	pub const fn pixel(numerus: usize, px: f32) -> Self { Self::new(numerus, VasSpatium::Pixel(px)) }

	/// Gniazdo o rozpiętości w procentach długości dostępnej na osi.
	#[must_use]
	pub const fn percent(numerus: usize, pct: f32) -> Self { Self::new(numerus, VasSpatium::Percent(pct)) }

	/// Ukrywa / pokazuje gniazdo (ukryte nie zajmuje miejsca).
	#[must_use]
	pub const fn latens(mut self, latens: bool) -> Self {
		self.latens = latens;
		self
	}

	/// Ustawia współczynnik kurczenia (zarezerwowane, zob. pole).
	#[must_use]
	pub const fn contractio(mut self, contractio: f32) -> Self {
		self.contractio = contractio;
		self
	}

	/// Ustawia współczynnik rośnięcia (zarezerwowane, zob. pole).
	#[must_use]
	pub const fn amplificatio(mut self, amplificatio: f32) -> Self {
		self.amplificatio = amplificatio;
		self
	}

	/// Ustawia flagę wypełniania (zarezerwowane, zob. pole).
	#[must_use]
	pub const fn repletio(mut self, repletio: bool) -> Self {
		self.repletio = repletio;
		self
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
	/// wzdłuż osi (Przewijanie / Scroll). Gniazda `Auto` mają tu minimalną
	/// rozpiętość [`Vas::VOLUTATIO_AUTO_MIN`].
	Volutatio,

	/// Zawartość, która nie mieści się w granicach kontenera,
	/// zostaje twardo obcięta dokładnie na linii jego krawędzi (Ucięcie /
	/// Clip). Przycinanie wykonuje odbiorca (np. `Ui::set_clip_rect`).
	Truncatio,

	/// Strumień elementów po napotkaniu krawędzi kontenera załamuje się,
	/// przenosząc nadmiarowe gniazda do nowo utworzonego rzędu lub linii
	/// (Zawijanie / Wrap). Zarezerwowane: `Vas::resolve` nie ma dla tego
	/// wariantu osobnej logiki (działa jak `Truncatio`).
	Flexio,

	/// Nadmiarowa zawartość zostaje pogrupowana w osobne, pełnowymiarowe
	/// ekrany, umożliwiając przełączanie się między nimi jak między
	/// kartami w przeglądarce (Stronicowanie / Tabs). Zarezerwowane:
	/// `Vas::resolve` nie ma dla tego wariantu osobnej logiki (działa jak
	/// `Truncatio`).
	Paginatio,
}

/// Wynik rozmieszczenia ([`Vas::resolve`]): widoczne gniazda w kolejności osi,
/// każde jako krotka `(numerus, Rect)`.
///
/// Gniazda ukryte (`latens`) nie występują w wyniku. Dostęp po numerze gniazda:
/// [`get`](Self::get) / [`rect`](Self::rect); po kolejności na osi:
/// [`iter`](Self::iter), [`as_slice`](Self::as_slice), `for` po wartości lub
/// referencji.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VasDispositio(Vec<(usize, Rect)>);

impl VasDispositio {
	/// Liczba widocznych gniazd.
	#[must_use]
	pub const fn len(&self) -> usize { self.0.len() }

	/// Czy nie ma żadnego widocznego gniazda.
	#[must_use]
	pub const fn is_empty(&self) -> bool { self.0.is_empty() }

	/// Gniazda w kolejności osi jako wycinek krotek `(numerus, Rect)`.
	#[must_use]
	pub fn as_slice(&self) -> &[(usize, Rect)] { &self.0 }

	/// Zamienia wynik na `Vec<(numerus, Rect)>`.
	#[must_use]
	pub fn into_vec(self) -> Vec<(usize, Rect)> { self.0 }

	/// Iterator po `&(numerus, Rect)` w kolejności osi.
	pub fn iter(&self) -> slice::Iter<'_, (usize, Rect)> { self.0.iter() }

	/// Same prostokąty, w kolejności osi.
	pub fn rects(&self) -> impl Iterator<Item = Rect> + '_ { self.0.iter().map(|&(_, rect)| rect) }

	/// Same numery gniazd, w kolejności osi.
	pub fn numeri(&self) -> impl Iterator<Item = usize> + '_ { self.0.iter().map(|&(numerus, _)| numerus) }

	/// Prostokąt gniazda o danym `numerus`; `None`, gdy gniazda nie ma lub jest
	/// ukryte (`latens`).
	#[must_use]
	pub fn get(&self, numerus: usize) -> Option<Rect> {
		self.0.iter().find(|(n, _)| *n == numerus).map(|&(_, rect)| rect)
	}

	/// Prostokąt gniazda o danym `numerus`.
	///
	/// # Panics
	/// Panikuje, gdy gniazda o tym numerze nie ma w wyniku (nie istnieje albo
	/// jest ukryte przez `latens`); komunikat zawiera dostępne numery. Dla
	/// gniazd opcjonalnych użyj [`get`](Self::get).
	#[must_use]
	pub fn rect(&self, numerus: usize) -> Rect {
		self.get(numerus).unwrap_or_else(|| {
			panic!(
				"VasDispositio: brak gniazda o numerze {numerus} (dostępne: {:?}); gniazdo może być ukryte (`latens`)",
				self.numeri().collect::<Vec<_>>()
			)
		})
	}

	/// Tworzy po jednym potomnym `Ui` na każde gniazdo, w kolejności osi (taka
	/// sama jak w [`iter`](Self::iter)).
	///
	/// `clip`: `None` — każde `Ui` jest przycinane do własnego prostokąta;
	/// `Some(r)` — wszystkie do `r` (np. do ramy kontenera w trybie
	/// `Volutatio`, gdzie gniazda wystają poza ramę).
	pub fn child_uis(&self, ui: &mut Ui, clip: Option<Rect>) -> Vec<Ui> {
		self.0
			.iter()
			.map(|&(_, rect)| {
				let mut child = ui.new_child(UiBuilder::new().max_rect(rect));
				child.set_clip_rect(clip.unwrap_or(rect));
				child
			})
			.collect()
	}
}

impl IntoIterator for VasDispositio {
	type IntoIter = std::vec::IntoIter<(usize, Rect)>;
	type Item = (usize, Rect);

	fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a VasDispositio {
	type IntoIter = slice::Iter<'a, (usize, Rect)>;
	type Item = &'a (usize, Rect);

	fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

/// Wspólny rdzeń geometryczny i strukturalny dla każdego kontenera.
///
/// Najkrócej: `Vas::horizontalis(redundantia).padding((..)).spatia([..])`,
/// potem [`resolve`](Self::resolve).
#[derive(Debug, Clone, PartialEq)]
pub struct Vas {
	/// Orientacja osi głównej: true = pozioma, false = pionowa.
	pub axis_maior_horizontalis: bool,

	/// Wektor ruchu na osi głównej: false = Directus, true = Reversus.
	pub axis_maior_reversus: bool,

	/// Strategia obsługi przepełnienia (overflow) na osi głównej.
	pub redundantia_maior: VasRedundantia,

	/// Pakiet wartości geometrycznych layoutu (`.0`: `p_am`, `.1`: `p_cm`,
	/// `.2`: `p_at`, `.3`: `p_ct`, `.4`: `gap`)
	pub padding_am_cm_at_ct_gap: (f32, f32, f32, f32, f32),

	/// Zablokowana sekwencja gniazd ułożonych wzdłuż osi głównej.
	pub foveae: Box<[VasFovea]>,
}

impl Vas {
	/// W `Volutatio` minimalna rozpiętość gniazda `Auto` (miejsce na ikonę i
	/// tekst bez łamania wiersza).
	pub const VOLUTATIO_AUTO_MIN: f32 = 115.0;

	// ---- konstruktory i buildery -----------------------------------------

	/// Kontener o osi głównej poziomej: bez odstępów, bez gniazd, kierunek
	/// `Directus`.
	#[must_use]
	pub fn horizontalis(redundantia: VasRedundantia) -> Self { Self::nova(true, redundantia) }

	/// Kontener o osi głównej pionowej: bez odstępów, bez gniazd, kierunek
	/// `Directus`.
	#[must_use]
	pub fn verticalis(redundantia: VasRedundantia) -> Self { Self::nova(false, redundantia) }

	fn nova(horizontalis: bool, redundantia: VasRedundantia) -> Self {
		Self {
			axis_maior_horizontalis: horizontalis,
			axis_maior_reversus: false,
			redundantia_maior: redundantia,
			padding_am_cm_at_ct_gap: (0.0, 0.0, 0.0, 0.0, 0.0),
			foveae: Box::default(),
		}
	}

	/// Kierunek `Reversus` na osi głównej.
	#[must_use]
	pub const fn reversus(mut self) -> Self {
		self.axis_maior_reversus = true;
		self
	}

	/// Ustawia cały pakiet `(p_am, p_cm, p_at, p_ct, gap)`.
	#[must_use]
	pub const fn padding(mut self, padding_am_cm_at_ct_gap: (f32, f32, f32, f32, f32)) -> Self {
		self.padding_am_cm_at_ct_gap = padding_am_cm_at_ct_gap;
		self
	}

	/// Dodaje jedno gniazdo z jawnym numerem.
	#[must_use]
	pub fn fovea(self, fovea: VasFovea) -> Self { self.foveae(std::iter::once(fovea)) }

	/// Dodaje gniazda z jawnymi numerami.
	#[must_use]
	pub fn foveae(mut self, foveae: impl IntoIterator<Item = VasFovea>) -> Self {
		let mut wszystkie = std::mem::take(&mut self.foveae).into_vec();
		wszystkie.extend(foveae);
		self.foveae = wszystkie.into_boxed_slice();
		self
	}

	/// Dodaje gniazda o podanych rozpiętościach, numerując je kolejno od
	/// najwyższego dotychczasowego numeru + 1 (od 0, gdy nie ma gniazd).
	#[must_use]
	pub fn spatia(self, spatia: impl IntoIterator<Item = VasSpatium>) -> Self {
		let start = self.foveae.iter().map(|f| f.numerus).max().map_or(0, |najwyzszy| najwyzszy + 1);
		let nowe: Vec<VasFovea> =
			spatia.into_iter().enumerate().map(|(i, spatium)| VasFovea::new(start + i, spatium)).collect();
		self.foveae(nowe)
	}

	// ---- geometria -------------------------------------------------------

	/// Widoczne gniazda (bez `latens`) w kolejności osi.
	fn widoczne(&self) -> Vec<&VasFovea> { self.foveae.iter().filter(|f| !f.latens).collect() }

	/// Długość dostępna dla gniazd po odjęciu odstępów między nimi.
	// Liczba gniazd jest mała, a f32 reprezentuje dokładnie liczby całkowite do 2^24.
	// `mul_add` zmieniłby wyniki bitowo względem dotychczasowej implementacji.
	#[allow(clippy::cast_precision_loss, clippy::suboptimal_flops)]
	fn dostepna_dlugosc(&self, dlugosc_osi: f32, liczba_gniazd: usize) -> f32 {
		let gap = self.padding_am_cm_at_ct_gap.4;
		(dlugosc_osi - gap * liczba_gniazd.saturating_sub(1) as f32).max(0.0)
	}

	/// Rozmiary gniazd (`Pixel` / `Percent` / `Auto`) przed ewentualnym
	/// ściśnięciem oraz ich suma.
	// Liczba gniazd jest mała, a f32 reprezentuje dokładnie liczby całkowite do 2^24.
	#[allow(clippy::cast_precision_loss)]
	fn rozmiary_bazowe(&self, widoczne: &[&VasFovea], dostepna: f32) -> (Vec<f32>, f32) {
		let mut rozmiary = vec![0.0; widoczne.len()];
		let mut przydzielone = 0.0;
		let mut liczba_auto = 0;

		for (rozmiar, fovea) in rozmiary.iter_mut().zip(widoczne) {
			match fovea.spatium {
				VasSpatium::Pixel(px) => {
					*rozmiar = px;
					przydzielone += px;
				}
				VasSpatium::Percent(pct) => {
					*rozmiar = dostepna * (pct / 100.0);
					przydzielone += *rozmiar;
				}
				VasSpatium::Auto => liczba_auto += 1,
			}
		}

		if liczba_auto > 0 {
			// W Volutatio gniazda zachowują oryginalny rozmiar i NIE MOGĄ
			// skurczyć się poniżej Vas::VOLUTATIO_AUTO_MIN
			let minimum =
				if self.redundantia_maior == VasRedundantia::Volutatio { Self::VOLUTATIO_AUTO_MIN } else { 0.0 };
			let reszta = (dostepna - przydzielone).max(0.0);
			let rozmiar_auto = (reszta / liczba_auto as f32).max(minimum);
			for (rozmiar, fovea) in rozmiary.iter_mut().zip(widoczne) {
				if matches!(fovea.spatium, VasSpatium::Auto) {
					*rozmiar = rozmiar_auto;
					przydzielone += rozmiar_auto;
				}
			}
		}

		(rozmiary, przydzielone)
	}

	/// Oblicza całkowitą wymaganą rozpiętość zawartości na osi głównej
	// Liczba gniazd jest mała, a f32 reprezentuje dokładnie liczby całkowite do 2^24.
	// `mul_add` zmieniłby wyniki bitowo względem dotychczasowej implementacji.
	#[allow(clippy::cast_precision_loss, clippy::suboptimal_flops)]
	#[must_use]
	pub fn total_content_length(&self, rect: Rect) -> f32 {
		let (am, cm, _at, _ct, gap) = self.padding_am_cm_at_ct_gap;
		let widoczne = self.widoczne();
		if widoczne.is_empty() {
			return 0.0;
		}
		let dlugosc_osi = if self.axis_maior_horizontalis { rect.width() } else { rect.height() };
		let dostepna = self.dostepna_dlugosc((dlugosc_osi - am - cm).max(0.0), widoczne.len());
		let (_, przydzielone) = self.rozmiary_bazowe(&widoczne, dostepna);
		przydzielone + gap * (widoczne.len() - 1) as f32 + am + cm
	}

	/// Rozmieszcza widoczne gniazda w `rect` (bez przewijania).
	#[must_use]
	pub fn resolve(&self, rect: Rect) -> VasDispositio { self.resolve_scroll(rect, 0.0) }

	/// Rozmieszcza widoczne gniazda w `rect`, przesuwając początek osi o
	/// `scroll_offset` (dla `Volutatio`).
	#[must_use]
	pub fn resolve_scroll(&self, rect: Rect, scroll_offset: f32) -> VasDispositio {
		let (am, cm, at, ct, gap) = self.padding_am_cm_at_ct_gap;
		let horizontalis = self.axis_maior_horizontalis;
		let reversus = self.axis_maior_reversus;

		// Wnętrze ramy po odjęciu paddingów: oś główna (am, cm), poprzeczna
		// (at, ct).
		let (mut min, mut max) = (rect.min, rect.max);
		if horizontalis {
			min.x += am;
			max.x -= cm;
			min.y += at;
			max.y -= ct;
		} else {
			min.y += am;
			max.y -= cm;
			min.x += at;
			max.x -= ct;
		}

		let widoczne = self.widoczne();
		if widoczne.is_empty() {
			return VasDispositio::default();
		}

		let (os_min, os_max, poprzeczna_min, dlugosc_osi, dlugosc_poprzeczna) = if horizontalis {
			(min.x, max.x, min.y, max.x - min.x, max.y - min.y)
		} else {
			(min.y, max.y, min.x, max.y - min.y, max.x - min.x)
		};

		let dostepna = self.dostepna_dlugosc(dlugosc_osi, widoczne.len());
		let (mut rozmiary, przydzielone) = self.rozmiary_bazowe(&widoczne, dostepna);

		// WYŁĄCZNIE Compressio ma prawo zmniejszać wymiary, aby upchnąć
		// zawartość
		if przydzielone > dostepna && self.redundantia_maior == VasRedundantia::Compressio {
			let skala = dostepna / przydzielone;
			for rozmiar in &mut rozmiary {
				*rozmiar *= skala;
			}
		}

		// W Volutatio punkt startowy jest przesuwany o scroll_offset
		let mut pozycja = if reversus { os_max + scroll_offset } else { os_min - scroll_offset };
		let mut gniazda = Vec::with_capacity(rozmiary.len());

		for (fovea, rozmiar) in widoczne.iter().zip(rozmiary) {
			let (poczatek, koniec) = if reversus { (pozycja - rozmiar, pozycja) } else { (pozycja, pozycja + rozmiar) };
			let rect_gniazda = if horizontalis {
				Rect::from_min_max(pos2(poczatek, poprzeczna_min), pos2(koniec, poprzeczna_min + dlugosc_poprzeczna))
			} else {
				Rect::from_min_max(pos2(poprzeczna_min, poczatek), pos2(poprzeczna_min + dlugosc_poprzeczna, koniec))
			};
			gniazda.push((fovea.numerus, rect_gniazda));
			pozycja = if reversus { pozycja - (rozmiar + gap) } else { pozycja + (rozmiar + gap) };
		}

		VasDispositio(gniazda)
	}
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // oczekiwane wartości są dokładnie reprezentowalne w f32
mod testy {
	use super::*;

	fn r(x0: f32, y0: f32, x1: f32, y1: f32) -> Rect { Rect::from_min_max(pos2(x0, y0), pos2(x1, y1)) }

	#[test]
	fn poziomy_compressio_z_paddingiem_i_odstepami() {
		let vas = Vas::horizontalis(VasRedundantia::Compressio).padding((10.0, 10.0, 5.0, 5.0, 10.0)).spatia([
			VasSpatium::Pixel(100.0),
			VasSpatium::Auto,
			VasSpatium::Percent(25.0),
		]);
		let ramka = r(0.0, 0.0, 500.0, 50.0);

		assert_eq!(vas.resolve(ramka).into_vec(), vec![
			(0, r(10.0, 5.0, 110.0, 45.0)),
			(1, r(120.0, 5.0, 365.0, 45.0)),
			(2, r(375.0, 5.0, 490.0, 45.0)),
		]);
		assert_eq!(vas.total_content_length(ramka), 500.0);
	}

	#[test]
	fn pionowy_reversus_liczy_od_dolu() {
		let wynik = Vas::verticalis(VasRedundantia::Compressio)
			.reversus()
			.spatia([VasSpatium::Pixel(40.0), VasSpatium::Pixel(20.0), VasSpatium::Auto])
			.resolve(r(0.0, 0.0, 100.0, 200.0));

		assert_eq!(wynik.into_vec(), vec![
			(0, r(0.0, 160.0, 100.0, 200.0)),
			(1, r(0.0, 140.0, 100.0, 160.0)),
			(2, r(0.0, 0.0, 100.0, 140.0)),
		]);
	}

	#[test]
	fn ukryte_gniazdo_nie_zajmuje_miejsca() {
		let wynik = Vas::horizontalis(VasRedundantia::Compressio)
			.fovea(VasFovea::auto(0))
			.fovea(VasFovea::auto(1).latens(true))
			.fovea(VasFovea::auto(2))
			.resolve(r(0.0, 0.0, 90.0, 10.0));

		assert_eq!(wynik.numeri().collect::<Vec<_>>(), vec![0, 2]);
		assert_eq!(wynik.get(1), None);
		assert_eq!(wynik.get(0), Some(r(0.0, 0.0, 45.0, 10.0)));
		assert_eq!(wynik.rect(2), r(45.0, 0.0, 90.0, 10.0));
	}

	#[test]
	fn tylko_compressio_skaluje_rozmiary() {
		let ramka = r(0.0, 0.0, 200.0, 10.0);
		let spatia = [VasSpatium::Pixel(300.0), VasSpatium::Pixel(100.0)];

		let sciskane = Vas::horizontalis(VasRedundantia::Compressio).spatia(spatia).resolve(ramka);
		assert_eq!(sciskane.rects().collect::<Vec<_>>(), vec![r(0.0, 0.0, 150.0, 10.0), r(150.0, 0.0, 200.0, 10.0)]);

		let ucinane = Vas::horizontalis(VasRedundantia::Truncatio).spatia(spatia).resolve(ramka);
		assert_eq!(ucinane.rects().collect::<Vec<_>>(), vec![r(0.0, 0.0, 300.0, 10.0), r(300.0, 0.0, 400.0, 10.0)]);
	}

	#[test]
	fn volutatio_ma_minimum_dla_auto_i_przesuniecie() {
		let vas = Vas::horizontalis(VasRedundantia::Volutatio).spatia([VasSpatium::Auto; 2]);
		let ramka = r(0.0, 0.0, 100.0, 10.0);

		assert_eq!(vas.resolve_scroll(ramka, 20.0).into_vec(), vec![
			(0, r(-20.0, 0.0, 95.0, 10.0)),
			(1, r(95.0, 0.0, 210.0, 10.0))
		]);
		assert_eq!(vas.total_content_length(ramka), 230.0);
	}

	#[test]
	fn spatia_kontynuuje_numeracje() {
		let numery = |vas: &Vas| vas.foveae.iter().map(|f| f.numerus).collect::<Vec<_>>();

		let od_zera = Vas::verticalis(VasRedundantia::Truncatio).spatia([VasSpatium::Auto; 2]);
		assert_eq!(numery(&od_zera), vec![0, 1]);

		let po_jawnym =
			Vas::verticalis(VasRedundantia::Truncatio).fovea(VasFovea::auto(5)).spatia([VasSpatium::Auto; 2]);
		assert_eq!(numery(&po_jawnym), vec![5, 6, 7]);
	}

	#[test]
	fn pusty_vas_daje_pusty_wynik() {
		let vas = Vas::horizontalis(VasRedundantia::Compressio);
		let ramka = r(0.0, 0.0, 100.0, 100.0);

		assert!(vas.resolve(ramka).is_empty());
		assert_eq!(vas.total_content_length(ramka), 0.0);
	}

	#[test]
	#[should_panic(expected = "brak gniazda o numerze 9")]
	fn rect_panikuje_dla_brakujacego_gniazda() {
		let wynik =
			Vas::horizontalis(VasRedundantia::Compressio).spatia([VasSpatium::Auto]).resolve(r(0.0, 0.0, 10.0, 10.0));
		let _ = wynik.rect(9);
	}

	#[test]
	fn dispositio_iteruje_po_wartosci_i_referencji() {
		let wynik = Vas::horizontalis(VasRedundantia::Compressio)
			.spatia([VasSpatium::Auto; 2])
			.resolve(r(0.0, 0.0, 10.0, 10.0));

		let po_referencji: Vec<usize> = (&wynik).into_iter().map(|&(n, _)| n).collect();
		assert_eq!(wynik.len(), 2);
		let po_wartosci: Vec<usize> = wynik.into_iter().map(|(n, _)| n).collect();
		assert_eq!(po_referencji, po_wartosci);
	}
}
