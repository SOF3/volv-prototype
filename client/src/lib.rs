#![allow(dead_code, unused_variables)]

use serde_json::json;
use volv::tree::System;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

mod handler;

#[wasm_bindgen]
extern {
    fn alert(message: &str);
}

#[wasm_bindgen]
pub fn main() {
    let canvas = web_sys::window().unwrap()
        .document().unwrap()
        .get_element_by_id("main-canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let (width, height) = (canvas.width(), canvas.height());

    let canvas = match canvas.get_context("2d").ok().flatten() {
        Some(canvas) => canvas,
        None => {
            alert("2D canvas is not available");
            return;
        },
    };

    let canvas = canvas.dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    canvas.set_fill_style(&JsValue::from_str("black"));
    canvas.fill_rect(0.0, 0.0, width as f64, height as f64);

    let schema = json!({
        "surface_radius": 1000.0,
        "grav_radius": std::f64::INFINITY,
        "mass": 1000.0,
        "children": [
            {
                "surface_radius": 100.0,
                "grav_radius": 500.0,
                "mass": 100.0,
            }
        ],
    });
    let system = System::from_schema(serde_json::from_value(schema).unwrap(), handler::Handler);
}
