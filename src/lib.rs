#![allow(unused)]
#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rand::Rng;

use std::f64;

pub mod consts;
pub mod circle;
use consts::*;
use circle::*;

#[wasm_bindgen]
pub struct CircleDrawer {
    ctx: web_sys::CanvasRenderingContext2d,
    circles: Vec<Circle>,
    speed: f64, /// mm/s
    time: f64, /// s - Overall time (Set every time you click "Begin drawing")
    clock: f64, /// s - Time counter
    iter_per_sec: i32, /// s - Duration of modulation performed each iteration
    next_circle_id: i32, /// ID to assign to the next circle
    is_still_growing: bool,

    // Parameters
    is_bounded: bool,
    should_wait_until_end: bool,
    is_hungry: bool,
    neighbour_limit: i32,
    should_gen_S: bool,
    should_gen_N: bool,
    should_gen_T: bool,
    dimensions: i32, /// 1/2/3 for 1D/2D/3D
    use_z_alpha: bool,

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
            speed: INITIAL_SPEED,
            time: INITIAL_TIME,
            clock: 0.,
            iter_per_sec: 1,
            next_circle_id: 0,
            is_still_growing: false,

            is_bounded: false,
            should_wait_until_end: false,
            is_hungry: false,
            neighbour_limit: INITIAL_NEIGHBOUR_LIMIT,
            should_gen_S: false,
            should_gen_N: false,
            should_gen_T: false,

            dimensions: 2,
            use_z_alpha: false,

