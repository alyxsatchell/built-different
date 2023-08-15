use crate::player::Player;
use crate::space::Space;
use crate::vector::Point;
use crate::velocity::Velocity;
use crate::object::Object;

use core::time;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Arc;
use std::thread::{self, JoinHandle, sleep};
use std::io;
use std::mem::swap;

const COMMAND_LIST: &'static [&'static str] = &["w", "s", "a", "d"];

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
    handle: Option<JoinHandle<()>>,
    sender: Option<Sender<msg>>,
    dormant: bool
}

impl InputWorker{
    fn new(tx_space: Sender<msg>) -> InputWorker{
        let handle = None;
        let dormant = true;
        let sender = Some(tx_space);
        InputWorker { handle, sender, dormant}
    }

    fn start(&mut self){
        let mut sender = None;
        swap(&mut sender, &mut self.sender);
        self.dormant = false;
        let mut handle = Some(thread::spawn(move || {
            let tx_space = sender.unwrap();
            loop{
                let input = get_input();
                if input.is_some(){
                    let (id, cmd, x) = input.unwrap();
                    println!("id: {}, cmd: {}, x: {}", id, cmd, x);
                    match cmd{
                        0 => {
                                             let closure = Arc::new(move |obj: &mut dyn Object| {
                                obj.accelerate(x, 0.);
                                println!("{}", &obj.get_velocity().vector);
                            });
                            println!("sent");
                            tx_space.send((id, closure,x)).expect("command failed");           
                        }
                        1 => {
                            let closure = Arc::new(move |obj: &mut dyn Object| {
                                obj.accelerate(x, 0.);
                                println!("{}", &obj.get_velocity().vector);
                            });
                            println!("sent");
                            tx_space.send((id, closure,x)).expect("command failed");
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
            }
        }));
        swap(&mut handle, &mut self.handle);
    }
}

pub struct SpaceWorker{
    pub handle: Option<JoinHandle<()>>,
    receiver: Option<Receiver<msg>>,
    dormant: bool
}

impl SpaceWorker{
    fn new(rx_space: Receiver<msg>) -> SpaceWorker{
        let handle = None;
        let receiver = Some(rx_space);
        let dormant = false;
        SpaceWorker {handle, receiver, dormant}
    }

    fn start(&mut self){
        let mut receiver = None;
        swap(&mut receiver, &mut self.receiver);
        self.dormant = false;
        let handle = thread::spawn(move || {
            let rx_space = receiver.unwrap();
            let mut space = set_up(rx_space);
            loop{
                //checks if a command has been issued and runs it on the object
                match space.rx.try_recv(){
                    Ok(t) => {
                        let (id, f, _) = t;
                        f(&mut **space.players[id].lock().unwrap());
                        println!("received");
                    },
                    Err(_) => (),
                }
                space.tick();
                sleep(time::Duration::from_millis(100));
            }
        });
        swap(&mut Some(handle), &mut self.handle);
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

    pub fn start(&mut self){
        self.space_worker.start();
        self.input_worker.start();
    }

    pub fn run(&mut self){
        self.start();
        let mut handle: Option<JoinHandle<()>> = None;
        swap(&mut handle, &mut self.space_worker.handle);
        handle.unwrap().join().expect("join failed");
    }
}

fn set_up(rx: Receiver<msg>) -> Box<Space>{
    let cor = 1.;
    let player1 = Player::create(Velocity::new(Point { x: 10., y: 10. }, 1., 0.));
    let player2 = Player::create(Velocity::new(Point { x: 20., y: 20. }, 0., 0.));
    let mut space = Space::new(player1, player2, cor, rx);
    space.update_canvas();
    Box::new(space)
}

fn read_input() -> (String, usize){
    let mut string = String::new();
    Input::get_input(&mut string, "");
    let id = 0;
    return (string, id);
}

pub fn parse_input(input: (String, usize)) -> Option<(usize, usize, f64)>{
    let (string, id) = input;
    let components: Vec<&str>= string.split(" ").collect();
    let cmd = components[0];
    let parameter = components[1].trim_end().parse::<f64>();
    if parameter.is_err(){
        return None
    }
    let mut index = usize::MAX;
    for i in 0..COMMAND_LIST.len(){
        if COMMAND_LIST[i] == cmd{
            index = i;
            break;
        }
    }
    if index == usize::MAX{
        return None
    }
    return Some((id, index, parameter.unwrap()));
}

pub fn get_input() -> Option<(usize, usize, f64)>{
    let input = parse_input(read_input());
    if input.is_none(){
        return None
    }
    Some(input.unwrap())
}