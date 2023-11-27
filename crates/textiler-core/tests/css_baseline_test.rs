use log::info;
use wasm_bindgen_test::wasm_bindgen_test;
use yew::{function_component, html, use_callback, Callback, Html, Renderer};

use textiler_core::prelude::*;

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
        <Sheet sx={sx!{
            "width": "100dvw",
            "height": "100dvh",
            "display": "inline-block",
            "bgcolor": "background.level1"
        }}>
            <Sheet variant={"outlined"} sx={sx!{
                "marginX": "15%",
                "marginTop": "5dvh",
                "bgcolor": "background.level2"
            }}>
                <Typography level="title-lg">{"Hello, world"}</Typography>
                <Typography level="body-md">
                {r"Welcome to the happy style system, a better way of writing text in yew.
                You can do many things with it, like "}<Typography variant="outlined">{"outlining text"}</Typography>{". \
                Or maybe giving it a bit of "}<Typography color="success">{"color"}</Typography>{"."}
                </Typography>
                <Typography level="body-xs" component="code" sx={sx!{
            display: "flex",
            alignItems: "center",
            flexDirection: "column"
        }}>
                {r###"<Typography level="title-lg">{"Hello, world"}</Typography>
<Typography level="body-sm">
    {r"Welcome to the happy style system, a better way of writing text in yew.
    You can do many things with it, like "}<Typography variant="outlined">{"outlining text"}</Typography>{"."}
</Typography>"###}
                </Typography>
                <Typography />
                <button {onclick}>{format!("{:?}", mode)}</button>
            </Sheet>
        </Sheet>
    }
}

#[wasm_bindgen_test]
async fn create_css() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    info!("starting test");
    let handle = Renderer::<App>::new().render();
}
