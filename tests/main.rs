// #[cfg(feature = "test_browser")]
mod browser;

// #[cfg(feature = "test_browser")]
#[test]
fn browser_events() {
    let recv = browser::launch();
    browser::mouse::run(&recv);
    browser::key::run(&recv);
}
