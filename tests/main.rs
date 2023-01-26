use enigo::KeyboardControllable;

// #[cfg(feature = "test_browser")]
mod browser;

// #[cfg(feature = "test_browser")]
#[test]
fn browser_events() {
    let recv = browser::launch();
    browser::mouse::run(&recv);
    browser::key::run(&recv);
}

#[test]
fn press_page_down() {
    std::thread::sleep(std::time::Duration::from_millis(500));
    enigo::Enigo::new().key_click(enigo::Key::PageUp);
}
