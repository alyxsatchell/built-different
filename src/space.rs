use wasm_bindgen::prelude::*;

use crate::vector::{Point};
use crate::player::{Player};
use crate::object::Object;

const SPACE_CELL: Cell = Cell{color: Color { r: 0, b: 0, g: 0, a: 255 }, occupied: false};


#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
#[wasm_bindgen]
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

    pub fn insert_player(&mut self, player: &mut Player){
        for (i,_) in &player.body.grid{
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
        player.body.grid = occupied_space;
    }


}

#[wasm_bindgen]
pub struct Space{
    size: Point,
    grid: CellGrid,
    players: Vec<Player>,
    canvas: Vec<u8>
}

#[wasm_bindgen]
impl Space{
    pub fn new() -> Space{
        let (width, height): (f64, f64) = (100.0, 100.0);
        let canvas: Vec<u8> = vec![0;width as usize * height as usize *4];
        let mut grid = CellGrid::new(width, height);
        let player = Player::new();
        let player2 = Player::new();
        *grid.get_mut(&player.velocity.origin).unwrap() = Cell{color: player.color, occupied: true};
        let players = vec![player, player2];

        Space {size: Point{x:width, y:height}, grid, players, canvas}
    }

    fn push_canvas(&mut self){
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

    pub fn get_canvas(&self) -> *const u8{
        return self.canvas.as_ptr()
    }

    fn turn(&mut self){
        for player in &mut self.players{
            player.velocity.translate(&self.size);
            self.grid.insert_player(player);
        }
    }

    pub fn tick(&mut self){
        self.turn();
        self.push_canvas();
    }

    pub fn accelerate(&mut self, id: i32, x: f64, y: f64){
        if (id as usize) < self.players.len(){
            self.players[id as usize].accelerate(x, y);
        }
    }
}