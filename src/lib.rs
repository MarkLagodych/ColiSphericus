#![allow(unused)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rand::Rng;

use std::f64;

mod circle;
use circle::*;

#[wasm_bindgen]
pub struct CircleDrawer {
    ctx: web_sys::CanvasRenderingContext2d,
    circles: Vec<Circle>,
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
            speed: 0.2,
        }
    }

    fn draw_circle(&self, circle: &Circle) {
        self.ctx.begin_path();
        self.ctx.ellipse(
            circle.x,
            circle.y,
            circle.r,
            circle.r,
            0., // rotation
            0.,
            f64::consts::PI * 2.,
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

    fn can_put_circle(&self, circle: &Circle) -> bool {
        for my_circle in &self.circles {
            if my_circle.intersects(circle) {
                return false;
            }
        }

        true
    }

    pub fn draw(&mut self) {
        let mut circle = Circle::new_random_color();

        let mut rng = rand::thread_rng();
        loop {
            circle.x = rng.gen_range(0. ..1000.);
            circle.y = rng.gen_range(0. ..1000.);
            if self.can_put_circle(&circle) {
                break;
            }
        }

        self.circles.push(circle);

        for circle in &mut self.circles {
            circle.grow(self.speed);
        }

        for circle in &self.circles {
            if circle.active {
                self.draw_circle(circle);
            }
        }

        for i in 0..(self.circles.len() - 1) {

            if self.circles[i].out_of_bounds() {
                self.circles[i].deactivate();
            }

            for j in (i + 1)..self.circles.len() {
                let c1 = &self.circles[i];
                let c2 = &self.circles[j];

                if c1.intersects(c2) {
                    self.circles[i].deactivate();
                    self.circles[j].deactivate();
                }
            }
        }
    }
}
