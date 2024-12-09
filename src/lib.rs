// src/lib.rs
use wasm_bindgen::prelude::*;
mod game;
mod renderer;
mod player;
mod map;
mod treasure;
pub use game::Game;
pub use renderer::Renderer;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}