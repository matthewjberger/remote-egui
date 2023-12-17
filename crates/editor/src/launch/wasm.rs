pub fn launch() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "editor_canvas", // hardcode it
                web_options,
                Box::new(|cc| Box::new(crate::app::App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
