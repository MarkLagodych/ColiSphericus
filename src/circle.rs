use rand;
use wasm_bindgen::prelude::*;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub fill_style: JsValue,
    pub active: bool,
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.r == other.r
    }
}

impl Eq for Circle {}

impl Circle {
    pub fn from(x: f64, y: f64, r: f64, fill_style: &str) -> Self {
        Self {
            x,
            y,
            r,
            fill_style: JsValue::from_str(fill_style),
            active: true,
        }
    }

    pub fn new() -> Self {
        Self::from(0., 0., 0., "#000000")
    }

    pub fn new_random_color() -> Self {
        let color = format!(
            "#{:02x}{:02x}{:02x}",
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>()
        );

        Self::from(0., 0., 0., color.as_str())
    }

    pub fn grow(&mut self, speed: f64) {
        if (self.active) {
            self.r += speed;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn intersects(&self, other: &Self) -> bool {
        (self.x - other.x).powf(2.) + (self.y - other.y).powf(2.) <= (self.r + other.r).powf(2.)
    }

    pub fn out_of_bounds(&self) -> bool {
        (self.r >= self.x)
            || (self.r >= self.y)
            || (self.r >= (1000. - self.x))
            || (self.r >= (1000. - self.y))
    }
}