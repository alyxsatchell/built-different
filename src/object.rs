use crate::{player::Body, vector::Point, velocity::Velocity, space::Color};

pub trait Object{

    fn get_mass(&self) -> &f64;

    fn get_body(&self) -> &Body;

    fn get_velocity(&self) -> &Velocity;

    fn get_velocity_mut(&mut self) -> &mut Velocity;

    fn accelerate(&mut self, x: f64, y: f64); //currently used while I get force working

    // fn accelerate_force(&mut self, f: Force); //For use once force is implemented

    fn get_pos(&self) -> &Point;

    fn draw(&self) -> Vec<(Point, Color)>;
}