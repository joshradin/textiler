use log::info;
use wasm_bindgen_test::wasm_bindgen_test;
use yew::{function_component, html, use_callback, Callback, Html, Renderer};

use textiler_core::prelude::*;
use textiler::surfaces::Card;
use textiler::typography::Typography;
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
    let (mode, set_mode): (ThemeMode, Callback<ThemeMode>) = use_mode();

    let onclick = {
        use_callback(
            mode.clone(),
            move |_: yew::events::MouseEvent, mode| match mode {
                ThemeMode::Light => set_mode.emit(ThemeMode::Dark),
                ThemeMode::Dark | ThemeMode::System => set_mode.emit(ThemeMode::Light),
            },
        )
    };

    html! {
        <Card variant="outlined">
            <Typography level="h1">{"hello, card!"}</Typography>
            <Typography level="title-lg">{"Welcome to a card"}</Typography>
            <Typography level="body-lg">{"This is a card"}</Typography>
        </Card>
    }
}

#[wasm_bindgen_test]
async fn cards() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    info!("starting test");
    let handle = Renderer::<App>::new().render();
}
