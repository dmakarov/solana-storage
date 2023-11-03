// todo remove deprecated
#![allow(non_snake_case, deprecated)]

extern crate ui;

use dioxus::events::{KeyCode, KeyboardEvent};
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_tui::TuiContext;

fn main() {
    // launch the app in the terminal
    dioxus_tui::launch_cfg(
        App,
        dioxus_tui::Config::new()
            .without_ctrl_c_quit()
            // Some older terminals only support 16 colors or ANSI colors
            // If your terminal is one of these, change this to BaseColors or ANSI
            .with_rendering_mode(dioxus_tui::RenderingMode::Rgb),
    );
}

/// create a component that renders the top-level UI layout
fn App(cx: Scope) -> Element {
    let tui_ctx: TuiContext = cx.consume_context().unwrap();

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            width: "100%",
            // height: "10px",
            // background_color: "red",
            // justify_content: "center",
            // align_items: "center",
            onkeydown: move |k: KeyboardEvent| if let KeyCode::Q = k.key_code {
                tui_ctx.quit();
            },

            div {
                display: "flex",
                flex_direction: "row",
                height: "80%",
                div {
                    width: "50%",
                    ui::Client {}
                }
                div {
                    width: "50%",
                    ui::Validator {}
                }
            }
            div {
                height: "20%",
                ui::Commands {}
            }
        }
    })
}
