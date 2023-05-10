use wasm_bindgen::prelude::*;
use rand::prelude::*;
use std::ops::{Index, IndexMut, Add, Sub, AddAssign};
use crate::player::Direction;

#[derive(Clone)]
pub struct Point{
    pub x: i32,
    pub y: i32
}

impl Point{
    pub fn new_rand() -> Point{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..=100);
        let y = rng.gen_range(0..=100);
        return Point{x,y}
    }

}

impl Index<usize> for Point {
    type Output = i32;
    fn index(&self, s: usize) -> &i32 {
        match s {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, s: usize) -> &mut i32 {
        match s {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("unknown field: {}", s),
        }
    }
}

pub struct Vector{
    pub origin: Point,
    pub x: i32,
    pub y: i32,
    pub modifier: i32,
    pub magnitude: f64
}

impl Vector{
    pub fn new(origin: Point, x: i32, y: i32) -> Vector{
        let modifier = 1;
        let magnitude = ((i32::pow(x,2) + i32::pow(y,2)) as f64).sqrt();
        Vector{origin, x, y, magnitude, modifier}
    }

    pub fn new_rand() -> Vector{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..=5);
        let y = rng.gen_range(0..=5);
        let modifier = 1;
        let magnitude = ((i32::pow(x,2) + i32::pow(y,2)) as f64).sqrt();
        return Vector{origin: Point::new_rand(), x, y, magnitude, modifier}
    }

    pub fn zero() -> Vector{
        return Vector{origin: Point{x:0, y:0}, x: 0, y: 0, modifier: 1, magnitude: 0.0}
    }

    pub fn translate(&mut self, map_size: &Point){

        let (new_x, new_y): (i32,i32) = (self.origin[0] + (self.modifier * self.x), self.origin[1] + (self.modifier * self.y));
        if new_x >= map_size[0] || new_x < 0{
            self.x *= -1;
        }
        if new_y >= map_size[1] || new_y < 0{
            self.y *= -1;
        }
        // web_sys::console::log_1(&self.x.into());
        // web_sys::console::log_1(&self.x.into());
        // web_sys::console::log_1(&self.y.into());
        self.origin = Point{x: new_x, y: new_y};
    }

    pub fn direction_vector(speed: i32, direction: Direction) -> Vector{
        match direction{
            Up => Vector::new(Point{x:0,y:0}, 0, speed),
            Down => Vector::new(Point{x:0,y:0}, 0, -speed),
            Right => Vector::new(Point{x:0,y:0}, speed, 0),
            Left => Vector::new(Point{x: 0,y: 0}, -speed, 0)
        }
    }

}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        let origin = self.origin;
        let x = self.x + other.x;
        let y = self.y + other.y;
        let magnitude = ((i32::pow(x,2) + i32::pow(y,2)) as f64).sqrt();
        let modifier = self.modifier;
        return Vector{x,y,origin, magnitude, modifier}
    }
}

impl Sub<Vector> for Vector{
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        let origin = self.origin;
        let x = self.x - other.x;
        let y = self.y - other.y;
        let magnitude = ((i32::pow(x,2) + i32::pow(y,2)) as f64).sqrt();
        let modifier = self.modifier;
        return Vector{x,y,origin, magnitude, modifier}
    } 
}

impl AddAssign<Vector> for Vector{
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.magnitude = ((i32::pow(self.x,2) + i32::pow(self.y,2)) as f64).sqrt();
        self.modifier = self.modifier;
    }
}