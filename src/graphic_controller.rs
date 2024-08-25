use crate::constants::block_constants::BLOCK_CHUNK_SIDE;
use crate::constants::block_skins::{NATURAL, SKIN_SIDE};
use crate::constants::colors::{
    BACKGROUND_COLOR, BLUE, BLUE_VAR, COLOR_VAR_WEIGHTS, GREEN, GREEN_VAR, RED, RED_VAR, YELLOW, YELLOW_VAR
};
use crate::constants::map_constants::GRAIN_SIDE_SIZE;
use crate::field::Field;
use macroquad::prelude::*;
use ::rand::distributions::WeightedIndex;
use ::rand::prelude::Distribution;

pub struct GraphicController {}

impl GraphicController {
    pub fn draw_fields(fields: Vec<Field>) {
        for field in fields {
            GraphicController::draw_field(&field);
        }
    }

    pub fn draw_background() {
        clear_background(BACKGROUND_COLOR);
    }

    fn draw_field(field: &Field) {
        let (win_x, win_y) =
            GraphicController::map_to_window_dimensions(field.get_x(), field.get_y());
        draw_rectangle(
            win_x,
            win_y,
            GRAIN_SIDE_SIZE as f32,
            GRAIN_SIDE_SIZE as f32,
            field.get_color(),
        );
    }

    pub fn draw_block(block_schema_color: (Vec<(i32, i32)>, Color)) {
        let (block_schema, color) = block_schema_color;
        for (x, y, color) in GraphicController::get_skin_for_schema(block_schema, color) {
            let (win_x, win_y) = GraphicController::map_to_window_dimensions(x, y);
            draw_rectangle(
                win_x,
                win_y,
                GRAIN_SIDE_SIZE as f32,
                GRAIN_SIDE_SIZE as f32,
                color,
            );
        }
    }

    pub fn get_skin_for_schema(block_schema: Vec<(i32, i32)>, color: Color) -> Vec<(i32, i32, Color)> {
        let mut output = Vec::new();
        let color_variations: [Color; 3] = match color {
            RED => RED_VAR,
            BLUE => BLUE_VAR,
            GREEN => GREEN_VAR,
            YELLOW => YELLOW_VAR,
            _ => [color.clone(), color.clone(), color.clone()],
        };

        for (x, y) in block_schema {
            for x_offset in 0..=BLOCK_CHUNK_SIDE {
                for y_offset in 0..=BLOCK_CHUNK_SIDE {
                    output.push(
                        (
                            x + x_offset,
                            y + y_offset,
                            color_variations[GraphicController::get_block_skin(x_offset, y_offset)],
                        )
                    );
                }
            }
        }

        output
    }

    fn get_block_skin(x: i32, y: i32) -> usize {
        let x_normalized = x as usize % SKIN_SIDE;
        let y_normalized = y as usize % SKIN_SIDE;
        NATURAL[y_normalized][x_normalized]
    }

    pub fn draw_text_with_outline(text: &str, x: f32, y: f32, font_size: u16, inner_color: Color, outer_color: Color, border_width: u16) {
        GraphicController::draw_text(text, x - border_width as f32 / 2.0, y - border_width as f32 / 2.0, font_size, outer_color);
        GraphicController::draw_text(text, x + border_width as f32 / 2.0, y + border_width as f32 / 2.0, font_size, outer_color);
        GraphicController::draw_text(text, x, y, font_size, inner_color);
    }

    pub fn draw_text(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font_size,
                color,
                ..Default::default()
            },
        );
    }

    pub fn get_text_center(text: &str, font_size: u16) -> (f32, f32) {
        let text_dimensions = measure_text(text, None, font_size, 1.0);

        (text_dimensions.width / 2.0, text_dimensions.height / 2.0)
    }

    pub fn map_to_window_dimensions(x: i32, y: i32) -> (f32, f32) {
        ((x * GRAIN_SIDE_SIZE) as f32, (y * GRAIN_SIDE_SIZE) as f32)
    }

    pub async fn flush() {
        next_frame().await;
    }
}

#[cfg(test)]
mod test {
    use crate::constants::{
        map_constants::{MAP_HEIGHT, MAP_WIDTH},
        window_constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    };

    use super::*;

    #[test]
    fn test_map_to_window_dimensions() {
        assert_eq!(
            GraphicController::map_to_window_dimensions(0, 0),
            (0.0, 0.0)
        );
        assert_eq!(
            GraphicController::map_to_window_dimensions(MAP_WIDTH - 1, 0),
            ((WINDOW_WIDTH - GRAIN_SIDE_SIZE) as f32, 0.0)
        );
        assert_eq!(
            GraphicController::map_to_window_dimensions(0, MAP_HEIGHT - 1),
            (0.0, (WINDOW_HEIGHT - GRAIN_SIDE_SIZE) as f32)
        );
        assert_eq!(
            GraphicController::map_to_window_dimensions(MAP_WIDTH - 1, MAP_HEIGHT - 1),
            (
                (WINDOW_WIDTH - GRAIN_SIDE_SIZE) as f32,
                (WINDOW_HEIGHT - GRAIN_SIDE_SIZE) as f32
            )
        );
    }
}
