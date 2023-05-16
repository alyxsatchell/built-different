use crate::{vector::Vector, vector::Point, space::Color, velocity::Velocity, object::{Object, Body}, material::Material};

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

pub struct Player{
    pub mass: f64,
    pub body: Body,
    pub velocity: Velocity,
    pub speed: f64, //placeholder till i get force working
    pub color: Color, //placeholder till i get bodies working
    pub occupied_space: Vec<(Point, Color)> //also placeholder till i get bodies working
}


impl Player {
    pub fn new() -> Player{
        let mass = 1.;
        let body = Body::null_body();
        let velocity = Velocity::zero();
        let speed = 0.5; //placeholder until momentum and force are added
        let color = Color::new(255,255,255,255);
        let player = Player {mass, body, velocity, speed, color, occupied_space: Vec::new()};
        player.make_body(2.);
        return player
    }

    pub fn tester(origin: Point, x: f64, y: f64, size: f64) -> Player{
        let mut p1 = Player{mass:0., body: Body::null_body(), velocity: Velocity::new(origin, x, y), speed: 0., color: Color::new(0, 0,0,0), occupied_space: Vec::new()};
        p1.body.size = size;
        p1
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

    fn make_body(&self, size: f64) -> Body {
        let base_material = Material{density: 1., color: Color { r: 255, b: 255, g: 255, a: 255 }};
        let circle = make_circle(size, base_material.clone());
        Body::new(size, circle, base_material)
    }

    fn draw(&self) -> Vec<(Point, Material)>{
        make_circle(self.body.size, self.body.base_material.clone())
    }

    fn get_size(&self) -> f64 {
        return self.body.size;
    }
}

pub fn calc_circle(x: i32, y: i32) -> i32{
    return (x.pow(2) as f64 + y.pow(2) as f64).sqrt().round() as i32
}

pub fn make_circle(size: f64, base_material: Material) -> Vec<(Point, Material)>{
    let mut circle_vec = Vec::new();
    let origin = Point{x:0.,y:0.};
    for x in 0..=size as i32{
        for y in 0..=size as i32{
            if calc_circle(x, y) <= size as i32{
                circle_vec.push((Point{x: origin.x + x as f64, y: origin.y + y as f64}, base_material.clone()));
                circle_vec.push((Point{x: origin.x - x as f64, y: origin.y + y as f64}, base_material.clone()));
                circle_vec.push((Point{x: origin.x + x as f64, y: origin.y - y as f64}, base_material.clone()));
                circle_vec.push((Point{x: origin.x - x as f64, y: origin.y - y as f64}, base_material.clone()));
            }
        }
    }
    return circle_vec
}