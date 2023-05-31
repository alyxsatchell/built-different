use crate::player::Player;
use crate::space::{Space};
use crate::vector::{Point, Vector};
use crate::velocity::{Velocity};
use crate::object::Object;

use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::{io};

const BROWSERS: &'static [&'static str] = &["firefox", "chrome"];
const COMMAND_LIST: &'static [&'static str; 4] = &["w", "s", "a", "d"];

pub type msg = (usize, Arc<dyn Fn(&mut dyn Object) -> () + Send + Sync>, f64); //the type of data that is sent for messages
//the closure will be the command that is to be executed on the object at the given usize in the array and the f64 will be the parameter for the closure

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

pub struct InputWorker{
    handle: JoinHandle<()>,
}

impl InputWorker{
    fn new(tx_space: Sender<msg>) -> InputWorker{
        let handle = thread::spawn(move || {
        // let (id, cmd, x) = get_input();
        let input = get_input();
        if input.is_some(){
            let (id, cmd, x) = input.unwrap();
            match cmd{
                1 => {
                    let closure = Arc::new(move |obj: &mut dyn Object| {
                        obj.accelerate(x, 0.)
                    });
                    tx_space.send((id, closure,x)).expect("command failed");
                    println!("sent")
                },
                2 => {
                    let closure = Arc::new( move|obj: &mut dyn Object| {
                        obj.accelerate(0., x)
                    });
                    tx_space.send((id, closure,x)).expect("command failed");
                },
                _ => {()}
            };
        }
        });
        InputWorker { handle}
    }
}

pub struct SpaceWorker{
    pub handle: JoinHandle<()>,
}

impl SpaceWorker{
    fn new(rx_space: Receiver<msg>) -> SpaceWorker{
        let handle = thread::spawn(move || {
            let mut space = set_up(rx_space);
            for _ in 0..10{
                //checks if a command has been issued and runs it on the object
                match space.rx.try_recv(){
                    Ok(t) => {
                        let (id, f, _) = t;
                        f(&mut **space.players[id].lock().unwrap());
                    },
                    Err(_) => (),
                }
                space.tick();
            }
        });
        SpaceWorker {handle}
    }
}


pub struct Universe{
    pub space_worker: SpaceWorker,
    pub input_worker: InputWorker,
}

impl Universe{
    pub fn new() -> Universe{
        let (tx_space, rx_space) = mpsc::channel(); //sends the input data to space
        let input_worker = InputWorker::new(tx_space);
        let space_worker = SpaceWorker::new(rx_space);
        Universe { space_worker, input_worker }
    }
}

fn set_up(rx: Receiver<msg>) -> Box<Space>{
    // let input = Input::new();
    let cor = 1.;
    let player1 = Player::create(Velocity::new(Point { x: 10., y: 10. }, 0., 0.));
    let player2 = Player::create(Velocity::new(Point { x: 20., y: 10. }, 0., 0.));
    Box::new(Space::new(player1, player2, cor, rx))
}

fn read_input() -> (&'static str, usize){
    return ("d1", 0);
}

fn parse_input(input: (&str, usize)) -> (usize, &str, f64){
    //TODO: get commands separated from parameters
    return (0, "d", 1.);
}

fn get_input() -> Option<(usize, usize, f64)>{
    let (id, cmd, x) = parse_input(read_input());
    let cmd_result = COMMAND_LIST.binary_search(&cmd);
    match cmd_result{
        Ok(t) => Some((id, t, x)),
        Err(_) => None
    }
}