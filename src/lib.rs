pub mod vector;
pub mod space;
pub mod player;
pub mod velocity;
pub mod material;
pub mod object;
pub mod physics;
pub mod universe;

#[cfg(test)]
mod tests {
    use crate::object::Object;
    use crate::player::Player;
    use crate::vector::{Point, Vector};
    use crate::physics;
    use crate::universe::{Universe, parse_input, get_input};

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

    #[test]
    fn test_time_collision_distance(){
        let body1 = Player::tester(Point{x: 0., y: 0.}, 1., 1., 2.);
        let body2 = Player::tester(Point{x: 4., y: 4.}, -1., -1., 2.);
        let (x,y,x0,y0, r1, r2) = physics::calculate_relative_values(&body1, &body2);
        println!("{}, {}, {}, {}, {}, {}", x,y,x0,y0,r1,r2);
        let time = physics::time_to_collision(x, y, x0, y0, r1, r2) ;
        println!("{}, {}", time.unwrap().0, time.unwrap().1);
        assert!(time == Some((3.414213562373095, 0.5857864376269049)));
    }

    #[test]
    fn test_time_border(){
        let body1 = Player::tester(Point{x: 0., y: 0.}, 1., 0., 1.);
        let body2 = Player::tester(Point{x: 2., y: 0.}, -1., 0., 1.);
        let (x,y,x0,y0, r1, r2) = physics::calculate_relative_values(&body1, &body2);
        println!("{}, {}, {}, {}, {}, {}", x,y,x0,y0,r1,r2);
        let time = physics::time_to_collision(x, y, x0, y0, r1, r2) ;
        println!("{}, {}", time.unwrap().0, time.unwrap().1);
        assert!(time == Some((2.,0.)));
    }

    #[test]
    fn testing_things(){
        let test = None;
        assert!(test.unwrap_or(2.) == 2.);
    }

    #[test]
    fn test_accelerate_force(){
        //mass = 32
        let mut player = Player::new(0., 0.);
        player.accelerate_force(Vector::new(1.,0.));
        println!("{}", player.get_mass());
        println!("{}", player.get_velocity().vector.x);
        assert!(player.get_velocity().vector.x == 1./32.);
        assert!(player.get_velocity().vector.y == 0.);
    }

    #[test]
    fn universe_create(){
        let _ = Universe::new();
    }


    #[test]
    fn test_parse_input(){
        let parsed_input = parse_input(("w 1".to_string(), 0));
        assert!(parsed_input == Some((0, 0, 1.)));
    }

    #[test]
    fn test_get_input(){
        println!("Enter Input 'w 1': ");
        let input = get_input().unwrap();
        assert!(input == (0,0,1.));
    }

    #[test]
    fn universe_run(){
        let mut uni = Universe::new();
        for _ in 0..4{
            uni.run();
        }
    }

    #[test]
    fn test_show(){
        let mut universe = Universe::new();
        universe.run();
    }
    // #[test]
    // fn test_run(){
    //     let mut space: Box<Space> = Box::new(set_up());
    //     space.tick();
    //     space.tick();
    //     space.tick();
    //     space.tick();
    //     space.tick();
    // }

    // fn set_up() -> Space{
        // let space = Space::new();
        // println!("{}, {}", &space.player1.get_velocity().origin.x,&space.player1.get_velocity().origin.y);
        // println!("{}, {}", &space.player2.get_velocity().origin.x,&space.player2.get_velocity().origin.y);
        // return space
    // }

}