use wasm_bindgen::prelude::wasm_bindgen;

use crate::{vector::Vector, vector::Point, space::Color};

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

pub struct Player{
    pub size: i32,
    pub vector: Vector,
    pub speed: i32,
    pub color: Color,
    pub occupied_space: Vec<Point>
}


impl Player {
    pub fn new() -> Player{
        let size = 2;
        let vector = Vector::zero();
        let speed = 1;
        let color = Color::new(255,255,255,255);
        return Player {size, vector, speed, color, occupied_space: Vec::new()}
    }

    // pub fn accelerate(&mut self, direction: Direction){
    //     let direction_vector = Vector::direction_vector(self.speed, direction);
    //     self.vector += direction_vector;
    // }
    pub fn accelerate(&mut self, x: i32, y: i32){
        let acceleration_vector = Vector{origin: Point{x:0,y:0}, x: self.speed * x, y: self.speed * y, modifier: 1, magnitude: 0.0};
        self.vector += acceleration_vector;
    }


    pub fn make_circle(&self) -> Vec<Point>{
        let mut circle_vec = Vec::new();
        let origin = self.vector.origin.clone();
        for x in 0..=self.size{
            for y in 0..=self.size{
                if calc_circle(x, y) <= self.size{
                    circle_vec.push(Point{x: origin.x + x, y: origin.y + y});
                    circle_vec.push(Point{x: origin.x - x, y: origin.y + y});
                    circle_vec.push(Point{x: origin.x + x, y: origin.y - y});
                    circle_vec.push(Point{x: origin.x - x, y: origin.y - y});
                }
                //adds a point in all 4 quads (circle is symmetrical in all directions)
                // circle_vec.push(Point{x: origin.x + x, y: origin.y + y});
                // circle_vec.push(Point{x: origin.x - x, y: origin.y + y});
                // circle_vec.push(Point{x: origin.x + x, y: origin.y - y});
                // circle_vec.push(Point{x: origin.x - x, y: origin.y - y});
            }
        }
        return circle_vec
    }
}

pub fn calc_circle(x: i32, y: i32) -> i32{
    return (x.pow(2) as f64 + y.pow(2) as f64).sqrt().round() as i32
}