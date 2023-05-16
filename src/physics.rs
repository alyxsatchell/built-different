use crate::object::Object;

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