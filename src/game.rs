// src/game.rs
use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::player::Player;
use crate::map::{GameMap, TileType};
use crate::treasure::TreasureManager;
use crate::audio::AudioSystem;

#[wasm_bindgen]
pub struct Game {
    width: u32,
    height: u32,
    is_running: bool,
    last_frame_time: f64,
    player: Player,
    map: GameMap,
    keys_pressed: Vec<String>,
    treasure_manager: TreasureManager,
    state: GameState,
    game_time: f64, 
    time_limit: f64,   
    best_time: Option<f64>,  
    first_timestamp: Option<f64>,
    audio: Option<AudioSystem>,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Playing,
    Won,
    TimeUp,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub async fn new(width: u32, height: u32) -> Result<Game, JsValue> {
        console::log_1(&"Creating new game instance".into());
        
        // 初始化音频系统
        let audio = match AudioSystem::new().await {
            Ok(audio_system) => {
                // 预加载所有音效
                audio_system.load_sound("collect", "./sounds/collect.mp3").await?;
                audio_system.load_sound("win", "./sounds/win.mp3").await?;
                audio_system.load_sound("timeup", "./sounds/timeup.mp3").await?;
                audio_system.load_sound("background", "./sounds/background.mp3").await?;
                Some(audio_system)
            },
            Err(_) => None,
        };

        let tile_size = 40.0;
        let map_width = (width as f64 / tile_size) as usize;
        let map_height = (height as f64 / tile_size) as usize;
        
        let map = GameMap::new(map_width, map_height, tile_size);
        let player = Player::new(tile_size * 1.5, tile_size * 1.5);
        let mut treasure_manager = TreasureManager::new();
        treasure_manager.generate_treasures(5, width as f64, height as f64, tile_size, &map);

        Ok(Game {
            width,
            height,
            is_running: false,
            last_frame_time: 0.0,
            player,
            map,
            treasure_manager,
            keys_pressed: Vec::new(),
            state: GameState::Playing,
            game_time: 0.0,
            time_limit: 60.0,
            best_time: None,
            first_timestamp: None,
            audio,
        })
    }

    // 添加 #[wasm_bindgen] 属性给这些公共方法
    #[wasm_bindgen]
    pub fn start(&mut self) {
        console::log_1(&"Game started!".into());
        self.is_running = true;
        self.last_frame_time = 0.0;
        self.game_time = 0.0;
        self.time_limit = 60.0;
        if let Some(audio) = &self.audio {
            audio.play_music("bgm");
        }
        console::log_1(&format!("Start: time_limit={}, game_time={}", 
            self.time_limit, self.game_time).into());
    }

    #[wasm_bindgen]
    pub fn stop(&mut self) {
        console::log_1(&"Game stopped!".into());
        self.is_running = false;
        
        if let Some(audio) = &self.audio {
            audio.stop_music();
        }
    }

    // 更新游戏状态检查
    #[wasm_bindgen]
    pub fn update(&mut self, timestamp: f64) {
        if !self.is_running {
            return;
        }

        // 处理无效的时间戳
        if timestamp.is_nan() {
            self.last_frame_time = 0.0;
            return;
        }

        if self.last_frame_time == 0.0 {
            self.last_frame_time = timestamp;
            return;
        }

        // 防止时间差出现 NaN
        let delta_time = if self.last_frame_time.is_nan() {
            0.0
        } else {
            (timestamp - self.last_frame_time) / 1000.0
        };

        self.last_frame_time = timestamp;
        
        // 防止游戏时间出现 NaN
        if !self.game_time.is_nan() {
            self.game_time += delta_time;
        } else {
            self.game_time = 0.0;
        }

        // 先检查时间限制
        if self.game_time >= self.time_limit {
            self.state = GameState::TimeUp;
            if let Some(audio) = &self.audio {
                audio.stop_music();
                audio.play_sound("timeup");
            }
            self.stop();
            return;
        }

        // 更新玩家位置和收集宝藏
        self.update_player();

        // 检查胜利条件
        if self.treasure_manager.all_treasures_collected() {
            self.state = GameState::Won;
            // 更新最佳时间
            if let Some(best_time) = self.best_time {
                if self.game_time < best_time {
                    self.best_time = Some(self.game_time);
                }
            } else {
                self.best_time = Some(self.game_time);
            }

            if let Some(audio) = &self.audio {
                audio.stop_music();
                audio.play_sound("win");
            }

            self.stop();
        }
    }

    // 重置游戏
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.game_time = 0.0;
        self.state = GameState::Playing;
        self.is_running = false;
        
