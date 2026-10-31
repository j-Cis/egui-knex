// 🦀 src\include\egui_run_app.rs

/// Konfiguracja startu aplikacji (wspólna dla natywnej i WASM).
///
/// Biblioteka nie narzuca żadnej nazwy, rozmiaru ani identyfikatora — wszystko
/// pochodzi stąd. Pola oznaczone „natywnie” / „WASM” są ignorowane na drugiej
/// platformie, więc ta sama konfiguracja działa w obu wariantach.
#[derive(Debug, Clone, PartialEq)]
pub struct AppConfig {
	/// Identyfikator aplikacji. Natywnie trafia do `eframe::run_native` jako
	/// `app_name` — według dokumentacji eframe służy m.in. jako miejsce zapisu
	/// stanu (persystencja), więc różne aplikacje powinny mieć różne nazwy.
	pub name: String,

	/// Tytuł okna (natywnie). Domyślnie równy [`AppConfig::name`].
	pub title: String,

	/// Początkowy rozmiar okna w pikselach logicznych: `(szerokość, wysokość)`
	/// (natywnie).
	pub inner_size: (f32, f32),

	/// Id elementu `<canvas>` w dokumencie HTML (WASM).
	pub canvas_id: String,

	/// Renderer. `None` — wybiera eframe na podstawie włączonych cech
	/// (features); `Some(..)` wymusza wskazany (wymaga włączenia
	/// odpowiedniej cechy `glow` / `wgpu` w `eframe`).
	pub renderer: Option<eframe::Renderer>,
}

impl Default for AppConfig {
	/// Neutralna konfiguracja startowa. Dla aplikacji z włączoną persystencją
	/// ustaw własną nazwę przez [`AppConfig::new`].
	fn default() -> Self { Self::new("egui-knex") }
}

impl AppConfig {
	/// Konfiguracja z nazwą aplikacji; tytuł okna = nazwa, rozmiar 1200×800,
	/// canvas `the_canvas_id`, renderer wybrany przez eframe.
	#[must_use]
	pub fn new(name: impl Into<String>) -> Self {
		let name = name.into();
		Self {
			title: name.clone(),
			name,
			inner_size: (1200.0, 800.0),
			canvas_id: "the_canvas_id".to_owned(),
			renderer: None,
		}
	}

	/// Ustawia tytuł okna (natywnie).
	#[must_use]
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = title.into();
		self
	}

	/// Ustawia początkowy rozmiar okna `(szerokość, wysokość)` (natywnie).
	#[must_use]
	pub const fn inner_size(mut self, inner_size: (f32, f32)) -> Self {
		self.inner_size = inner_size;
		self
	}

	/// Ustawia id elementu `<canvas>` (WASM).
	#[must_use]
	pub fn canvas_id(mut self, canvas_id: impl Into<String>) -> Self {
		self.canvas_id = canvas_id.into();
		self
	}

	/// Wymusza renderer (zob. [`AppConfig::renderer`]).
	#[must_use]
	pub const fn renderer(mut self, renderer: eframe::Renderer) -> Self {
		self.renderer = Some(renderer);
		self
	}
}

/// Uruchamia aplikację `A` z konfiguracją domyślną ([`AppConfig::default`]).
///
/// Natywnie blokuje do zamknięcia okna. Na WASM zwraca od razu `Ok(())` —
/// start jest asynchroniczny, a jego błędy trafiają do konsoli przeglądarki.
///
/// # Errors
/// Natywnie: błędy z `eframe::run_native`.
pub fn run_app<A>() -> Result<(), eframe::Error>
where A: eframe::App + Default + 'static {
	run_app_with::<A>(AppConfig::default())
}

/// Jak [`run_app`], ale z własną konfiguracją.
///
/// # Errors
/// Natywnie: błędy z `eframe::run_native`.
pub fn run_app_with<A>(config: AppConfig) -> Result<(), eframe::Error>
where A: eframe::App + Default + 'static {
	run_app_with_creator(config, |_cc| A::default())
}

/// Jak [`run_app_with`], ale aplikację tworzy podany kreator, który dostaje
/// `eframe::CreationContext` (czcionki, styl, odczyt zapisanego stanu itp.) —
/// aplikacja nie musi implementować `Default`.
///
/// # Errors
/// Natywnie: błędy z `eframe::run_native`.
#[cfg_attr(not(target_arch = "wasm32"), allow(clippy::needless_pass_by_value))] // WASM przenosi `config` do zadania async
pub fn run_app_with_creator<A, F>(config: AppConfig, creator: F) -> Result<(), eframe::Error>
where
	A: eframe::App + 'static,
	F: FnOnce(&eframe::CreationContext<'_>) -> A + 'static, {
	#[cfg(not(target_arch = "wasm32"))]
	{
		eframe::run_native(&config.name, native_options(&config), Box::new(move |cc| Ok(Box::new(creator(cc)))))
	}

	#[cfg(target_arch = "wasm32")]
	{
		start_web(config, creator);
		Ok(())
	}
}

#[cfg(not(target_arch = "wasm32"))]
fn native_options(config: &AppConfig) -> eframe::NativeOptions {
	let mut options = eframe::NativeOptions {
		viewport: eframe::egui::ViewportBuilder::default()
			.with_inner_size(<[f32; 2]>::from(config.inner_size))
			.with_title(config.title.clone()),
		..Default::default()
	};
	if let Some(renderer) = config.renderer {
		options.renderer = renderer;
	}
	options
}

#[cfg(target_arch = "wasm32")]
fn start_web<A, F>(config: AppConfig, creator: F)
where
	A: eframe::App + 'static,
	F: FnOnce(&eframe::CreationContext<'_>) -> A + 'static, {
	// Logowanie do konsoli przeglądarki (F12). Ponowna inicjalizacja loggera
	// nie jest błędem krytycznym, stąd `ok()`.
	eframe::WebLogger::init(log::LevelFilter::Debug).ok();

	wasm_bindgen_futures::spawn_local(async move {
		let canvas = match find_canvas(&config.canvas_id) {
			Ok(canvas) => canvas,
			Err(err) => {
				log::error!("{err}");
				return;
			}
		};

		let mut web_options = eframe::WebOptions::default();
		if let Some(renderer) = config.renderer {
			web_options.renderer = renderer;
		}

		let result =
			eframe::WebRunner::new().start(canvas, web_options, Box::new(move |cc| Ok(Box::new(creator(cc))))).await;
		if let Err(err) = result {
			log::error!("Nie udało się wystartować eframe w WASM: {err:?}");
		}
	});
}

#[cfg(target_arch = "wasm32")]
fn find_canvas(canvas_id: &str) -> Result<web_sys::HtmlCanvasElement, String> {
	use wasm_bindgen::JsCast;

	let document = web_sys::window()
		.and_then(|window| window.document())
		.ok_or_else(|| "Brak obiektu window lub document".to_owned())?;

	document
		.get_element_by_id(canvas_id)
		.ok_or_else(|| format!("Nie znaleziono elementu o id `{canvas_id}`"))?
		.dyn_into::<web_sys::HtmlCanvasElement>()
		.map_err(|_| format!("Element o id `{canvas_id}` nie jest HtmlCanvasElement"))
}
