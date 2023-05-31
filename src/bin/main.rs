use built_different_lib::{self, velocity::{Velocity}, vector::Point, player::Player};
use image::{self, Frame, ImageBuffer, Rgba, open};

use crate::built_different_lib::space::Space;
use crate::built_different_lib::universe::Input;

// const MASS1: f64 = 0.2;
// const MASS2: f64 = 0.6;

// fn one_motion_elastic() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:15.,y: 30.}, 4.43, 0.), MASS1);
//     let player2 = Player::create(Velocity::new(Point { x: 47., y: 30. }, 0., 0.), MASS2);
//     Box::new(Space::new(player1, player2, 1.))
// }

// fn two_motion_elastic() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:80.,y: 80.}, 0.5, 0.5), MASS1);
//     let player2 = Player::create(Velocity::new(Point { x: 85., y: 85. }, -0., -0.), MASS2);
//     Box::new(Space::new(player1, player2, 1.))
// }

// fn two_motion_head_on_inelastic() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:45.,y: 30.}, 4.43, 0.), MASS1);
//     let player2 = Player::create(Velocity::new(Point { x: 97., y: 30. }, -4.43, 0.), MASS2);
//     Box::new(Space::new(player1, player2, 0.5))
// }

// fn two_motion_chase_partial_elastic() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:85.,y: 30.}, -2., 0.), MASS1);
//     let player2 = Player::create(Velocity::new(Point { x: 97., y: 30. }, -3.336, 0.), MASS2);
//     Box::new(Space::new(player1, player2, 0.5))
// }

// fn head_on_partial_elastic() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:25.,y: 30.}, 1., 0.), 1.);
//     let player2 = Player::create(Velocity::new(Point { x: 37., y: 30. }, -2., 0.), 1.);
//     Box::new(Space::new(player1, player2, 0.5))
// }

// fn default_set_up() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:25.,y: 30.}, 1., 0.), 1.);
//     let player2 = Player::create(Velocity::new(Point { x: 37., y: 30. }, 0., 0.), 1.);
//     Box::new(Space::new(player1, player2, 1.))
// }

// fn glance() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:15.,y: 15.}, 0.5, 0.5), MASS1);
//     let player2 = Player::create(Velocity::new(Point { x: 45., y: 55. }, -1., -1.5), MASS2);
//     Box::new(Space::new(player1, player2, 1.))
// }

// fn default_set_up_partial_elastic()-> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:30.,y: 30.}, 1., 0.), 1.);
//     let player2 = Player::create(Velocity::new(Point { x: 37., y: 30. }, 0., 0.), 1.);
//     Box::new(Space::new(player1, player2, 0.5))
// }

// fn default_set_up_totally_inelastic() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:0.,y: 0.}, 1., 0.), 1.);
//     let player2 = Player::create(Velocity::new(Point { x: 7., y: 0. }, 0., 0.), 1.);
//     Box::new(Space::new(player1, player2, 0.))
// }

// fn inverse_default_set_up() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:50.,y: 50.}, 0., 0.), 1.);
//     let player2 = Player::create(Velocity::new(Point { x: 57., y: 50. }, -1., 0.), 1.);
//     Box::new(Space::new(player1, player2, 1.))
// }

// fn glance2() -> Box<Space>{
//     let player1 = Player::create(Velocity::new(Point{x:50.,y: 50.}, 1.5, 5.5), 1.);
//     let player2 = Player::create(Velocity::new(Point { x: 25., y: 25. }, -1., 3.5), 1.);
//     Box::new(Space::new(player1, player2, 1.))
// }

fn main(){
    // let mut space = Box<Space::new(Player)
    // let mut space = set_up();
    // let mut space = default_set_up();
    // let mut space = inverse_default_set_up();
    // let mut space = glance();
    // let mut space = default_set_up_partial_elastic();
    // let mut space = one_motion_elastic();
    // let mut space = two_motion_head_on_inelastic();
    // let mut space = two_motion_chase_partial_elastic();
    // let mut space = two_motion_elastic();
    // let mut num_of_ticks: String = String::new();
    // Input::get_input(&mut num_of_ticks, "Input Number Of Simulated Ticks");
    // for i in 0..num_of_ticks.split("\n").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(){
    //     space.turn();
    //     let img1 = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(100, 100, space.push_canvas()).unwrap();
    //     let filename = format!("project_frames/testing/{i}.png");
    //     img1.save(&filename).expect("failed to save image");
    // }
}