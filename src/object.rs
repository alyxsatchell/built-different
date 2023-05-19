use crate::{vector::{Point, Vector}, velocity::Velocity, space::Color, material::Material};

pub trait Object{

    fn get_mass(&self) -> &f64;

    fn get_body(&self) -> &Body;

    fn get_body_mut(&mut self) -> &mut Body;

    fn get_velocity(&self) -> &Velocity;

    fn get_velocity_mut(&mut self) -> &mut Velocity;

    fn accelerate(&mut self, x: f64, y: f64); //currently used while I get force working

    // fn accelerate_force(&mut self, f: Force); //For use once force is implemented

    fn get_pos(&self) -> &Point;

    fn make_body(&mut self, size: f64);

    fn draw(&self) -> Vec<(Point, Material)>;

    fn get_size(&self) -> f64;

    fn translate_pos(&self, t: f64) -> Point;

    fn collide(&mut self, other: &mut dyn Object, cor: f64) -> Option<f64>;

    fn get_edge_material(&self, n: &Vector) -> Material;
}

pub struct Body{
    pub grid: Vec<(Point,Material)>,
    pub size: f64,
    pub base_material: Material
}

impl Body{
    pub fn new(size: f64, grid: Vec<(Point, Material)>, base_material: Material) -> Body{
        Body{size, grid, base_material}
    }

    pub fn null_body() -> Body{
        Body { grid: Vec::new(), size: 0., base_material: Material::null_material() }
    }
}

// impl Object for Box<dyn Object>{
//     fn accelerate(&mut self, x: f64, y: f64) {
//         return self.accelerate(x, y);
//     }

//     fn collide(&mut self, other: &mut dyn Object) -> Option<f64> {
//         return self.collide(other)
//     }

//     fn draw(&self) -> Vec<(Point, Material)> {
//         return self.draw()
//     }

//     fn get_body(&self) -> &Body {
//         return self.get_body()
//     }

//     fn get_edge_material(&self, n: &Vector) -> Material {
//         return self.get_edge_material(n)
//     }

//     fn get_mass(&self) -> &f64 {
//          return self.get_mass()
//     }

//     fn get_pos(&self) -> &Point {
//         return self.get_pos()
//     }

//     fn get_size(&self) -> f64 {
//         return self.get_size()
//     }

//     fn get_velocity(&self) -> &Velocity {
//         self.get_velocity()
//     }

//     fn get_velocity_mut(&mut self) -> &mut Velocity {
//         self.get_velocity_mut()
//     }

//     fn make_body(&self, size: f64) -> Body {
//         self.make_body(size)
//     }

//     fn translate_pos(&self, t: f64) -> Point {
//         self.translate_pos(t)
//     }

//     fn get_body_mut(&mut self) -> &mut Body {
//         self.get_body_mut()
//     }
// }