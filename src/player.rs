// src/player.rs
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Player {
    position: Position,
    speed: f64,
    size: f64,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Player {
        console::log_1(&"Creating new player".into());
        Player {
            position: Position { x, y },
            speed: 5.0,
            size: 20.0,
        }
    }

    pub fn x(&self) -> f64 {
        self.position.x
    }

    pub fn y(&self) -> f64 {
        self.position.y
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn move_up(&mut self) {
        self.position.y -= self.speed;
    }

    pub fn move_down(&mut self, max_height: f64) {
        let new_y = self.position.y + self.speed;
        if new_y + self.size <= max_height {
            self.position.y = new_y;
        }
    }

    pub fn move_left(&mut self) {
        self.position.x -= self.speed;
    }

    pub fn move_right(&mut self, max_width: f64) {
        let new_x = self.position.x + self.speed;
        if new_x + self.size <= max_width {
            self.position.x = new_x;
        }
    }

    pub fn update(&mut self) {
        // 将来可以在这里添加更多的更新逻辑
    }
}