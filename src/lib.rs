pub mod vector;
pub mod space;
pub mod player;
pub mod velocity;
pub mod material;
pub mod object;
pub mod physics;

// use crate::space::Space;

// extern crate console_error_panic_hook;
// use std::panic;

// #[wasm_bindgen]
// pub fn my_init_function() {
//     panic::set_hook(Box::new(console_error_panic_hook::hook));
// }

// #[wasm_bindgen]
// pub fn init_panic_hook() {
//     console_error_panic_hook::set_once();
// }

#[cfg(test)]
mod tests {
    use crate::object::Body;
    use crate::player::Player;
    use crate::space::Space;
    use crate::vector::Point;
    use crate::{physics, velocity};
    use crate::space::Color;

    #[test]
    fn it_works() {
        let _ = Space::new();
    }

    #[test]
    fn test_quadratic() {
        println!("{},{}", physics::solve_quadratic(4., -12., 5.).unwrap().0, physics::solve_quadratic(4., -12., 5.).unwrap().1);
        assert!(physics::solve_quadratic(4., -12., 5.) == Some((2.5, 0.5)))
    }
    #[test]
    fn test_time_to_collision_head_on() {
        let body1 = Player::tester(Point{x: -3., y: 0.}, 1., 0., 1.);
        let body2 = Player::tester(Point{x: 0., y: 0.}, -1., 0., 1.);
        let (x,y,x0,y0, r1, r2) = physics::calculate_relative_values(&body1, &body2);
        println!("{}, {}, {}, {}, {}, {}", x,y,x0,y0,r1,r2);
        let time = physics::time_to_collision(x, y, x0, y0, r1, r2) ;
        println!("{}, {}", time.unwrap().0, time.unwrap().1);
        assert!(time == Some((2.5, 0.5)));
    }

    #[test]
    fn test_time_glance_head_on(){
        let body1 = Player::tester(Point{x: -3., y: 1.}, 1., 0., 1.);
        let body2 = Player::tester(Point{x: 0., y: 0.}, -1., 0., 1.);
        let (x,y,x0,y0, r1, r2) = physics::calculate_relative_values(&body1, &body2);
        println!("{}, {}, {}, {}, {}, {}", x,y,x0,y0,r1,r2);
        let time = physics::time_to_collision(x, y, x0, y0, r1, r2) ;
        println!("{}, {}", time.unwrap().0, time.unwrap().1);
        assert!(time == Some((2.3660254037844384, 0.6339745962155614)));
    }

    #[test]
    fn test_time_glance_fail(){
        let body1 = Player::tester(Point{x: -3., y: -5.}, 2., 3., 1.);
        let body2 = Player::tester(Point{x: 1., y: -4.}, 1., 2., 1.);
        let (x,y,x0,y0, r1, r2) = physics::calculate_relative_values(&body1, &body2);
        println!("{}, {}, {}, {}, {}, {}", x,y,x0,y0,r1,r2);
        let time = physics::time_to_collision(x, y, x0, y0, r1, r2) ;
        println!("{}, {}", time.unwrap_or((-27.,-27.)).0, time.unwrap_or((-27.,-27.)).1);
        assert!(time == None);
    }

    #[test]
    fn test_time_glance(){
        let body1 = Player::tester(Point{x: -3., y: 1.}, 1., 1., 1.);
        let body2 = Player::tester(Point{x: 1., y: 1.}, -1., 2., 1.);
        let (x,y,x0,y0, r1, r2) = physics::calculate_relative_values(&body1, &body2);
        println!("{}, {}, {}, {}, {}, {}", x,y,x0,y0,r1,r2);
        let time = physics::time_to_collision(x, y, x0, y0, r1, r2) ;
        println!("{}, {}", time.unwrap().0, time.unwrap().1);
        assert!(time == Some((2., 1.2)));
    }

}