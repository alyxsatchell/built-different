use rand::prelude::*;
use std::ops::{Index, IndexMut, Add, Sub, AddAssign};

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

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point {x, y}
    }
}

impl Sub<Point> for Point{
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        let x = self.x - other.x;
        let y = self.y - other.y;
        return Point{x,y}
    } 
}

impl AddAssign<Point> for Point{
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub struct Vector{
    pub x: f64,
    pub y: f64,
    pub magnitude: f64
}

impl Vector{
    pub fn new(x:f64, y: f64) -> Vector{
        Vector { x, y, magnitude: Vector::magnitude(x, y) }
    }

    pub fn zero() -> Vector{
        Vector { x: 0., y: 0., magnitude: 0. }
    }

    fn magnitude(x:f64, y:f64) -> f64{
        ((f64::powf(x,2.0) + f64::powf(y,2.0)) as f64).sqrt()
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let magnitude = Vector::magnitude(x, y);
        return Vector{x, y, magnitude}
    }
}

impl Sub<Vector> for Vector{
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let magnitude = ((f64::powf(x,2.0) + f64::powf(y,2.0)) as f64).sqrt();
        return Vector{x,y, magnitude}
    } 
}

impl AddAssign<Vector> for Vector{
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.magnitude = ((f64::powf(self.x,2.0) + f64::powf(self.y,2.0)) as f64).sqrt();
    }
}