use std::boxed::Box;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::Receiver;

use constellation::canvas::Canvas;
use constellation::stencil::Stencil;

use crate::player::Player;
use crate::object::Object;
use crate::universe::msg;

type ObjectCell = Arc<Mutex<Box<dyn Object>>>;

#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub struct Color{
    pub r: u8,
    pub b: u8,
    pub g: u8,
    pub a: u8
}
impl Color{

    pub fn new(r: u8, g:u8, b: u8, a: u8) -> Color{
        return Color{r,g,b,a}
    }

    pub fn black() -> Color{
        return Color{r:0, b:0, g:0, a:0}
    }

    pub fn get_values(&self) -> (u8,u8,u8,u8){
        return (self.r, self.g, self.b, self.a)
    }
}

impl From<Color> for constellation::canvas::Color{
    fn from(value: Color) -> Self {
        return constellation::canvas::Color::new(value.r, value.g, value.b, true)
    }
}

impl From<constellation::canvas::Color> for Color{
    fn from(value: constellation::canvas::Color) -> Self {
        return Color::new(value.r, value.g, value.b, 255);
    }
}

pub struct Space{
    cor: f64,
    pub players: Vec<ObjectCell>,
    pub canvas: Canvas,
    pub rx: Receiver<msg>
}

impl Space{
    pub fn new(player1: Player, player2: Player, cor: f64, rx: Receiver<msg>) -> Space{
        let canvas: Canvas = Canvas::new((1,1), (50,30), constellation::canvas::Color::new(0,0,0,true));
        let player1: ObjectCell = Arc::new(Mutex::new(Box::new(player1)));
        let player2: ObjectCell = Arc::new(Mutex::new(Box::new(player2)));
        let players: Vec<ObjectCell> = vec![player1, player2];
        Space {canvas, players, cor, rx}
    }

    pub fn update_canvas(&mut self){
        for object in &mut self.players{
            self.canvas.update(object.lock().unwrap().get_body_mut().get_map_mut());
        }
    }

    fn draw(&mut self){
        self.canvas.draw();
    }

    fn get_collision_pairings(&self) -> Vec<(usize, usize)>{
        return vec![(0,1)];
    }

    pub fn turn(&mut self){
        let collision_pairings = self.get_collision_pairings();
        for (x,y) in collision_pairings{
            let mut obj_1 = self.players[x].lock().unwrap();
            let mut obj_2 = self.players[y].lock().unwrap();
            let obj_1_pos = *obj_1.get_pos();
            let obj_2_pos = *obj_2.get_pos();
            // let obj_1: Rc<RefCell<Box<dyn Object>>> = self.players[x].clone();
            // let obj_2 = self.players[y].clone();
            let magnitude1_initial = obj_1.get_velocity().vector.magnitude;
            let magnitude2_initial = obj_2.get_velocity().vector.magnitude;
            let collision_time = obj_1.collide(&mut **obj_2, self.cor);
            let t = collision_time.unwrap_or(27.);
            if t > 1. || t < 0.{
                let vec1 = obj_1.get_velocity_mut();
                vec1.translate(&self.canvas.size.into());
                let p1 = constellation::canvas::Point{x: vec1.vector.x as i32, y: vec1.vector.y as i32};
                let vec2 = obj_2.get_velocity_mut();
                vec2.translate(&self.canvas.size.into());
                let p2 = constellation::canvas::Point{x: vec2.vector.x as i32, y: vec2.vector.y as i32};
                obj_1.get_body_mut().get_map_mut().translate(p1);
                obj_2.get_body_mut().get_map_mut().translate(p2);
            }
            else{
                let mut intermediate_point_1 = obj_1.translate_pos(t);
                let mut intermediate_point_2 = obj_2.translate_pos(t);
                //finds the distance traveled before collision
                let pre_collision_vector1 = obj_1.get_pos().between(&intermediate_point_1);
                let pre_collision_vector2 = obj_2.get_pos().between(&intermediate_point_2);
                //finds the distance that still needs to be traveled
                let magnitude1 = magnitude1_initial - pre_collision_vector1.magnitude;
                let magnitude2 = magnitude2_initial - pre_collision_vector2.magnitude;
                //travels the remaining distance
                pre_collision_vector1.translate_magnitude(&mut intermediate_point_1, magnitude1);
                pre_collision_vector2.translate_magnitude(&mut intermediate_point_2, magnitude2);
                let translation_1: constellation::canvas::Point = obj_1_pos.between(&obj_1.get_pos()).into();
                let translation_2: constellation::canvas::Point = obj_2_pos.between(&obj_2.get_pos()).into();
                obj_1.get_body_mut().get_map_mut().translate(translation_1);
                obj_2.get_body_mut().get_map_mut().translate(translation_2);
            }
        }

    }

    pub fn tick(&mut self){
        self.turn();
        self.update_canvas();
        self.draw();
    }

    // pub fn accelerate(&mut self, id: i32, x: f64, y: f64){
    //     if (id as usize) < self.players.len(){
    //         self.players[id as usize].accelerate(x, y);
    //     }
    // }
}