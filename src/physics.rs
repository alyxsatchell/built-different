use crate::{object::Object, vector::Vector, velocity::Velocity};

const i_vector: Vector = Vector{x:1., y:0., magnitude: 1.};
const j_vector: Vector = Vector{x:0., y:1., magnitude: 1.};

pub fn solve_quadratic(a: f64,b: f64,c: f64) -> Option<(f64, f64)>{
    let mut discriminate = b.powf(2.) - 4. * a * c;
    if discriminate < 0.{
        return None
    }
    discriminate = discriminate.sqrt();
    let positive = (- b + discriminate) / (2. * a);
    let negative = (- b - discriminate) / (2. * a);
    return Some((positive, negative))
}

pub fn velocity_after_collision(k: f64, m1: f64, m2: f64, v1: f64, v2: f64) -> f64{
    let step1 = k * (v2 - v1);
    let step2 = v1 * m1 + v2 * m2;
    let step3 = m1 + m2;
    return (step1 + step2) / step3
}

pub fn calculate_relative_values(body1: &dyn Object, body2: &dyn Object) -> (f64, f64, f64, f64, f64, f64){
    let vec1 = &body1.get_velocity().vector;
    let vec2 = &body2.get_velocity().vector;
    let pos1 = body1.get_pos();
    let pos2 = body2.get_pos();
    let x = vec1.x - vec2.x;
    let y = vec1.y - vec2.y;
    let x0 = pos1.x - pos2.x;
    let y0 = pos1.y - pos2.y;
    return (x,y,x0,y0, body1.get_size(), body2.get_size())
}

pub fn time_to_collision(x: f64,y: f64,x0: f64,y0: f64, r1: f64, r2: f64) -> Option<(f64, f64)>{
    // 0 = t^2(x^2 + y^2) + t(2 * (x * x0 + y * y0)) + x0^2 + y0^2
    let a = x.powf(2.) + y.powf(2.);
    let b = 2. * (x * x0 + y * y0);
    let c = x0.powf(2.) + y0.powf(2.) - (r1 + r2).powf(2.);
    solve_quadratic(a, b, c)
}

pub fn get_collision_vectors(body1: &dyn Object, body2: &dyn Object, t: f64) -> (Vector, Vector){
    let vector1 = body1.translate_pos(t).between(&body2.translate_pos(t));
    let vector2 = body2.translate_pos(t).between(&body1.translate_pos(t));
    return (vector1, vector2)
}

pub fn get_collision_normal(body1: &dyn Object, body2: &dyn Object) -> Option<Vector>{
    let (x,y,x0,y0,r1,r2) = calculate_relative_values(body1, body2);
    let time = time_to_collision(x, y, x0, y0, r1, r2);
    if time.is_none(){
        return None
    }
    return Some(get_collision_vectors(body1, body2, time.unwrap().1).0);
}

pub fn collision_velocity(cor: f64, v1: f64, v2: f64, m1: f64, m2: f64) -> f64{
    let step1 = cor * m2 * (v2 - v1);
    let step2 = m1 * v1 + m2 * v2;
    let step3 = m1 + m2;
    return (step1 + step2) / step3
}

pub fn post_collision_velocity(body1: &dyn Object, body2: &dyn Object) -> Option<(Vector, Vector)>{
    //normal vector of the collision
    let n = get_collision_normal(body1, body2);
    //checks if the collision even occurred
    if n.is_none(){
        return None
    }
    let edge_material = body1.get_edge_material(&n.as_ref().unwrap());
    let cor = edge_material.coefficient_of_restitution;
    //gets the components of the first body
    let (xi1, yi1) = &body1.get_velocity().vector.split(&n.as_ref().unwrap());
    let m1 = body1.get_mass();
    //gets the components of the second body
    let (xi2, yi2) = &body2.get_velocity().vector.split(&n.unwrap());
    let m2 = body2.get_mass();
    //finds the components of the final velocities of the 2 bodies
    let xf1 = collision_velocity(cor, *xi1, *xi2, *m1, *m2);
    let yf1 = collision_velocity(cor, *yi1, *yi2, *m1, *m2);
    let xf2 = collision_velocity(cor, *xi2, *xi1, *m1, *m2);
    let yf2 = collision_velocity(cor, *yi2, *yi1, *m1, *m2);
    //makes the new vectors the bodies will use and rotates them back to the normal xy plane
    let mut vector1 = Vector::new(xf1,yf1);
    vector1.rotate(&i_vector);
    let mut vector2 = Vector::new(xf2, yf2);
    vector2.rotate(&i_vector);
    return Some((vector1,vector2));
}

pub fn calculate_impulse(m: f64, v1: &Vector, v2: &Vector) -> (f64, f64){
    let x = m * (v2.x - v1.x);
    let y = m * (v2.y - v1.y);
    return (x,y)
}