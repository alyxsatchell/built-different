use rand::prelude::*;
use std::ops::{Index, IndexMut, Add, Sub, AddAssign, Mul, MulAssign};
use std::cmp::{PartialEq,Eq};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Point{
    pub x: f64,
    pub y: f64
}

impl Point{
    pub fn new_rand() -> Point{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..=100) as f64;
        let y = rng.gen_range(0..=100) as f64;
        return Point{x,y}
    }

    pub fn between(&self, other: &Point) -> Vector{
        let x = self.x - other.x;
        let y = self.y - other.y;
        Vector { x, y, magnitude: Vector::magnitude(x, y)}
    }

}

impl From<constellation::canvas::Point> for Point{
    fn from(value: constellation::canvas::Point) -> Self {
        return Point{x: value.x as f64, y: value.y as f64}
    }
}

impl From<Point> for constellation::canvas::Point{
    fn from(value: Point) -> Self {
        return constellation::canvas::Point{x: value.x as i32, y: value.y as i32}
    }
}

impl PartialEq for Point{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point{

}

impl Index<usize> for Point {
    type Output = f64;
    fn index(&self, s: usize) -> &f64 {
        match s {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, s: usize) -> &mut f64{
        match s {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Point {x, y}
    }
}

impl Sub<Point> for Point{
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        let x = self.x - other.x;
        let y = self.y - other.y;
        return Point{x,y}
    } 
}

impl AddAssign<Point> for Point{
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign<Vector> for &mut Point{
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub struct Vector{
    pub x: f64,
    pub y: f64,
    pub magnitude: f64
}

impl Vector{
    pub fn new(x:f64, y: f64) -> Vector{
        Vector { x, y, magnitude: Vector::magnitude(x, y) }
    }

    pub fn zero() -> Vector{
        Vector { x: 0., y: 0., magnitude: 0. }
    }

    fn magnitude(x:f64, y:f64) -> f64{
        ((f64::powf(x,2.0) + f64::powf(y,2.0)) as f64).sqrt()
    }

    pub fn resize(& self, new_mag: f64) -> Vector{
        let k = new_mag / self.magnitude;
        let x = self.x * k;
        let y = self.y * k;
        Vector {x, y, magnitude: k }
    }

    pub fn normal(&self) -> Vector{
        Vector::new(-self.x, self.y)
    }

    pub fn angle_between(&self, other: &Vector) -> f64{
        ((self * other) / (self.magnitude * other.magnitude)).acos()
    }

    pub fn split(&self, other: &Vector) -> (f64, f64){
        let theta = self.angle_between(other);
        println!("{} theata", theta);
        if theta.is_nan(){ //this is caused by the original vector being a zero vector
            return (0.,0.)
        }
        let x = self.magnitude * theta.cos();
        let y = self.magnitude * theta.sin();
        return (x,y)
    }

    pub fn rotate(&mut self, other: &Vector){
        let (x,y) = self.split(other);
        self.x = x;
        self.y = y;
        self.magnitude = Vector::magnitude(x, y);
    }

    pub fn translate_magnitude(&self, mut point: &mut Point, magnitude: f64){
        let temp_vector = self.resize(magnitude);
        point += temp_vector;
    }
}

impl Mul<Vector> for Vector {
    type Output = f64;
    fn mul(self, other: Vector) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl Mul<&Vector> for &Vector {
    type Output = f64;
    fn mul(self, other: &Vector) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let magnitude = Vector::magnitude(x, y);
        return Vector{x, y, magnitude}
    }
}

impl Add<&Vector> for &Vector{
    type Output = Vector;
    fn add(self, other: &Vector) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let magnitude = Vector::magnitude(x, y);
        return Vector{x, y, magnitude}
    }
}

impl Sub<Vector> for Vector{
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let magnitude = ((f64::powf(x,2.0) + f64::powf(y,2.0)) as f64).sqrt();
        return Vector{x,y, magnitude}
    } 
}

impl AddAssign<Vector> for Vector{
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.magnitude = ((f64::powf(self.x,2.0) + f64::powf(self.y,2.0)) as f64).sqrt();
    }
}

impl MulAssign<f64> for Vector{
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }
}

impl fmt::Display for Vector{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<Vector> for constellation::canvas::Point{
    fn from(value: Vector) -> Self {
        constellation::canvas::Point{x: value.x as i32, y: value.y as i32}
    }
}