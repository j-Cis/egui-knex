// 🦀 src/element/ora/modus.rs

/// Tryby zachowania i nakładania powierzchni okna `Ora`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OraModus {
	/// Swobodne okno pływające nad kanwą roboczą.
	#[default]
	Fluctuans,

	/// Pływające okno z półprzezroczystą tarczą blokującą klikanie w tło.
	FluctuansObvelatum,

	/// Wycentrowane okno modalne na pełnym ekranie z przyciemnionym tłem.
	CentraleObvelatum,
}

/// Stan przycisku belki tytułowej (`✕`, `—`, `⛶`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OraButonStatus {
	/// Przycisk ukryty.
	Occultus,

	/// Przycisk widoczny i aktywny.
	#[default]
	Visibilis,

	/// Przycisk widoczny, ale zablokowany: wyszarzony, bez reakcji na klik.
	Clausus,
}
