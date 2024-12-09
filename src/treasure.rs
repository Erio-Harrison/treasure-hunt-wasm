// src/treasure.rs
use wasm_bindgen::prelude::*;
use js_sys::Math;

use crate::map::GameMap;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Treasure {
    x: f64,
    y: f64,
    size: f64,
    collected: bool,
}

#[wasm_bindgen]
impl Treasure {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Treasure {
        Treasure {
            x,
            y,
            size: 20.0, // 宝藏大小
            collected: false,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn is_collected(&self) -> bool {
        self.collected
    }

    pub fn collect(&mut self) {
        self.collected = true;
    }
}

#[wasm_bindgen]
pub struct TreasureManager {
    treasures: Vec<Treasure>,
    score: u32,
}

#[wasm_bindgen]
impl TreasureManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TreasureManager {
        TreasureManager {
            treasures: Vec::new(),
            score: 0,
        }
    }

    pub fn generate_treasures(
        &mut self, 
        count: u32, 
        map_width: f64, 
        map_height: f64, 
        tile_size: f64,
        game_map: &GameMap,  // 添加地图参数用于碰撞检测
    ) {
        self.treasures.clear();
        let mut placed_count = 0;
        let max_attempts = count * 100; // 防止无限循环
        let mut attempts = 0;

        while placed_count < count && attempts < max_attempts {
            let x = Math::random() * (map_width - 2.0 * tile_size) + tile_size;
            let y = Math::random() * (map_height - 2.0 * tile_size) + tile_size;

            // 检查该位置是否可行走
            if game_map.is_walkable(x, y) {
                self.treasures.push(Treasure::new(x, y));
                placed_count += 1;
            }

            attempts += 1;
        }

        // 如果无法放置所有宝藏，输出警告
        if placed_count < count {
            web_sys::console::log_1(&"Warning: Could not place all treasures in valid positions".into());
        }
    }

    pub fn check_collection(&mut self, player_x: f64, player_y: f64, player_size: f64) -> bool {
        let mut collected = false;
        for treasure in &mut self.treasures {
            if !treasure.is_collected() {
                let dx = treasure.x - (player_x + player_size / 2.0);
                let dy = treasure.y - (player_y + player_size / 2.0);
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance < (player_size + treasure.size) / 2.0 {
                    treasure.collect();
                    self.score += 10;
                    collected = true;
                }
            }
        }
        collected
    }

    pub fn all_treasures_collected(&self) -> bool {
        self.treasures.iter().all(|t| t.is_collected())
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_treasure_count(&self) -> usize {
        self.treasures.len()
    }

    pub fn get_treasure_x(&self, index: usize) -> Option<f64> {
        self.treasures.get(index).map(|t| t.x)
    }

    pub fn get_treasure_y(&self, index: usize) -> Option<f64> {
        self.treasures.get(index).map(|t| t.y)
    }

    pub fn is_treasure_collected(&self, index: usize) -> bool {
        self.treasures.get(index).map(|t| t.is_collected()).unwrap_or(true)
    }

    pub fn reset_score(&mut self) {
        self.score = 0;
    }
}