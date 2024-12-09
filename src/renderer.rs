// src/renderer.rs
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::{game::GameState, Game};

#[wasm_bindgen]
pub struct Renderer {
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, width: u32, height: u32) -> Result<Renderer, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        canvas.set_width(width);
        canvas.set_height(height);

        Ok(Renderer {
            context,
            width,
            height,
        })
    }

    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    #[wasm_bindgen]
    pub fn render(&self, game: &Game) {
        self.clear();
        self.render_map(game);
        self.render_player(game);
        self.render_treasures(game);
        self.render_ui(game);
        match game.get_state() {
            GameState::Won => self.render_victory_screen(game),
            GameState::TimeUp => self.render_game_over_screen(),
            _ => {}
        }
    }

    fn render_map(&self, game: &Game) {
        for y in 0..game.get_map_height() {
            for x in 0..game.get_map_width() {
                let tile = game.get_map_tile(x, y);
                let color = match tile {
                    0 => "#FFFFFF", // Empty
                    1 => "#333333", // Wall
                    2 => "#666666", // Obstacle
                    _ => "#FF0000", // Error case
                };

                self.context.set_fill_style(&JsValue::from_str(color));
                self.context.fill_rect(
                    x as f64 * game.get_map_tile_size(),
                    y as f64 * game.get_map_tile_size(),
                    game.get_map_tile_size(),
                    game.get_map_tile_size(),
                );
            }
        }
    }

    fn render_player(&self, game: &Game) {
        let player = game.player();
        self.context.set_fill_style(&JsValue::from_str("#0000FF"));
        self.context.fill_rect(
            player.x(),
            player.y(),
            player.size(),
            player.size(),
        );
    }

    fn render_treasures(&self, game: &Game) {
        for i in 0..game.get_treasure_count() {
            if !game.is_treasure_collected(i) {
                if let (Some(x), Some(y)) = (game.get_treasure_x(i), game.get_treasure_y(i)) {
                    self.context.set_fill_style(&JsValue::from_str("#FFD700")); // 金色
                    self.context.begin_path();
                    self.context.arc(
                        x,
                        y,
                        10.0, // 宝藏半径
                        0.0,
                        2.0 * std::f64::consts::PI,
                    ).unwrap();
                    self.context.fill();
                }
            }
        }
    }

    fn render_ui(&self, game: &Game) {
        self.context.set_fill_style(&JsValue::from_str("#000000"));
        self.context.set_font("20px Arial");
        
        // 显示分数
        self.context.fill_text(
            &format!("Score: {}", game.get_score()),
            10.0,
            30.0,
        ).unwrap();

        // 显示剩余时间，保留一位小数
        let remaining_time = game.get_remaining_time().max(0.0);
        self.context.fill_text(
            &format!("Time: {:.1}", remaining_time),
            10.0,
            60.0,
        ).unwrap();

        // 显示最佳时间（如果有）
        if let Some(best_time) = game.get_best_time() {
            self.context.fill_text(
                &format!("Best: {:.1}", best_time),
                10.0,
                90.0,
            ).unwrap();
        }
    }

    fn render_victory_screen(&self, game: &Game) {
        self.render_overlay();
        self.context.set_fill_style(&JsValue::from_str("#000000"));
        self.context.set_font("40px Arial");
        self.context.fill_text("Victory!", self.width as f64 / 2.0 - 70.0, self.height as f64 / 2.0 - 40.0).unwrap();
        self.context.set_font("20px Arial");
        self.context.fill_text(
            &format!("Time: {:.1} seconds", game.get_game_time()),
            self.width as f64 / 2.0 - 80.0,
            self.height as f64 / 2.0,
        ).unwrap();
    }

    fn render_game_over_screen(&self) {
        self.render_overlay();
        self.context.set_fill_style(&JsValue::from_str("#000000"));
        self.context.set_font("40px Arial");
        self.context.fill_text(
            "Time's Up!",
            self.width as f64 / 2.0 - 80.0,
            self.height as f64 / 2.0,
        ).unwrap();
    }

    fn render_overlay(&self) {
        self.context.set_global_alpha(0.7);
        self.context.set_fill_style(&JsValue::from_str("#FFFFFF"));
        self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
        self.context.set_global_alpha(1.0);
    }
}