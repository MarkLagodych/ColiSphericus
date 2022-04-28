#![allow(unused)]
#![allow(non_snake_case)]

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
    time: i32, /// SECONDS!!!
    is_still_growing: bool,

    // Parameters
    is_bounded: bool,
    should_wait_until_end: bool,
    is_hungry: bool,
    should_gen_St: bool,
    should_gen_Nt: bool,
    should_gen_tt: bool,
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
            time: 15 * 60,
            is_still_growing: false,

            is_bounded: false,
            should_wait_until_end: false,
            is_hungry: false,
            should_gen_St: false,
            should_gen_Nt: false,
            should_gen_tt: false,
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
        self.is_still_growing = false;
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn set_time(&mut self, time: f64) {
        self.time = time.ceil() as i32;
        self.is_still_growing = true;
    }

    pub fn set_bounded(&mut self, bounded: bool) {
        self.is_bounded = bounded;
    }

    pub fn set_should_wait_until_end(&mut self, value: bool) {
        self.should_wait_until_end = value;
    }

    fn is_time_passed(&self) -> bool {
        self.time <= 0
    }

    fn tick(&mut self) {
        self.time -= 1;
    }

    pub fn is_finished(&self) -> bool {
        self.is_time_passed() && !self.is_still_growing
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
        self.tick();

        if self.is_time_passed() && !(self.should_wait_until_end && self.is_still_growing) {
            return;
        }

        if !self.is_time_passed() {
            // Create new circle
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
        }

        for circle in &mut self.circles {
            circle.grow(self.speed);
        }

        // 1. Draw circles;  2. Find out if there are still growing circles
        self.is_still_growing = false;
        for circle in &self.circles {
            if circle.active {
                self.is_still_growing = true;
                self.draw_circle(circle);
            }
        }

        for i in 0..(self.circles.len() - 1) {

            if self.is_bounded && self.circles[i].out_of_bounds() {
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
