#![allow(unused)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rand::Rng;

use std::f64;

// #[wasm_bindgen]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
    fill_style: JsValue,
}

impl Circle {
    pub fn from(x: f64, y: f64, r: f64, fill_style: &str) -> Self {
        Self {
            x, y, r, fill_style: JsValue::from_str(fill_style)
        }
    }

    pub fn new() -> Self {
        Self::from(0., 0., 0., "#000000")
    }
    
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();

        let color = format!(
            "#{:02x}{:02x}{:02x}",
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>()
        );
        
        Self::from(
            rng.gen_range(0. .. 1000.),
            rng.gen_range(0. .. 1000.),
            0.,
            color.as_str()
        )
    }

    pub fn grow(&mut self, speed: f64) {
        self. r += speed;
    }
}



#[wasm_bindgen]
pub struct CircleDrawer {
    ctx: web_sys::CanvasRenderingContext2d,
    circles: Vec::<Circle>,
    speed: f64,
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
            ctx: context,
            circles: vec![],
            speed: 0.2
        }
    }

    fn draw_circle(&self, circle: &Circle) {
        self.ctx.begin_path();
        self.ctx.ellipse(
            circle.x, circle.y, circle.r, circle.r,
            0., // rotation
            0., f64::consts::PI * 2.
        );
        self.ctx.set_fill_style(&circle.fill_style);
        self.ctx.fill();
    }

    pub fn clear(&mut self) {
        self.ctx.clear_rect(0., 0., 1000., 1000.);
        self.circles.clear();
    }


    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }


    pub fn draw(&mut self) {
        self.circles.push(Circle::new_random());

        for circle in &mut self.circles {
            circle.grow(self.speed);
        }

        for circle in &self.circles {
            self.draw_circle(circle);
        }
    }
    
}