            data_S: vec![],
            data_N: vec![],
            data_T: vec![],
        }
    }

    fn draw_circle(&self, c: &Circle) {
        self.ctx.begin_path();
        self.ctx.ellipse(
            c.x, c.y,
            c.r, c.r,
            0., // rotation
            0.,
            f64::consts::PI * 2.,
        );

        if self.has_z() && self.use_z_alpha {
            // Alpha will be in range [MIN_ALPHA; 1]; the closer, the more transparent
            let k = 1.0f64 - MIN_ALPHA;
            self.ctx.set_global_alpha(MIN_ALPHA + k * c.z / CANVAS_SIZE);
        }

        if self.has_z() {
            // CAUTION: magic numbers
            let x = c.x - c.r * 0.5;
            let y = c.y - c.r * 0.5;
            let gradient = self.ctx.create_radial_gradient(
                x, y, c.r * 0.2, 
                x, y, c.r * 1.9
            ).unwrap();
            gradient.add_color_stop(0., c.fill_style_str.as_str());
            gradient.add_color_stop(1., "#000000");
            self.ctx.set_fill_style(&gradient);
        } else {
            self.ctx.set_fill_style(&c.fill_style);
        }

        self.ctx.fill();
    }

    fn begin_drawing(&self) {
        self.clear_canvas();
    }

    fn clear_canvas(&self) {
        self.ctx.clear_rect(0., 0., CANVAS_SIZE, CANVAS_SIZE);
    }

    fn end_drawing(&self) {
        self.ctx.set_global_alpha(1.);
    }

    pub fn clear(&mut self) {
        self.clear_canvas();
        self.circles.clear();
        self.is_still_growing = false;
        self.data_S.clear();
        self.data_N.clear();
        self.data_T.clear();
        self.clock = 0.;
        self.next_circle_id = 0;
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn set_time(&mut self, time: f64) {
        self.time = time;
        self.is_still_growing = true;
    }

    pub fn set_iter_per_sec(&mut self, value: i32) {
        self.iter_per_sec = value;
    }

    pub fn set_bounded(&mut self, bounded: bool) {
        self.is_bounded = bounded;
    }

    pub fn set_should_wait_until_end(&mut self, value: bool) {
        self.should_wait_until_end = value;
    }

    pub fn set_dimensions(&mut self, value: i32) {
        self.dimensions = value;
    }

    pub fn set_use_z_alpha(&mut self, value: bool) {
        self.use_z_alpha = value;
    }

    fn is_time_passed(&self) -> bool {
        self.clock >= self.time
    }

    /// Returns time modeled by 1 iteration
    fn tick_time(&self) -> f64 {
        1. / self.iter_per_sec as f64
    }

    fn tick(&mut self) {
        self.clock += self.tick_time();
    }

    pub fn is_finished(&self) -> bool {
        self.is_time_passed() && !(self.should_wait_until_end && self.is_still_growing)
    }

    fn emergency_stop(&mut self) {
        self.clock = self.time;
    }

    /// Indicates if the current time (self.clock) is approximately equal to xxx.0
    /// This is important because the time may not change by exactly 1 second
    fn is_second_finished(&self) -> bool {
        self.clock - self.clock.floor() < self.tick_time()
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

    pub fn set_hungry(&mut self, value: bool) {
        self.is_hungry = value;
    }

    pub fn set_neighbour_limit(&mut self, value: i32) {
        self.neighbour_limit = value;
    }


    pub fn get_data_S(&self) -> js_sys::Float64Array {
        js_sys::Float64Array::from(&self.data_S[..])
    }

    pub fn get_data_N(&self) -> js_sys::Int32Array {
        js_sys::Int32Array::from(&self.data_N[..])
    }

    pub fn get_data_T(&self) -> js_sys::Float64Array {
        js_sys::Float64Array::from(&self.data_T[..])
    }

    /// Returns a sorted array of area/volumes of the circles that are present on the field
    pub fn get_data_size_distrib(&self) -> js_sys::Float64Array {
        let mut sizes = Vec::<f64>::new();

        for circle in &self.circles {
            sizes.push(circle.get_size(self.dimensions));
        }

        sizes.sort_by(|a, b| a.partial_cmp(b).unwrap());

        js_sys::Float64Array::from(&sizes[..])
    }

    fn can_put_circle(&self, circle: &Circle) -> bool {
        for my_circle in &self.circles {
            if my_circle.intersects(circle) {
                return false;
            }
        }

        true
    }
    
    fn has_y(&self) -> bool {
        self.dimensions >= 2
    }

    fn has_z(&self) -> bool {
        self.dimensions >= 3
    }

    pub fn draw(&mut self) {
        self.tick();
        let tick_time = self.tick_time();

        if self.is_finished() {
            return;
        }

        if !self.is_time_passed() {
            // (nearly) every second
            if self.is_second_finished() {
                // Create new circle
                let mut circle = Circle::new_random_color(self.next_circle_id);

                let mut rng = rand::thread_rng();
                let mut attempts_left = 10_000i32;
                loop {
                    attempts_left -= 1;
                    if attempts_left <= 0 {
                        self.emergency_stop();
                    }

                    circle.x = rng.gen_range(0. .. CANVAS_SIZE);

                    circle.y =
                        if self.has_y()
                            {rng.gen_range(0. .. CANVAS_SIZE)}
                        else
                            {CANVAS_SIZE / 2.};

                    circle.z =
                        if self.has_z()
                            {rng.gen_range(0. .. CANVAS_SIZE)}
                        else
                            {CANVAS_SIZE / 2.};
                            
                    if self.can_put_circle(&circle) {
                        break;
                    }
                }

                if !self.has_z() {
                    self.circles.push(circle);
                } else {
                    // Insert sorted by Z
                    let mut index = 0usize;
                    while index < self.circles.len() && self.circles[index].z >= circle.z {
                        index += 1;
                    }
                    self.circles.insert(index, circle);
                }
                
                if self.should_gen_T {
                    self.data_T.push(0.);
                }

                self.next_circle_id += 1;
            }
        }

        for circle in &mut self.circles {
            circle.grow(self.speed * tick_time);
            circle.grow_older(tick_time);
        }

        self.begin_drawing();

        // 1. Draw circles;  2. Find out if there are still growing circles
        self.is_still_growing = false;
        let mut n_active = 0i32;
        for circle in &self.circles {
            self.draw_circle(circle);

            if circle.is_active {
                n_active += 1;
                self.is_still_growing = true;
            }
        }

        self.end_drawing();

        // Generate data
        if self.is_second_finished() {
            if self.should_gen_S {
                let mut S = 0.0f64;
                for circle in &self.circles {
                    S += circle.get_size(self.dimensions);
                }
                self.data_S.push(S);
            }

            if self.should_gen_N {
                self.data_N.push(n_active);
            }

            if self.should_gen_T {
                for circle in &self.circles {
                    self.data_T[circle.id as usize] = circle.life_length;
                }
            }
        }


        if self.is_hungry {
            for circle in &mut self.circles {
                circle.clear_neighbours();
            }
        }


        for i in 0..self.circles.len() {

            if self.is_bounded && self.circles[i].out_of_bounds() {
                self.circles[i].deactivate();
            }

            for j in (i + 1)..self.circles.len() {
                if self.circles[i].intersects(&self.circles[j]) {
                    self.circles[i].deactivate();
                    self.circles[j].deactivate();

                    if self.is_hungry {
                        self.circles[i].add_neighbour();
                        self.circles[j].add_neighbour();
                    }
                }
            }
        }


        if self.is_hungry {
            let mut i = 0;
            while i < self.circles.len() {
                if self.circles[i].is_jammed(self.neighbour_limit) {
                    self.circles.remove(i);
                    continue;
                } else if self.circles[i].is_free() {
                    self.circles[i].activate();
                }
                i += 1;
            }
        }

    }
}
