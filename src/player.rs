use crate::{vector::Vector, vector::Point, space::Color, velocity::Velocity, object::{Object, Body}, material::Material, physics::{post_collision_velocity, calculate_impulse, calculate_kinetic_energy, round}};

const COLLISION_OUTPUT: bool = false;

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

pub struct Player{
    pub body: Body,
    pub velocity: Velocity,
}


impl Player {
    pub fn new(x: f64, y: f64) -> Player{
        let body = Body::null_body();
        let velocity = Velocity::new(Point{x: x,y: y}, 0., 0.);
        let mut player = Player {body, velocity};
        player.make_body(2.);
        return player
    }

    pub fn create(velocity: Velocity) -> Player{
        let body = Body::null_body();
        let mut player = Player {body, velocity};
        player.make_body(2.);
        return player
    }

    pub fn tester(origin: Point, x: f64, y: f64, size: f64) -> Player{
        let mut p1 = Player{body: Body::null_body(), velocity: Velocity::new(origin, x, y)};
        p1.body.size = size;
        p1
    }
}

impl Object for Player{
    fn get_mass(&self) -> &f64 {
        &self.body.mass
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
        let (circle, mass) = make_circle(size, base_material.clone(), &self.velocity.origin);
        self.body = Body::new(size, circle, base_material, mass)
    }

    fn draw(&self) -> Vec<(Point, Material)>{
        make_circle(self.body.size, self.body.base_material.clone(), &self.velocity.origin).0
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
            return None
        }
        let (vf1, vf2, t) = velocities.unwrap();
        if t.is_nan(){
            return None
        }
        if t > 1. || t < 0.{
            return Some(t)
        }
        if COLLISION_OUTPUT{
            let j = calculate_impulse(*self.get_mass(), &self.velocity.vector, &vf1);
            let kei_1 = calculate_kinetic_energy(*self.get_mass(), self.velocity.vector.magnitude);
            let kei_2 = calculate_kinetic_energy(*other.get_mass(), other.get_velocity().vector.magnitude);
            let kef_1 = calculate_kinetic_energy(*self.get_mass(), vf1.magnitude);
            let kef_2 = calculate_kinetic_energy(*other.get_mass(), vf2.magnitude);
            let kei = kei_1 + kei_2;
            let kef = kef_1 + kef_2;
            println!("Vi Of Body 1 was: {}, Vf of Body 1 was: {}\nVi of Body 2 was: {}, Vf of Body 2 was: {}", self.velocity, &vf1, &other.get_velocity(), &vf2);
            println!("The Impulse Of The Collision On Body 1 Was: {}", j.0);
            println!("The Total Kinetic Energy Before The Collision Was: {}, The Total Kinetic Energy After The Collision Was: {}, Which Is A Loss of {}", round(kei), round(kef), round(kei - kef));
        }
        self.velocity.vector = vf1;
        other.get_velocity_mut().vector = vf2;
        return Some(t)
    }

    fn get_edge_material(&self, n: &Vector) -> Material{
        let mut material: Material = Material::null_material();
        let resized_n = n.resize(self.body.size);
        let p = Point{x: resized_n.x + self.velocity.origin.x, y: resized_n.y + self.velocity.origin.y};
        for (point, mat) in &self.body.grid{
            if p == *point{
                material = mat.clone();
                break
            }
        }
        return material
    }

    fn get_body_mut(&mut self) -> &mut Body {
        return &mut self.body;
    }

    fn accelerate_force(&mut self, f: Vector) {
        let (x,y) = (f.x / *self.get_mass(), f.y / *self.get_mass());
        self.accelerate(x, y)
    }
}

pub fn calc_circle(x: i32, y: i32) -> i32{
    return (x.pow(2) as f64 + y.pow(2) as f64).sqrt().round() as i32
}

pub fn make_circle(size: f64, base_material: Material, origin: &Point) -> (Vec<(Point, Material)>, f64){
    let mut mass = 0.;
    let mut circle_vec = Vec::new();
    for x in 0..=size as i32{
        for y in 0..=size as i32{
            if calc_circle(x, y) <= size as i32{
                circle_vec.push((Point{x: origin.x + x as f64, y: origin.y + y as f64}, base_material.clone()));
                circle_vec.push((Point{x: origin.x - x as f64, y: origin.y + y as f64}, base_material.clone()));
                circle_vec.push((Point{x: origin.x + x as f64, y: origin.y - y as f64}, base_material.clone()));
                circle_vec.push((Point{x: origin.x - x as f64, y: origin.y - y as f64}, base_material.clone()));
                mass += base_material.density * 4.;
            }
        }
    }
    return (circle_vec, mass)
}