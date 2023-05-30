use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;
use std::boxed::Box;

use crate::vector::{Point};
use crate::player::{Player};
use crate::object::Object;

type ObjectCell = Rc<RefCell<Box<dyn Object>>>;

const SPACE_CELL: Cell = Cell{color: Color { r: 0, b: 0, g: 0, a: 255 }, occupied: false};


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
#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub struct Cell{
    color: Color,
    occupied: bool
}

pub struct CellGrid{
    grid: Vec<Vec<Cell>>,
}

impl CellGrid{
    pub fn new(width: f64, height: f64) -> CellGrid{
        let mut grid = Vec::new();
        for h in 0..height as i32{
            grid.push(Vec::new());
            for _ in 0..width as i32{
                grid[h as usize].push(SPACE_CELL.clone());
            }
        }
        CellGrid { grid}
    }

    pub fn get(&self, p: Point) -> Option<&Cell>{
        if p.y < 0.0 || p.y >= self.grid.len() as f64{
            return None
        }
        if p.x < 0.0 || p.x >= self.grid[p.y as usize].len() as f64{
            return None
        }
        Some(&self.grid[p.y as usize][p.x as usize])
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut Cell>{
        if p.y < 0.0 || p.y >= self.grid.len() as f64{
            return None
        }
        if p.x < 0.0 || p.x >= self.grid[p.y as usize].len() as f64{
            return None
        }
        Some(&mut self.grid[p.x as usize][p.y as usize])
    }

    pub fn insert_player(&mut self, player: &mut dyn Object){
        for (i,_) in &player.get_body().grid{
            let cell = self.get_mut(i);
            if cell.is_some(){
                *cell.unwrap() = SPACE_CELL.clone();
            }
        }
        let occupied_space = player.draw();
        for (i, mat) in &occupied_space{
            let cell = self.get_mut(i);
            if cell.is_some(){
                *cell.unwrap() = Cell{color: mat.color.clone(), occupied: true};
            }
        }
        player.get_body_mut().grid = occupied_space;
    }


}

pub struct Space{
    size: Point,
    grid: CellGrid,
    cor: f64,
    // players: Vec<Rc<RefCell<dyn Object>>>,
    players: Vec<ObjectCell>,
    // player1: Box<dyn Object>,
    // player2: Box<dyn Object>,
    pub canvas: Vec<u8>
}

impl Space{
    pub fn new(player1: Player, player2: Player, cor: f64) -> Space{
        let (width, height): (f64, f64) = (100.0, 100.0);
        let canvas: Vec<u8> = vec![0;width as usize * height as usize *4];
        let mut grid = CellGrid::new(width, height);
        let player1: ObjectCell = Rc::new(RefCell::new(Box::new(Player::new(0., 0.))));
        let player2: ObjectCell = Rc::new(RefCell::new(Box::new(Player::new(0., 0.))));
        let players: Vec<ObjectCell> = vec![player1, player2];
        Space {size: Point{x:width, y:height}, grid, canvas, players, cor}
    }

    fn update_canvas(&mut self){
        let mut red: u8;
        let mut blue: u8;
        let mut green: u8;
        let mut alpha: u8;
        for (x, row) in self.grid.grid.iter().enumerate(){
            for (y, cell) in row.iter().enumerate(){
                (red, green, blue, alpha) = cell.color.get_values();
                let index: usize = ((y as i32 * self.size.x as i32 + x as i32) * 4) as usize;
                    self.canvas[index] = red;
                    self.canvas[index + 1] = green;
                    self.canvas[index + 2] = blue;
                    self.canvas[index + 3] = alpha;
            }
        }
    }

    fn get_collision_pairings(&self) -> Vec<(usize, usize)>{
        return vec![(0,1)];
    }

    pub fn push_canvas(&self) -> Vec<u8>{
        let mut image = vec![0;self.size.x as usize * self.size.y as usize *4];
        let mut red: u8;
        let mut blue: u8;
        let mut green: u8;
        let mut alpha: u8;
        for (x, row) in self.grid.grid.iter().enumerate(){
            for (y, cell) in row.iter().enumerate(){
                (red, green, blue, alpha) = cell.color.get_values();
                let index: usize = ((y as i32 * self.size.x as i32 + x as i32) * 4) as usize;
                    image[index] = red;
                    image[index + 1] = green;
                    image[index + 2] = blue;
                    image[index + 3] = alpha;
            }
        }
        return image
    }

    pub fn get_canvas(&self) -> *const u8{
        return self.canvas.as_ptr()
    }

    pub fn turn(&mut self){
        let collision_pairings = self.get_collision_pairings();
        for (x,y) in collision_pairings{
            let obj_1: Rc<RefCell<Box<dyn Object>>> = self.players[x].clone();
            let obj_2 = self.players[y].clone();
            let magnitude1_initial = obj_1.borrow().get_velocity().vector.magnitude;
            let magnitude2_initial = obj_2.borrow().get_velocity().vector.magnitude;
            let collision_time = obj_1.borrow_mut().collide(&mut **obj_2.borrow_mut(), self.cor);
            let t = collision_time.unwrap_or(27.);
            if t > 1. || t < 0.{
                obj_1.borrow_mut().get_velocity_mut().translate(&self.size);
                obj_2.borrow_mut().get_velocity_mut().translate(&self.size);
            }
            else{
            let mut intermediate_point_1 = obj_1.borrow_mut().translate_pos(t);
            let mut intermediate_point_2 = obj_2.borrow_mut().translate_pos(t);
            //finds the distance traveled before collision
            let pre_collision_vector1 = obj_1.borrow_mut().get_pos().between(&intermediate_point_1);
            let pre_collision_vector2 = obj_2.borrow_mut().get_pos().between(&intermediate_point_2);
            //finds the distance that still needs to be traveled
            let magnitude1 = magnitude1_initial - pre_collision_vector1.magnitude;
            let magnitude2 = magnitude2_initial - pre_collision_vector2.magnitude;
            //travels the remaining distance
            pre_collision_vector1.translate_magnitude(&mut intermediate_point_1, magnitude1);
            pre_collision_vector2.translate_magnitude(&mut intermediate_point_2, magnitude2);
            }
            self.grid.insert_player(&mut **obj_1.borrow_mut());
            self.grid.insert_player(&mut **obj_2.borrow_mut());
        }

    }

    pub fn tick(&mut self){
        self.turn();
        self.update_canvas();
    }

    pub fn accelerate(&mut self, id: i32, x: f64, y: f64){
        if (id as usize) < self.players.len(){
            self.players[id as usize].borrow_mut().accelerate(x, y);
        }
    }
}