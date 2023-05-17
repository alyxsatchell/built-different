use crate::{vector::{Point, Vector}, velocity::Velocity, space::Color, material::Material};

pub trait Object{

    fn get_mass(&self) -> &f64;

    fn get_body(&self) -> &Body;

    fn get_velocity(&self) -> &Velocity;

    fn get_velocity_mut(&mut self) -> &mut Velocity;

    fn accelerate(&mut self, x: f64, y: f64); //currently used while I get force working

    // fn accelerate_force(&mut self, f: Force); //For use once force is implemented

    fn get_pos(&self) -> &Point;

    fn make_body(&self, size: f64) -> Body;

    fn draw(&self) -> Vec<(Point, Material)>;

    fn get_size(&self) -> f64;

    fn translate_pos(&self, t: f64) -> Point;

    fn collide(&mut self, other: &mut dyn Object);

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