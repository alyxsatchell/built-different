use std::thread::JoinHandle;

use crate::vector::{Point, Vector};
use crate::player::Player;

const space_cell: Cell = Cell{color: Color { r: 0, b: 0, g: 0, a: 0 }, occupied: false};

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
    pub fn new(width: i32, height: i32) -> CellGrid{
        let mut grid = Vec::new();
        for h in 0..height{
            grid.push(Vec::new());
            for w in 0..width{
                grid[h as usize].push(space_cell.clone());
            }
        }
        CellGrid { grid}
    }

    pub fn get(&self, p: Point) -> Option<&Cell>{
        if p.y < 0 || p.y >= self.grid.len() as i32{
            return None
        }
        if p.x < 0 || p.x >= self.grid[p.y as usize].len() as i32{
            return None
        }
        Some(&self.grid[p.y as usize][p.x as usize])
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut Cell>{
        if p.y < 0 || p.y >= self.grid.len() as i32{
            return None
        }
        if p.x < 0 || p.x >= self.grid[p.y as usize].len() as i32{
            return None
        }
        Some(&mut self.grid[p.y as usize][p.x as usize])
    }

    pub fn insert_player(&mut self, player: &mut Player){
        for i in &player.occupied_space{
            let cell = self.get_mut(i);
            if cell.is_some(){
                *cell.unwrap() = space_cell.clone();
            }
        }
        let occupied_space = player.make_circle();
        for i in &occupied_space{
            let cell = self.get_mut(i);
            if cell.is_some(){
                *cell.unwrap() = space_cell.clone();
            }
        }
        player.occupied_space = occupied_space;
    }


}

pub struct Space{
    size: Point,
    grid: CellGrid,
    players: Vec<Player>,
    canvas: Vec<u8>,
    input_worker_handle: JoinHandle<String>,
    input
}

impl Space{
    pub fn new(width: i32, height: i32) -> Space{
        let canvas: Vec<u8> = Vec::new();
        let mut grid = CellGrid::new(width, height);
        let player = Player::new();
        *grid.get_mut(&player.vector.origin).unwrap() = Cell{color: player.color, occupied: true};
        let players = vec![player];
        Space {size: Point{x:width, y:height}, grid, players, canvas}
    }

    pub fn push_canvas(&mut self){
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
}