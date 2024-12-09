use wasm_bindgen::prelude::*;
use js_sys::Math;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Obstacle,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct GameMap {
    width: usize,
    height: usize,
    tile_size: f64,
    tiles: Vec<TileType>,
}

#[wasm_bindgen]
impl GameMap {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, tile_size: f64) -> GameMap {
        let mut map = GameMap {
            width,
            height,
            tile_size,
            tiles: vec![TileType::Empty; width * height],
        };
        map.generate_map();
        map
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tile_size(&self) -> f64 {
        self.tile_size
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileType {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x]
        } else {
            TileType::Wall
        }
    }

    fn generate_map(&mut self) {
        // 生成外墙
        for x in 0..self.width {
            self.tiles[x] = TileType::Wall; // 上墙
            self.tiles[(self.height - 1) * self.width + x] = TileType::Wall; // 下墙
        }
        for y in 0..self.height {
            self.tiles[y * self.width] = TileType::Wall; // 左墙
            self.tiles[y * self.width + self.width - 1] = TileType::Wall; // 右墙
        }

        // 计算保护区域（左上角2x2的区域）
        let protected_area = [
            (1, 1), (1, 2),
            (2, 1), (2, 2)
        ];

        // 随机生成障碍物，但避开左上角
        let obstacle_count = (self.width * self.height) as f64 * 0.1; // 10% 的区域是障碍物
        let mut placed = 0;
        let max_attempts = (obstacle_count as i32) * 100;
        let mut attempts = 0;

        while placed < obstacle_count as i32 && attempts < max_attempts {
            let x = (Math::random() * (self.width - 2) as f64) as usize + 1;
            let y = (Math::random() * (self.height - 2) as f64) as usize + 1;

            // 检查是否在保护区域内
            let is_protected = protected_area.iter()
                .any(|&(px, py)| px == x && py == y);

            if !is_protected && self.tiles[y * self.width + x] == TileType::Empty {
                self.tiles[y * self.width + x] = TileType::Obstacle;
                placed += 1;
            }
            attempts += 1;
        }
    }

    pub fn is_walkable(&self, x: f64, y: f64) -> bool {
        let tile_x = (x / self.tile_size) as usize;
        let tile_y = (y / self.tile_size) as usize;
        self.get_tile(tile_x, tile_y) == TileType::Empty
    }
}