        // 重置玩家到左上角固定位置
        let tile_size = 40.0;
        self.player = Player::new(tile_size * 1.5, tile_size * 1.5); 
        
        // 重置按键状态
        self.keys_pressed.clear();

        // 重置分数
        self.treasure_manager.reset_score();

        // 重置初始时间戳
        self.first_timestamp = None;
        
        // 重新生成宝藏
        self.treasure_manager.generate_treasures(
            5,
            self.width as f64,
            self.height as f64,
            40.0,
            &self.map
        );
    }

    // 获取游戏状态
    #[wasm_bindgen]
    pub fn get_state(&self) -> GameState {
        self.state
    }

    // 获取当前游戏时间
    #[wasm_bindgen]
    pub fn get_game_time(&self) -> f64 {
        self.game_time
    }

    // 获取剩余时间
    #[wasm_bindgen]
    pub fn get_remaining_time(&self) -> f64 {
        let remaining = self.time_limit - self.game_time;
        console::log_1(&format!("Calculating remaining time: {} - {} = {}", 
            self.time_limit, self.game_time, remaining).into());
        remaining.max(0.0)
    }

    // 获取最佳时间
    #[wasm_bindgen]
    pub fn get_best_time(&self) -> Option<f64> {
        self.best_time
    }

    // 添加时间戳设置方法
    #[wasm_bindgen]
    pub fn set_first_timestamp(&mut self, timestamp: f64) {
        if self.first_timestamp.is_none() {
            self.first_timestamp = Some(timestamp);
        }
    }    

    #[wasm_bindgen]
    pub fn handle_key_down(&mut self, key: String) {
        if !self.keys_pressed.contains(&key) {
            self.keys_pressed.push(key);
        }
    }

    #[wasm_bindgen]
    pub fn handle_key_up(&mut self, key: String) {
        self.keys_pressed.retain(|k| k != &key);
    }

    #[wasm_bindgen]
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    #[wasm_bindgen]
    pub fn player(&self) -> Player {
        self.player
    }

    // 内部方法不需要 #[wasm_bindgen]
    pub fn update_player(&mut self) {
        let mut new_x = self.player.x();
        let mut new_y = self.player.y();
        let speed = 5.0;

        for key in &self.keys_pressed {
            match key.as_str() {
                "ArrowUp" => new_y -= speed,
                "ArrowDown" => new_y += speed,
                "ArrowLeft" => new_x -= speed,
                "ArrowRight" => new_x += speed,
                _ => {}
            }
        }

        // 碰撞检测
        if self.map.is_walkable(new_x, new_y) 
            && self.map.is_walkable(new_x + self.player.size(), new_y)
            && self.map.is_walkable(new_x, new_y + self.player.size())
            && self.map.is_walkable(new_x + self.player.size(), new_y + self.player.size()) {
            self.player = Player::new(new_x, new_y);
        }

        if self.treasure_manager.check_collection(
            self.player.x(),
            self.player.y(),
            self.player.size()
        ) {
            console::log_1(&"Treasure collected!".into());
            
            if let Some(audio) = &self.audio {
                audio.play_sound("collect");
            }
        }
    }
    
    // 添加为了map的方法
    #[wasm_bindgen]
    pub fn get_map_width(&self) -> usize {
        self.map.width()
    }

    #[wasm_bindgen]
    pub fn get_map_height(&self) -> usize {
        self.map.height()
    }

    #[wasm_bindgen]
    pub fn get_map_tile_size(&self) -> f64 {
        self.map.tile_size()
    }

    #[wasm_bindgen]
    pub fn get_map_tile(&self, x: usize, y: usize) -> i32 {
        match self.map.get_tile(x, y) {
            TileType::Empty => 0,
            TileType::Wall => 1,
            TileType::Obstacle => 2,
        }
    }

    // 为treasure添加方法
    #[wasm_bindgen]
    pub fn get_score(&self) -> u32 {
        self.treasure_manager.get_score()
    }

    #[wasm_bindgen]
    pub fn get_treasure_count(&self) -> usize {
        self.treasure_manager.get_treasure_count()
    }

    #[wasm_bindgen]
    pub fn get_treasure_x(&self, index: usize) -> Option<f64> {
        self.treasure_manager.get_treasure_x(index)
    }

    #[wasm_bindgen]
    pub fn get_treasure_y(&self, index: usize) -> Option<f64> {
        self.treasure_manager.get_treasure_y(index)
    }

    #[wasm_bindgen]
    pub fn is_treasure_collected(&self, index: usize) -> bool {
        self.treasure_manager.is_treasure_collected(index)
    }    
}