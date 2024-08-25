use crate::constants::block_constants::BLOCK_CHUNK_SIDE;
use crate::constants::colors::{
    BACKGROUND_COLOR, BLUE, COLOR_VAR_WEIGHTS, GREEN, RED, RED_VAR, YELLOW
};
use crate::constants::game_config::RANDOMIZE_BLOCK_COLOUR;
use crate::constants::map_constants::GRAIN_SIDE_SIZE;
use crate::field::Field;

use macroquad::prelude::*;
use ::rand::distributions::WeightedIndex;
use ::rand::prelude::Distribution;

pub struct GraphicController {}

impl GraphicController {
    pub async fn draw_fields(fields: Vec<Field>) {
        for field in fields {
            GraphicController::draw_field(&field);
        }
    }

    pub fn draw_background() {
        clear_background(BACKGROUND_COLOR);
    }

    fn draw_field(field: &Field) {
        let (win_x, win_y) = GraphicController::map_to_window_dimensions(field.get_x(), field.get_y());
        draw_rectangle(
            win_x,
            win_y,
            GRAIN_SIDE_SIZE as f32,
            GRAIN_SIDE_SIZE as f32,
            field.get_color(),
        );
    }

    pub async fn draw_block(block_schema_color: (Vec<(i32, i32)>, Color)) {
        let (block_schema, color) = block_schema_color;
        for (x, y) in block_schema {
            let (win_x, win_y) = GraphicController::map_to_window_dimensions(x, y);
            let box_color: Color = match RANDOMIZE_BLOCK_COLOUR {
                true => GraphicController::randomize_block_colour(color),
                false => color,
            };
            draw_rectangle(
                win_x,
                win_y,
                (BLOCK_CHUNK_SIDE * GRAIN_SIDE_SIZE) as f32,
                (BLOCK_CHUNK_SIDE * GRAIN_SIDE_SIZE) as f32,
                box_color,
            );
        }
    }

    fn map_to_window_dimensions(x: i32, y: i32) -> (f32, f32) {
        (
            (x * GRAIN_SIDE_SIZE) as f32,
            (y * GRAIN_SIDE_SIZE) as f32,
        )
    }

    fn randomize_block_colour(color: Color) -> Color {
        let colour_variations: [Color; 3] = match color {
            RED => RED_VAR,
            BLUE => RED_VAR,
            GREEN => RED_VAR,
            YELLOW => RED_VAR,
            _ => [color.clone(), color.clone(), color.clone()],
        };

        let dist = WeightedIndex::new(&COLOR_VAR_WEIGHTS).unwrap();
        let mut rng = ::rand::thread_rng();
        let random_color = colour_variations[dist.sample(&mut rng)];

        random_color
    }

    pub async fn flush() {
        next_frame().await;
    }
}


#[cfg(test)]
mod test {
    use crate::constants::{map_constants::{MAP_HEIGHT, MAP_WIDTH}, window_constants::{WINDOW_HEIGHT, WINDOW_WIDTH}};

    use super::*;

    #[test]
    fn test_map_to_window_dimensions() {
        assert_eq!(GraphicController::map_to_window_dimensions(0, 0), (0.0, 0.0));
        assert_eq!(GraphicController::map_to_window_dimensions(MAP_WIDTH - 1, 0), ((WINDOW_WIDTH - GRAIN_SIDE_SIZE) as f32, 0.0));
        assert_eq!(GraphicController::map_to_window_dimensions(0, MAP_HEIGHT - 1), (0.0, (WINDOW_HEIGHT - GRAIN_SIDE_SIZE) as f32));
        assert_eq!(GraphicController::map_to_window_dimensions(MAP_WIDTH - 1, MAP_HEIGHT - 1), ((WINDOW_WIDTH - GRAIN_SIDE_SIZE) as f32, (WINDOW_HEIGHT - GRAIN_SIDE_SIZE) as f32));
    }
}