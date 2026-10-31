// 🦀 src\include\egui_run_app.rs

/// Uniwersalny punkt wejścia uruchamiający dowolną aplikację eframe (Native i
/// WASM). Przyjmuje nazwę okna (dla wersji natywnej) oraz ID elementu canvas
/// (dla wersji webowej).
pub fn run_app<A>() -> Result<(), eframe::Error>
where A: eframe::App + Default + 'static {
	#[cfg(not(target_arch = "wasm32"))]
	{
		let options = eframe::NativeOptions {
			viewport: eframe::egui::ViewportBuilder::default()
				.with_inner_size([1200.0, 800.0])
				.with_title("Ecliptica - Symulator Orbity Ziemi"),
			..Default::default()
		};

		eframe::run_native("Ecliptica", options, Box::new(|_cc| Ok(Box::new(A::default()))))
	}

	#[cfg(target_arch = "wasm32")]
	{
		use wasm_bindgen::JsCast;

		// Inicjalizacja logowania w przeglądarce (F12)
		eframe::WebLogger::init(log::LevelFilter::Debug).ok();

		let web_options = eframe::WebOptions { renderer: eframe::Renderer::Glow, ..Default::default() };

		wasm_bindgen_futures::spawn_local(async move {
			let document = web_sys::window().expect("Brak obiektu window").document().expect("Brak obiektu document");

			let canvas = document
				.get_element_by_id("the_canvas_id")
				.expect("Nie znaleziono elementu canvas o podanym ID")
				.dyn_into::<web_sys::HtmlCanvasElement>()
				.expect("Element nie jest poprawnym HtmlCanvasElement");

			let runner = eframe::WebRunner::new();
			runner
				.start(canvas, web_options, Box::new(|_cc| Ok(Box::new(A::default()))))
				.await
				.expect("Błąd: Nie udało się wystartować eframe w WASM");
		});

		Ok(())
	}
}
