use ::rand::seq::SliceRandom;
use ::rand::thread_rng;
use macroquad::{color::Color, input::KeyCode};

use crate::{
    block_controller::BlockController,
    constants::{
        animation_constants::DEMOLISHION_CHUNK_SIZE,
        colors::{BLACK, WHITE},
        interface_constants::{
            GAME_OVER_BOTTOM_FONT_SIZE, GAME_OVER_BOTTOM_TEXT, GAME_OVER_FONT_SIZE,
            GAME_OVER_OUTLINE_WIDTH, GAME_OVER_TEXT, H_BORDER_OFFSET, SCORE_FONT_SIZE,
            SCORE_OUTLINE_WIDTH, SCORE_TEXT, V_BORDER_OFFSET,
        },
        map_constants::{MAP_HEIGHT, MAP_WIDTH},
    },
    field::Field,
    graphic_controller::GraphicController,
    map::Map,
};

pub struct GameController {
    score: u32,
    is_game_over: bool,
    block_controller: BlockController,
    map: Map,
}

impl GameController {
    pub fn new() -> GameController {
        let block_controller: BlockController = BlockController::new();
        let map: Map = Map::new(MAP_WIDTH, MAP_HEIGHT);

        GameController {
            score: 0,
            is_game_over: false,
            block_controller,
            map,
        }
    }

    pub fn reset_game(&mut self) {
        self.score = 0;
        self.is_game_over = false;
        self.map.clear();
        self.block_controller.clear();
    }

    pub async fn tick(&mut self) {
        self.draw_game().await;

        if self.is_game_over {
            return ();
        }

        self.tick_map_and_update_score().await;
        self.tick_block_and_check_game_over().await;
    }

    async fn draw_game(&mut self) {
        self.draw_gamefield();

        if self.is_game_over {
            self.display_game_over();
        }

        GraphicController::flush().await;
    }

    async fn tick_map_and_update_score(&mut self) {
        let score_fields: Vec<(i32, i32)> = self.map.tick_and_get_score_fields();
        if score_fields.len() > 0 {
            self.draw_row_demolishion(&score_fields).await;
            self.map.demolish_fields(&score_fields);
            self.score += score_fields.len() as u32;
        }
    }

    async fn tick_block_and_check_game_over(&mut self) {
        let is_game_over = self
            .block_controller
            .tick_and_check_game_over(&mut self.map);
        if is_game_over {
            self.handle_game_over();
        }
    }

    fn handle_game_over(&mut self) {
        self.is_game_over = true;
    }

    fn draw_gamefield(&self) {
        GraphicController::draw_background();
        GraphicController::draw_block(self.block_controller.get_block_to_draw());
        GraphicController::draw_fields(&self.map.get_fields_to_draw());
        self.draw_interface();
    }

    fn draw_interface(&self) {
        self.draw_score();
        self.draw_next_block();
    }

    fn draw_score(&self) {
        let score_text = format!("{}:{}", SCORE_TEXT, self.score);
        let text_center = GraphicController::get_text_center(&score_text, SCORE_FONT_SIZE);
        let score_position = GraphicController::map_to_window_dimensions(MAP_WIDTH, 0);

        GraphicController::draw_text_with_outline(
            &score_text,
            score_position.0 - 2.0 * text_center.0 - H_BORDER_OFFSET,
            score_position.1 + 2.0 * text_center.1 + V_BORDER_OFFSET,
            SCORE_FONT_SIZE,
            BLACK,
            WHITE,
            SCORE_OUTLINE_WIDTH,
        );
    }

    fn draw_next_block(&self) {
        let next_block = self.get_next_block_miniature();

        GraphicController::draw_block_miniature(next_block, (H_BORDER_OFFSET, V_BORDER_OFFSET));
    }

    fn get_next_block_miniature(&self) -> (Vec<(i32, i32)>, Color) {
        let mut next_block = self.block_controller.get_next_block_miniature();
        let x_normalize = next_block.0.iter().min_by_key(|block| block.0).unwrap().0;
        let y_normalize = next_block.0.iter().min_by_key(|block| block.1).unwrap().1;

        for unnormalized_block in next_block.0.iter_mut() {
            unnormalized_block.0 += x_normalize.abs();
            unnormalized_block.1 += y_normalize.abs();
        }

        next_block
    }

    fn display_game_over(&self) {
        let text_center = GraphicController::get_text_center(GAME_OVER_TEXT, GAME_OVER_FONT_SIZE);
        let bottom_text_center =
            GraphicController::get_text_center(GAME_OVER_BOTTOM_TEXT, GAME_OVER_BOTTOM_FONT_SIZE);
        let map_center = GraphicController::map_to_window_dimensions(MAP_WIDTH / 2, MAP_HEIGHT / 2);

        GraphicController::draw_text_with_outline(
            GAME_OVER_TEXT,
            map_center.0 - text_center.0,
            map_center.1 + text_center.1,
            GAME_OVER_FONT_SIZE,
            BLACK,
            WHITE,
            GAME_OVER_OUTLINE_WIDTH,
        );
        GraphicController::draw_text_with_outline(
            GAME_OVER_BOTTOM_TEXT,
            map_center.0 - bottom_text_center.0,
            map_center.1 + text_center.1 + 2.0 * bottom_text_center.1,
            GAME_OVER_BOTTOM_FONT_SIZE,
            BLACK,
            WHITE,
            GAME_OVER_OUTLINE_WIDTH,
        );
    }

    async fn draw_row_demolishion(&self, fields_coords: &Vec<(i32, i32)>) {
        let fields_to_demolish = self.get_shuffled_fields(fields_coords);
        let mut demolishion_stash = Vec::new();

        for fields in fields_to_demolish.chunks(DEMOLISHION_CHUNK_SIZE) {
            demolishion_stash.extend(fields);

            self.draw_gamefield();
            GraphicController::draw_fields_vanish(&demolishion_stash);

            GraphicController::flush().await;
        }
    }

    fn get_shuffled_fields(&self, fields_coords: &Vec<(i32, i32)>) -> Vec<&Field> {
        let mut fields_to_demolish = Vec::new();
        for (x, y) in fields_coords {
            fields_to_demolish.push(self.map.get_field(*x, *y).unwrap());
        }
        fields_to_demolish.shuffle(&mut thread_rng());
        fields_to_demolish
    }

    pub fn do_move(&mut self, key: KeyCode) {
        if self.is_game_over {
            return ();
        }
        let is_game_over = match key {
            KeyCode::D => self.block_controller.handle_move_right(&mut self.map),
            KeyCode::A => self.block_controller.handle_move_left(&mut self.map),
            KeyCode::S => self.block_controller.handle_move_down(&mut self.map),
            KeyCode::E => self.block_controller.handle_rotate_clockwise(&mut self.map),
            KeyCode::Q => self
                .block_controller
                .handle_rotate_counter_clockwise(&mut self.map),
            _ => false,
        };
        if is_game_over {
            self.handle_game_over();
        }
    }
}
