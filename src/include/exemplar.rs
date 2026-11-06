// 🦀 src/include/exemplar.rs

use crate::eframe;

/// Formalny kontrakt architektoniczny dla plików demonstracyjnych (exemplar).
///
/// Wymusza powiązanie 4 filarów aplikacji w strukturze `main_scope`:
/// 1. `Words` — stałe tekstowe, komunikaty, etykiety (`app_words`)
/// 2. `Theme` — stałe kolorystyczne, wymiary, motyw (`app_theme`)
/// 3. `State` — stan aplikacji implementujący `eframe::App` (`app_state`)
/// 4. `Style` — logika ramy wizualnej / layoutu (`app_style`)
pub trait ExemplarScope {
	type Words;
	type Theme;
	type State: eframe::App + Default + 'static;
	type Style;
}

/// Makro weryfikujące na etapie kompilacji, czy dany moduł `main_scope`
/// zawiera wymagane podmoduły: `app_words`, `app_theme`, `app_state`,
/// `app_style`.
#[macro_export]
macro_rules! assert_exemplar_scope {
	($scope:ident) => {
		const _: () = {
			use $scope::{app_state as _, app_style as _, app_theme as _, app_words as _};
		};
	};
}
