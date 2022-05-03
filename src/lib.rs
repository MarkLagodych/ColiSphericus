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
    speed: f64, /// mm/s
    time: f64, /// s
    iter_duration: f64, /// s - Duration of modulation performed each iteration
    is_still_growing: bool,

    // Parameters
    is_bounded: bool,
    should_wait_until_end: bool,
    is_hungry: bool,
    should_gen_S: bool,
    should_gen_N: bool,
    should_gen_T: bool,

    // Generated data
    data_S: Vec<f64>,
    data_N: Vec<i32>,
    data_T: Vec<f64>,
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
            time: 15. * 60.,
            iter_duration: 1000.,
            is_still_growing: false,

            is_bounded: false,
            should_wait_until_end: false,
            is_hungry: false,
            should_gen_S: false,
            should_gen_N: false,
            should_gen_T: false,

            data_S: vec![],
            data_N: vec![],
            data_T: vec![],
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
        self.data_S.clear();
        self.data_N.clear();
        self.data_T.clear();
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn set_time(&mut self, time: f64) {
        self.time = time;
        self.is_still_growing = true;
    }

    pub fn set_iter_duration(&mut self, value: f64) {
        self.iter_duration = value;
    }

    pub fn set_bounded(&mut self, bounded: bool) {
        self.is_bounded = bounded;
    }

    pub fn set_should_wait_until_end(&mut self, value: bool) {
        self.should_wait_until_end = value;
    }

    fn is_time_passed(&self) -> bool {
        self.time <= 0.
    }

    fn tick(&mut self) {
        self.time -= self.iter_duration;
    }

    pub fn is_finished(&self) -> bool {
        self.is_time_passed() && !self.is_still_growing
    }

    /// Indicates if the current time is approximately equal to xxx.0
    /// This is important because the time may not change by exactly 1 second
    fn is_second_finished(&self) -> bool {
        self.time - self.time.floor() <= self.iter_duration
    }

    pub fn set_gen_S(&mut self, value: bool) {
        self.should_gen_S = value;
    }

    pub fn set_gen_N(&mut self, value: bool) {
        self.should_gen_N = value;
    }

    pub fn set_gen_T(&mut self, value: bool) {
        self.should_gen_T = value;
    }


    pub fn get_data_S(&self) -> js_sys::Float64Array {
        js_sys::Float64Array::from(&self.data_S[..])
    }

    pub fn get_data_N(&self) -> js_sys::Int32Array {
        js_sys::Int32Array::from(&self.data_N[..])
    }

    pub fn get_data_t(&self) -> js_sys::Float64Array {
        let mut t = Vec::<f64>::new();
        for circle in &self.circles {
            t.push(circle.life_length);
        }
        js_sys::Float64Array::from(&t[..])
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
            // (nearly) every second
            if self.is_second_finished() {
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
        }

        for circle in &mut self.circles {
            circle.grow(self.speed * self.iter_duration);
            circle.grow_old(self.iter_duration);
        }

        // 1. Draw circles;  2. Find out if there are still growing circles
        self.is_still_growing = false;
        let mut n_active = 0i32;
        for circle in &self.circles {
            if circle.active {
                n_active += 1;
                self.is_still_growing = true;
                self.draw_circle(circle);
            }
        }

        // Generate data
        if self.is_second_finished() {
            if self.should_gen_S {
                let mut S = 0.0f64;
                for circle in &self.circles {
                    S += circle.area();
                }
                self.data_S.push(S);
            }

            if self.should_gen_N {
                self.data_N.push(n_active);
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
