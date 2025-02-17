use leptos::html::*;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn Test(max: u32) -> impl IntoView {
    macro_rules! heading {
        ($q:expr) => {{
            $q().style("margin: 0.5em;")
                .child(format!("Level {max}"))
                .into_any()
        }};
    }

    let tag = match max {
        i if i <= 1 => heading!(h5),
        i if i <= 2 => heading!(h4),
        i if i <= 4 => heading!(h3),
        i if i <= 8 => heading!(h2),
        _ => heading!(h1),
    };

    let initial = div()
        .style("border: 1px solid black; margin: 0.5em;")
        .child(tag);

    let mut inner = vec![];

    for i in 0..max {
        let mut spans = vec![];
        for j in 0..(max / 2) {
            spans.push(span().child((i * j).to_string() + ":"));
        }
        let s = div().child(spans).into_any();

        let half = max / 2;

        if half > 0 {
            let q = Test(TestProps::builder().max(half).build()).into_any();

            inner.push((q, s).into_any())
        } else {
            inner.push(s.into_any());
        }
    }

    initial.child(inner)
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let (num, set_num) = signal(None);

    spawn_local(async move {
        let n = invoke("get_num", JsValue::NULL).await.as_f64().unwrap() as u32;

        set_num.set(Some(n));
    });

    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>

            {move || {
                if let Some(n) = num.get() {
                    view! {
                        <Test max=n />
                    }.into_any()
                }
                else {
                    ().into_any()
                }
            }}
        </main>
    }
}
