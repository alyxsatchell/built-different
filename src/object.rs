use constellation::{stencil::{Stencil, StencilMap}, canvas::Tile};

use crate::{vector::{Point, Vector}, velocity::Velocity, material::Material};

use std::collections::HashMap;

pub trait Object{

    fn get_mass(&self) -> &f64;

    fn get_body(&self) -> &Body;

    fn get_body_mut(&mut self) -> &mut Body;

    fn get_velocity(&self) -> &Velocity;

    fn get_velocity_mut(&mut self) -> &mut Velocity;

    fn accelerate(&mut self, x: f64, y: f64); //currently used while I get force working

    fn accelerate_force(&mut self, f: Vector); //For use once force is implemented

    fn get_pos(&self) -> &Point;

    fn make_body(&mut self, size: f64);

    fn draw(&self) -> Vec<(Point, Material)>;

    fn get_size(&self) -> f64;

    fn translate_pos(&self, t: f64) -> Point;

    fn collide(&mut self, other: &mut dyn Object, cor: f64) -> Option<f64>;

    fn get_edge_material(&self, n: &Vector) -> Material;
}

impl Stencil for dyn Object{
    fn get_map(&self) -> &constellation::stencil::StencilMap {
        &self.get_body().stencilmap
    }

    fn get_map_mut(&mut self) -> &mut constellation::stencil::StencilMap {
        &mut self.get_body_mut().stencilmap
    }
}

pub struct Body{
    pub grid: Vec<(Point,Material)>,
    pub stencilmap: StencilMap,
    pub size: f64,
    pub base_material: Material,
    pub mass: f64,
}

impl Body{
    pub fn new(size: f64, grid: Vec<(Point, Material)>, base_material: Material, mass: f64) -> Body{
        let mut new_grid = Vec::new();
        let mut stencilmap_map: HashMap<constellation::canvas::Point, Tile> = HashMap::new();
        for (point, material) in grid{
            let tile = Tile::new(material.color.into());
            stencilmap_map.insert(point.into(), tile);
            new_grid.push((point, material));
        }
        let stencilmap = StencilMap::new(constellation::canvas::Point{x:0,y:0}, stencilmap_map);
        Body{mass, size, grid: new_grid, base_material, stencilmap}
    }

    pub fn null_body() -> Body{
        Body { mass: 0., grid: Vec::new(), size: 0., base_material: Material::null_material(), stencilmap: StencilMap { origin: constellation::canvas::Point{x:0,y:0}, addition_map: HashMap::new(), subtraction_map: Vec::new(), current_map: HashMap::new() }}
    }
}