use wasm_bindgen::prelude::*;
pub mod vector;
pub mod space;
pub mod player;
pub mod velocity;
pub mod material;
pub mod object;

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
    use crate::space::Space;

    #[test]
    fn it_works() {
        let space = Space::new();
    }
}