use wasm_bindgen::prelude::wasm_bindgen;

use crate::{vector::Vector, vector::Point, space::Color, velocity::Velocity, material::Material, object::Object};

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

pub struct Body{
    pub grid: Vec<Vec<Material>>,
    pub size: f64,
    pub edges: Vec<Point>, //unclear how this will be used yet, possible ideas are list of vertices or just every edge point
    pub occupied_space: Vec<Point>
}

impl Body{
    //placeholder for now
    pub fn new() -> Body{
        Body{grid: Vec::new(), size: 0., edges: Vec::new(), occupied_space: Vec::new()}
    }
}

pub struct Player{
    pub mass: f64,
    pub body: Body,
    pub size: i32, //placeholder till i get bodies working
    pub velocity: Velocity,
    pub speed: f64, //placeholder till i get force working
    pub color: Color, //placeholder till i get bodies working
    pub occupied_space: Vec<(Point, Color)> //also placeholder till i get bodies working
}


impl Player {
    pub fn new() -> Player{
        let mass = 1.;
        let body = Body::new();
        let size = 2;
        let velocity = Velocity::zero();
        let speed = 0.5; //placeholder until momentum and force are added
        let color = Color::new(255,255,255,255);
        return Player {size, mass, body, velocity, speed, color, occupied_space: Vec::new()}
    }

    pub fn make_circle(&self) -> Vec<(Point, Color)>{
        let mut circle_vec = Vec::new();
        let origin = self.velocity.origin.clone();
        for x in 0..=self.size as i32{
            for y in 0..=self.size as i32{
                if calc_circle(x, y) <= self.size{
                    circle_vec.push((Point{x: origin.x + x as f64, y: origin.y + y as f64}, self.color.clone()));
                    circle_vec.push((Point{x: origin.x - x as f64, y: origin.y + y as f64}, self.color.clone()));
                    circle_vec.push((Point{x: origin.x + x as f64, y: origin.y - y as f64}, self.color.clone()));
                    circle_vec.push((Point{x: origin.x - x as f64, y: origin.y - y as f64}, self.color.clone()));
                }
            }
        }
        return circle_vec
    }
}

impl Object for Player{
    fn get_mass(&self) -> &f64 {
        &self.mass
    }

    fn get_body(&self) -> &Body {
        &self.body
    }

    fn get_pos(&self) -> &Point {
        &self.velocity.origin
    }

    fn get_velocity(&self) -> &Velocity {
        &self.velocity
    }

    fn get_velocity_mut(&mut self) -> &mut Velocity {
        &mut self.velocity
    }

    fn accelerate(&mut self, x:f64, y:f64) {
        let acceleration_vector = Vector::new(x,y);
        self.velocity += acceleration_vector;
    }

    fn draw(&self) -> Vec<(Point, Color)>{
        self.make_circle()
    }
}

pub fn calc_circle(x: i32, y: i32) -> i32{
    return (x.pow(2) as f64 + y.pow(2) as f64).sqrt().round() as i32
}