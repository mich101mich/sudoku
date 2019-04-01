#![feature(
    proc_macro_hygiene,
    slice_patterns,
    custom_attribute,
    bind_by_move_pattern_guards
)]

use rand::{thread_rng, Rng};
use smithy::{smd, types::Component};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement, InputEvent};

#[wasm_bindgen]
pub fn start(root_element: Element) {
    let app = render();

    smithy::mount(Box::new(app), root_element);
}

fn render() -> impl Component {
    let mut difficulty = 4;
    let mut started = false;

    smd!(
        <div style="margin: 0.5cm">
            Difficulty:
            {if difficulty > 3 {
                smd!(
                    <input
                        type="range"
                        min="0" max="5"
                        name="difficulty"
                        value={ difficulty.to_string() }
                        on_input={ |e: &InputEvent| {
                            let target: HtmlInputElement = e.target().unwrap().unchecked_into();
                            difficulty = usize::from_str(&target.value()).unwrap_or(4);
                        }}
                    />
                    <br/>
                    <br/>
                    <button on_click={ |_| { started = true; }}>Generate</button>
                )
            } else {
                smd!({difficulty.to_string()})
            }}
        </div>
    )
}
