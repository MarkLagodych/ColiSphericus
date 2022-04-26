#![allow(unused)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rand::Rng;

use std::f64;

#[wasm_bindgen]
pub struct CircleDrawer {
    ctx: web_sys::CanvasRenderingContext2d
}

#[wasm_bindgen]
impl CircleDrawer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self {
            ctx: context
        }
    }

    pub fn draw_circle(&self, x: i32, y: i32, radius: f64) {
        self.ctx.begin_path();
        self.ctx.ellipse(
            x as f64, y as f64, radius, radius,
            0., // rotation
            0., f64::consts::PI * 2.
        );
        self.ctx.fill();
    }

    pub fn draw(&self) {
        let mut rng = rand::thread_rng();

        let color = format!(
            "#{:02x}{:02x}{:02x}",
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>()
        );
        self.ctx.set_fill_style(&JsValue::from_str(color.as_str()));
        
        self.draw_circle(
            rng.gen_range(0 .. 1000),
            rng.gen_range(0 .. 1000),
            rng.gen_range(0. .. 500.),
        );
    }
}