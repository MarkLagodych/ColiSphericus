use rand;
use wasm_bindgen::prelude::*;

use std::f64;
use crate::consts::*;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub r: f64,
    pub fill_style: JsValue,
    pub is_active: bool,
    
    pub id: i32, /// Unique circle identifier
    pub life_length: f64,
    pub n_neighbours: i32,
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        [self.x, self.y, self.z, self.r] == [other.x, other.y, other.z, other.r]
    }
}

impl Eq for Circle {}

impl Circle {
    pub fn from(id: i32, x: f64, y: f64, z: f64, r: f64, fill_style: &str) -> Self {
        Self {
            x,
            y,
            z,
            r,
            fill_style: JsValue::from_str(fill_style),
            is_active: true,
            life_length: 0.,
            id,
            n_neighbours: 0,
        }
    }

    pub fn new(id: i32) -> Self {
        Self::from(id, 0., 0., 0., 0., "#000000")
    }

    pub fn new_random_color(id: i32) -> Self {
        let color = format!(
            "#{:02x}{:02x}{:02x}",
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>()
        );

        Self::from(id, 0., 0., 0., 0., color.as_str())
    }

    pub fn grow(&mut self, speed: f64) {
        if self.is_active {
            self.r += speed;
        }
    }

    pub fn grow_older(&mut self, time: f64) {
        if self.is_active {
            self.life_length += time;
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn intersects(&self, other: &Self) -> bool {
        (self.x - other.x).powf(2.) +
        (self.y - other.y).powf(2.) +
        (self.z - other.z).powf(2.)
        <=
        (self.r + other.r).powf(2.)
    }

    pub fn out_of_bounds(&self) -> bool {
        (self.x <= self.r)
            || (self.y <= self.r)
            || (self.z <= self.r)
            || (self.x + self.r >= CANVAS_SIZE)
            || (self.y + self.r >= CANVAS_SIZE)
            || (self.z + self.r >= CANVAS_SIZE)
    }

    /// Returns: metres
    pub fn length(&self) -> f64 {
        self.r * 2.
    }

    /// Returns: square metres
    pub fn area(&self) -> f64 {
        f64::consts::PI * self.r.powf(2.) / 1e6 // metre^2 contains 10^6 millimetre^2
    }

    /// Returns: cubic metres
    pub fn volume(&self) -> f64 {
        0.75 * f64::consts::PI * self.r.powf(3.) / 1e9 // // metre^3 contains 10^9 millimetre^3
    }

    pub fn get_size(&self, dimensions: i32) -> f64 {
        match dimensions {
            1 => self.length(),
            2 => self.area(),
            3 => self.volume(),
            _ => panic!("Invalid number of dimensions: {}", dimensions)
        }
    }

    pub fn clear_neighbours(&mut self) {
        self.n_neighbours = 0;
    }

    pub fn add_neighbour(&mut self) {
        self.n_neighbours += 1;
    }

    // Returns true if there are too many neighbours
    pub fn is_jammed(&self, limit: i32) -> bool {
        self.n_neighbours >= limit
    }

    pub fn is_free(&self) -> bool {
        self.n_neighbours == 0
    }
}
