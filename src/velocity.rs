use wasm_bindgen::prelude::*;
use rand::prelude::*;
use std::ops::{Index, IndexMut, Add, Sub, AddAssign};

use crate::vector::{Vector, Point};

pub struct Velocity{
    pub origin: Point,
    pub vector: Vector
}

impl Velocity{
    pub fn new(origin: Point, x: f64, y: f64) -> Velocity{
        let vector = Vector::new(x,y);
        Velocity{origin, vector}
    }

    pub fn new_rand() -> Velocity{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..=5) as f64;
        let y = rng.gen_range(0..=5) as f64;
        let magnitude = ((f64::powf(x,2.0) + f64::powf(y,2.0)) as f64).sqrt();
        return Velocity{origin: Point::new_rand(), vector: Vector::new(x,y)}
    }

    pub fn zero() -> Velocity{
        return Velocity{origin: Point{x:0.0, y:0.0}, vector: Vector::zero()}
    }

    pub fn translate(&mut self, map_size: &Point){
        let (new_x, new_y): (f64,f64) = (self.origin[0] + (self.vector.x), self.origin[1] + (self.vector.y));
        if new_x >= map_size[0] || new_x < 0.0{
            self.vector.x *= -1.0;
        }
        if new_y >= map_size[1] || new_y < 0.0{
            self.vector.y *= -1.0;
        }
        self.origin = Point{x: new_x, y: new_y};
    }
}

impl Index<usize> for Velocity {
    type Output = f64;
    fn index(&self, s: usize) -> &f64 {
        match s {
            0 => &self.vector.x,
            1 => &self.vector.y,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<usize> for Velocity {
    fn index_mut(&mut self, s: usize) -> &mut f64{
        match s {
            0 => &mut self.vector.x,
            1 => &mut self.vector.y,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl Add<Vector> for Velocity {
    type Output = Velocity;
    fn add(self, other: Vector) -> Velocity {
        let vector = self.vector + other;
        return Velocity { origin: self.origin, vector}
    }
}

impl Sub<Vector> for Velocity{
    type Output = Velocity;
    fn sub(self, other: Vector) -> Velocity {
        let vector = self.vector - other;
        return Velocity { origin: self.origin, vector}
    } 
}

impl AddAssign<Vector> for Velocity{
    fn add_assign(&mut self, other: Vector) {
        self.vector += other;
    }
}