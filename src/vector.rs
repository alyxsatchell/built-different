use wasm_bindgen::prelude::*;
use rand::prelude::*;
use std::ops::{Index, IndexMut, Add, Sub, AddAssign};
use crate::player::Direction;

#[derive(Clone)]
pub struct Point{
    pub x: f64,
    pub y: f64
}

impl Point{
    pub fn new_rand() -> Point{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..=100) as f64;
        let y = rng.gen_range(0..=100) as f64;
        return Point{x,y}
    }

}

impl Index<usize> for Point {
    type Output = f64;
    fn index(&self, s: usize) -> &f64 {
        match s {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, s: usize) -> &mut f64{
        match s {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("unknown field: {}", s),
        }
    }
}

pub struct Vector{
    pub origin: Point,
    pub x: f64,
    pub y: f64,
    pub modifier: f64,
    pub magnitude: f64
}

impl Vector{
    pub fn new(origin: Point, x: f64, y: f64) -> Vector{
        let modifier = 1.0;
        let magnitude = ((f64::powf(x,2.0) + f64::powf(y,2.0)) as f64).sqrt();
        Vector{origin, x, y, magnitude, modifier}
    }

    pub fn new_rand() -> Vector{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..=5) as f64;
        let y = rng.gen_range(0..=5) as f64;
        let modifier = 1.0;
        let magnitude = ((f64::powf(x,2.0) + f64::powf(y,2.0)) as f64).sqrt();
        return Vector{origin: Point::new_rand(), x: x as f64, y: y as f64, magnitude, modifier}
    }

    pub fn zero() -> Vector{
        return Vector{origin: Point{x:0.0, y:0.0}, x: 0.0, y: 0.0, modifier: 1.0, magnitude: 0.0}
    }

    pub fn translate(&mut self, map_size: &Point){

        let (new_x, new_y): (f64,f64) = (self.origin[0] + (self.modifier * self.x), self.origin[1] + (self.modifier * self.y));
        if new_x >= map_size[0] || new_x < 0.0{
            self.x *= -1.0;
        }
        if new_y >= map_size[1] || new_y < 0.0{
            self.y *= -1.0;
        }
        self.origin = Point{x: new_x, y: new_y};
    }

    pub fn direction_vector(speed: f64, direction: Direction) -> Vector{
        match direction{
            Up => Vector::new(Point{x:0.0,y:0.0}, 0.0, speed),
            Down => Vector::new(Point{x:0.0,y:0.0}, 0.0, -speed),
            Right => Vector::new(Point{x:0.0,y:0.0}, speed, 0.0),
            Left => Vector::new(Point{x: 0.0,y: 0.0}, -speed, 0.0)
        }
    }

}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        let origin = self.origin;
        let x = self.x + other.x;
        let y = self.y + other.y;
        let magnitude = ((f64::powf(x,2.0) + f64::powf(y,2.0)) as f64).sqrt();
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
        let magnitude = ((f64::powf(x,2.0) + f64::powf(y,2.0)) as f64).sqrt();
        let modifier = self.modifier;
        return Vector{x,y,origin, magnitude, modifier}
    } 
}

impl AddAssign<Vector> for Vector{
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.magnitude = ((f64::powf(self.x,2.0) + f64::powf(self.y,2.0)) as f64).sqrt();
        self.modifier = self.modifier;
    }
}