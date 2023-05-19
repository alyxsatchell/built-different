use crate::{vector::Vector, vector::Point, space::Color, velocity::Velocity, object::{Object, Body}, material::Material, physics::{post_collision_velocity, calculate_impulse}};

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
    pub fn new(x: f64, y: f64) -> Player{
        let mass = 1.;
        let body = Body::null_body();
        let velocity = Velocity::new(Point{x: x,y: y}, 0., 0.);
        let speed = 0.5; //placeholder until momentum and force are added
        let color = Color::new(255,255,255,255);
        let mut player = Player {mass, body, velocity, speed, color, occupied_space: Vec::new()};
        player.make_body(2.);
        // println!("bod length during const {}", &player.get_body().grid.len());
        return player
    }

    pub fn create(velocity: Velocity, mass: f64) -> Player{
        let body = Body::null_body();
        let speed = 0.5; //placeholder until momentum and force are added
        let color = Color::new(255,255,255,255);
        let mut player = Player {mass, body, velocity, speed, color, occupied_space: Vec::new()};
        player.make_body(2.);
        // println!("bod length during const {}", &player.get_body().grid.len());
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

    fn make_body(&mut self, size: f64) {
        let base_material = Material{density: 1., color: Color { r: 255, b: 255, g: 255, a: 255 }, coefficient_of_restitution: 1.};
        let circle = make_circle(size, base_material.clone(), &self.velocity.origin);
        // println!("circle len {}", &circle.len());
        self.body = Body::new(size, circle, base_material)
    }

    fn draw(&self) -> Vec<(Point, Material)>{
        println!("Player At {},{} With A Velocity Of {},{}", &self.velocity.origin.x, &self.velocity.origin.y, &self.velocity.vector.x, &self.velocity.vector.y);
        make_circle(self.body.size, self.body.base_material.clone(), &self.velocity.origin)
    }

    fn get_size(&self) -> f64 {
        return self.body.size;
    }

    fn translate_pos(&self, t: f64) -> Point {
        let x = self.velocity.origin.x + self.velocity.vector.x * t;
        let y = self.velocity.origin.y + self.velocity.vector.y * t;
        return  Point{x,y};
    }

    fn collide(&mut self, other: &mut dyn Object, cor: f64) -> Option<f64>{
        let velocities = post_collision_velocity(self, other, cor);
        if velocities.is_none(){
            // println!("its none");
            return None
        }
        let (vf1, vf2, t) = velocities.unwrap();
        if t.is_nan(){
            // println!("its nan");
            return None
        }
        if t > 1.{
            return Some(t)
        }
        let j = calculate_impulse(self.mass, &self.velocity.vector, &vf1);
        println!("The Impulse Of The Collision On Body 1 Was: {}", j.0);
        self.velocity.vector = vf1;
        other.get_velocity_mut().vector = vf2;
        return Some(t)
    }

    fn get_edge_material(&self, n: &Vector) -> Material{
        // println!("or else");
        let mut material: Material = Material::null_material();
        let resized_n = n.resize(self.body.size);
        let p = Point{x: resized_n.x + self.velocity.origin.x, y: resized_n.y + self.velocity.origin.y};
        // println!("{}, {}", &p.x, &p.y);
        for (point, mat) in &self.body.grid{
            if p == *point{
                // println!("this line needs to get read");
                material = mat.clone();
                break
            }
        }
        return material
    }

    fn get_body_mut(&mut self) -> &mut Body {
        return &mut self.body;
    }
}

//hopefully this is temporary

pub fn calc_circle(x: i32, y: i32) -> i32{
    return (x.pow(2) as f64 + y.pow(2) as f64).sqrt().round() as i32
}

pub fn make_circle(size: f64, base_material: Material, origin: &Point) -> Vec<(Point, Material)>{
    let mut circle_vec = Vec::new();
    // let origin = Point{x:0.,y:0.};
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