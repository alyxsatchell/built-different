use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;

use crate::vector::{Point, Vector};
use crate::player::{Player, Direction};

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
    rx_main: Receiver<Direction>,
    tx_main: Sender<String>
}

impl Space{
    pub fn new(width: i32, height: i32) -> Space{
        let canvas: Vec<u8> = Vec::new();
        let mut grid = CellGrid::new(width, height);
        let player = Player::new();
        *grid.get_mut(&player.vector.origin).unwrap() = Cell{color: player.color, occupied: true};
        let players = vec![player];
        let (tx_main, rx_worker) = channel();
        let (tx_worker, rx_main) = channel();
        let input_worker_handle = thread::spawn(move || {
            loop{
                let input = get_input();
                match rx_worker.try_recv(){
                    Ok(v) => {tx_worker.send(input);},
                    Err(e) => ()
                }
            }
        });
        Space {size: Point{x:width, y:height}, grid, players, canvas, input_worker_handle, rx_main, tx_main}
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

    fn turn(&mut self){
        for player in &mut self.players{
            player.vector.translate(&self.size);
            self.grid.insert_player(player);
        }
    }

    fn get_input(&mut self){
        self.tx_main.send(String::from("input"));
        match self.rx_main.recv_timeout(Duration::from_millis(10)){
            Ok(dir) => self.players[0].accelerate(dir),
            Err(e) => {println!("{}", e);}
        }
    }

    pub fn tick(&mut self){
        self.get_input();
        self.turn();
        self.push_canvas();
    }
}

//placeholder
fn get_input() -> Direction{
    return Direction::Right
}