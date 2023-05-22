use built_different_lib::{self, velocity::{Velocity}, vector::Point, player::Player};
use image::{self, Frame, ImageBuffer, Rgba, open};

use crate::built_different_lib::space::Space;


use std::{io};

struct Input{
    mass1: f64,
    mass2: f64,
    v1: Velocity,
    v2: Velocity,
    cor: f64
}

impl Input{
    fn get_input(mut variable: &mut String, message: &str){
        println!("{}", message);
        io::stdin().read_line(&mut variable).expect("failed to read line");
    }

    pub fn new() -> Input{
        let mut point1_i = String::new();
        let mut point2_i = String::new();
        let mut mass1_i = String::new();
        let mut mass2_i = String::new();
        let mut cor_i = String::new();
        let mut v1_i = String::new();
        let mut v2_i = String::new();
        println!("Welcome To Built Different, A Physics Simulator\nThe Format x,y Is Used When Entering Points Or Vectors. Please Omit Units.\nEnter The Following Parameters:");
        Input::get_input(&mut point1_i, "For Object 1:\nStarting Point: ");
        Input::get_input(&mut mass1_i, "Mass: ");
        Input::get_input(&mut v1_i, "Initial Velocity");
        Input::get_input(&mut point2_i, "For Object 2:\nStarting Point: ");
        Input::get_input(&mut mass2_i, "Mass: ");
        Input::get_input(&mut v2_i, "Initial Velocity");
        Input::get_input(&mut cor_i, "Coefficient Of Restitution");
        let p1: Vec<&str> = point1_i.split("\n").collect::<Vec<&str>>()[0].split(",").collect();
        let point1 = Point{x: p1[0].parse::<f64>().unwrap(), y: p1[1].parse::<f64>().unwrap()};
        let p2: Vec<&str> = point2_i.split("\n").collect::<Vec<&str>>()[0].split(",").collect();
        let point2 = Point{x: p2[0].parse::<f64>().unwrap(), y: p2[1].parse::<f64>().unwrap()};
        let mass1 = mass1_i.split("\n").collect::<Vec<&str>>()[0].parse::<f64>().unwrap();
        let mass2 = mass2_i.split("\n").collect::<Vec<&str>>()[0].parse::<f64>().unwrap();
        let cor = cor_i.split("\n").collect::<Vec<&str>>()[0].parse::<f64>().unwrap();
        let velocity1: Vec<&str> = v1_i.split("\n").collect::<Vec<&str>>()[0].split(",").collect();
        let velocity2: Vec<&str> = point1_i.split("\n").collect::<Vec<&str>>()[0].split(",").collect();
        let v1 = Velocity::new(point1, velocity1[0].parse::<f64>().unwrap(), velocity1[1].parse::<f64>().unwrap());
        let v2 = Velocity::new(point2, velocity2[0].parse::<f64>().unwrap(), velocity2[1].parse::<f64>().unwrap());
        Input{mass1,mass2,v1,v2, cor}
    }
}

fn set_up() -> Box<Space>{
    let input = Input::new();
    let player1 = Player::create(input.v1, input.mass1);
    let player2 = Player::create(input.v2, input.mass2);
    Box::new(Space::new(player1, player2, input.cor))
}

fn default_set_up() -> Box<Space>{
    let player1 = Player::create(Velocity::new(Point{x:0.,y: 0.}, 1., 0.), 1.);
    let player2 = Player::create(Velocity::new(Point { x: 7., y: 0. }, 0., 0.), 1.);
    Box::new(Space::new(player1, player2, 1.))
}

fn inverse_default_set_up() -> Box<Space>{
    let player1 = Player::create(Velocity::new(Point{x:50.,y: 50.}, 0., 0.), 1.);
    let player2 = Player::create(Velocity::new(Point { x: 57., y: 50. }, -1., 0.), 1.);
    Box::new(Space::new(player1, player2, 1.))
}

fn glance() -> Box<Space>{
    let player1 = Player::create(Velocity::new(Point{x:50.,y: 50.}, 1.5, 5.5), 1.);
    let player2 = Player::create(Velocity::new(Point { x: 25., y: 25. }, -1., 3.5), 1.);
    Box::new(Space::new(player1, player2, 1.))
}

fn main(){
    // let mut space = set_up();
    // let mut space = default_set_up();
    // let mut space = inverse_default_set_up();
    let mut space = glance();
    let mut num_of_ticks: String = String::new();
    Input::get_input(&mut num_of_ticks, "Input Number Of Simulated Ticks");
    for i in 0..num_of_ticks.split("\n").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(){
        space.turn();
        let img1 = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(100, 100, space.push_canvas()).unwrap();
        let filename = format!("images/{i}.png");
        img1.save(&filename);
    }
}