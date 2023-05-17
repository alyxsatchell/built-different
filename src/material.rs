use crate::space::Color;

#[derive(Clone)]
pub struct Material{
    pub density: f64,
    pub color: Color,
    pub coefficient_of_restitution: f64,
    //more to be added as more physics are added
}

impl Material{
    pub fn null_material() -> Material{
        return Material { density: 0., color: Color::black() , coefficient_of_restitution: 0.}
    }
}