use log::info;
use wasm_bindgen_test::wasm_bindgen_test;
use yew::{function_component, html, use_callback, Callback, Html, Renderer};

use textiler_core::prelude::*;
use textiler::Link;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn App() -> Html {
    let theme = yew::functional::use_mut_ref(|| Theme::default());

    html! {
       <ThemeProvider theme={theme.borrow().clone()}>
            <CssBaseline />
            <Main />
        </ThemeProvider>
    }
}

#[function_component]
fn Main() -> Html {
    let theme = Theme::default();

    html! {
        <Link>{"Click me!"}</Link>
    }
}

#[wasm_bindgen_test]
async fn links() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    info!("starting test");
    let handle = Renderer::<App>::new().render();
}
