use crate::player::Player;
use crate::space::{Space};
use crate::vector::{Point};
use crate::velocity::{Velocity};

use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};
use std::{io};

pub type msg = (usize, Box<dyn Fn(i32) -> i32>, f64);

pub struct Input{
    mass1: f64,
    mass2: f64,
    v1: Velocity,
    v2: Velocity,
    cor: f64
}

impl Input{
    pub fn get_input(mut variable: &mut String, message: &str){
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

struct InputWorker{
    tx_space: Sender<msg>,
    handle: JoinHandle<()>,
}

impl InputWorker{
    fn new(tx_space: Sender<msg>) -> InputWorker{
        let handle = thread::spawn(|| {});
        InputWorker { tx_space, handle}
    }
}

struct SpaceWorker{
    handle: JoinHandle<()>,
}

impl SpaceWorker{
    fn new(rx_space: Receiver<msg>) -> SpaceWorker{
        let mut space = set_up(rx_space);
        let handle = thread::spawn(move || {
            for i in 0..10{
                space.tick();
            }
        });
        SpaceWorker {handle}
    }
}


pub struct Universe{
    space_worker: SpaceWorker,
    input_worker: InputWorker,
}

impl Universe{
    pub fn new() -> Universe{
        let (tx_space, rx_space) = mpsc::channel(); //sends the input data to space
        let space_worker = SpaceWorker::new(rx_space);
        let input_worker = InputWorker::new(tx_space);
        Universe { space_worker, input_worker }
    }
}

fn set_up(rx: Receiver<msg>) -> Box<Space>{
    let input = Input::new();
    let player1 = Player::create(input.v1, input.mass1);
    let player2 = Player::create(input.v2, input.mass2);
    Box::new(Space::new(player1, player2, input.cor, rx))
}