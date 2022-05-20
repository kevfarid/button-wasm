use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement};
use yew::prelude::*;

struct Counter {
    value: i32,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Counter { value: 0 });

    let edit_state = {
        let state = state.clone();

        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();

            let button = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());

            if let Some(button) = button {
                let operation_type = button.get_attribute("id").unwrap_or("".to_string());

                state.set(Counter {
                    value: if operation_type == "sum" {
                        state.value + 1
                    } else {
                        state.value - 1
                    },
                })
            }
        })
    };

    let onclick_sum = edit_state.clone();
    let onclick_sub = edit_state.clone();

    html! {
        <main>
            <div class="button-group">
                <button onclick={onclick_sum} id="sum">{"+1"}</button>
                <button onclick={onclick_sub} id="sub">{"-1"}</button>
            </div>
            <div class="counter_wrapper">
                <p> {state.value} </p>
            </div>
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
