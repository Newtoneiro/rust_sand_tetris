use macroquad::prelude::*;

use crate::{
    constants::{
        block_skins::{NATURAL, SKIN_SIDE},
        colors::{
            BACKGROUND_COLOR, BLUE, BLUE_VAR, GREEN, GREEN_VAR, RED, RED_VAR, YELLOW, YELLOW_VAR,
        },
    },
    objects::field::Field,
};

pub struct GraphicController {}

impl GraphicController {
    pub fn draw_background() {
        clear_background(BACKGROUND_COLOR);
    }

    pub fn draw_fields(fields: &Vec<&Field>, grain_side_size: i32) {
        for field in fields {
            GraphicController::draw_field(field, grain_side_size);
        }
    }

    pub fn draw_field(field: &Field, grain_side_size: i32) {
        let (win_x, win_y) = GraphicController::map_to_window_dimensions(
            field.get_x(),
            field.get_y(),
            grain_side_size,
        );
        draw_rectangle(
            win_x,
            win_y,
            grain_side_size as f32,
            grain_side_size as f32,
            field.get_color(),
        );
    }

    pub fn draw_block(
        block_schema_color: (Vec<(i32, i32)>, Color),
        grain_side_size: i32,
        block_chunk_side: i32,
    ) {
        let (block_schema, color) = block_schema_color;
        for (x, y, color) in
            GraphicController::get_skin_for_schema(block_schema, color, block_chunk_side)
        {
            let (win_x, win_y) = GraphicController::map_to_window_dimensions(x, y, grain_side_size);
            draw_rectangle(
                win_x,
                win_y,
                grain_side_size as f32,
                grain_side_size as f32,
                color,
            );
        }
    }

    pub fn draw_block_miniature(
        block_schema_color: (Vec<(i32, i32)>, Color),
        origin_point: (f32, f32),
        preview_block_chunk_side: i32,
        grain_side_size: i32,
    ) {
        let (block_schema, color) = block_schema_color;
        for (x, y) in block_schema {
            let (win_x, win_y) = GraphicController::map_to_window_dimensions(x, y, grain_side_size);
            draw_rectangle(
                win_x + origin_point.0,
                win_y + origin_point.1,
                (preview_block_chunk_side * grain_side_size) as f32,
                (preview_block_chunk_side * grain_side_size) as f32,
                color,
            );
        }
    }

    pub fn draw_text_with_outline(
        text: &str,
        x: f32,
        y: f32,
        font_size: u16,
        inner_color: Color,
        outer_color: Color,
        border_width: u16,
    ) {
        GraphicController::draw_text(
            text,
            x - border_width as f32 / 2.0,
            y - border_width as f32 / 2.0,
            font_size,
            outer_color,
        );
        GraphicController::draw_text(
            text,
            x + border_width as f32 / 2.0,
            y + border_width as f32 / 2.0,
            font_size,
            outer_color,
        );
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

    pub fn draw_fields_vanish(fields: &Vec<&Field>, grain_side_size: i32) {
        for field in fields {
            GraphicController::draw_field_vanish(field, grain_side_size);
        }
    }

    fn draw_field_vanish(field: &Field, grain_side_size: i32) {
        let (win_x, win_y) = GraphicController::map_to_window_dimensions(
            field.get_x(),
            field.get_y(),
            grain_side_size,
        );
        draw_rectangle(
            win_x,
            win_y,
            grain_side_size as f32,
            grain_side_size as f32,
            BACKGROUND_COLOR,
        );
    }

    pub fn get_skin_for_schema(
        block_schema: Vec<(i32, i32)>,
        color: Color,
        block_chunk_side: i32,
    ) -> Vec<(i32, i32, Color)> {
        let mut output = Vec::new();
        let color_variations: [Color; 3] = match color {
            RED => RED_VAR,
            BLUE => BLUE_VAR,
            GREEN => GREEN_VAR,
            YELLOW => YELLOW_VAR,
            _ => [color.clone(), color.clone(), color.clone()],
        };

        for (x, y) in block_schema {
            for x_offset in 0..block_chunk_side {
                for y_offset in 0..block_chunk_side {
                    output.push((
                        x + x_offset,
                        y + y_offset,
                        color_variations[GraphicController::get_block_skin(x_offset, y_offset)],
                    ));
                }
            }
        }

        output
    }

    pub fn normalize_color(color: Color) -> Color {
        if RED_VAR.contains(&color) {
            return RED;
        } else if BLUE_VAR.contains(&color) {
            return BLUE;
        } else if GREEN_VAR.contains(&color) {
            return GREEN;
        } else if YELLOW_VAR.contains(&color) {
            return YELLOW;
        } else {
            return BACKGROUND_COLOR;
        }
    }

    fn get_block_skin(x: i32, y: i32) -> usize {
        let x_normalized = x as usize % SKIN_SIDE;
        let y_normalized = y as usize % SKIN_SIDE;
        NATURAL[y_normalized][x_normalized]
    }

    pub fn get_text_center(text: &str, font_size: u16) -> (f32, f32) {
        let text_dimensions = measure_text(text, None, font_size, 1.0);

        (text_dimensions.width / 2.0, text_dimensions.height / 2.0)
    }

    pub fn map_to_window_dimensions(x: i32, y: i32, grain_side_size: i32) -> (f32, f32) {
        ((x * grain_side_size) as f32, (y * grain_side_size) as f32)
    }

    pub async fn flush() {
        next_frame().await;
    }
}

#[cfg(test)]
mod test {
    use crate::constants::{
        map_constants::{GRAIN_SIDE_SIZE, MAP_HEIGHT, MAP_WIDTH},
        window_constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    };

    use super::*;

    #[test]
    fn draw_background() {
        // let mut called_clear_background = false;

        // macroquad::prelude::clear_background = |color| {
        //     called_clear_background = true;
        //     assert_eq!(color, BACKGROUND_COLOR);
        // };

        // GraphicController::draw_background();

        // assert!(called_clear_background);
    }

    #[test]
    fn test_map_to_window_dimensions() {
        assert_eq!(
            GraphicController::map_to_window_dimensions(0, 0, GRAIN_SIDE_SIZE),
            (0.0, 0.0)
        );
        assert_eq!(
            GraphicController::map_to_window_dimensions(MAP_WIDTH - 1, 0, GRAIN_SIDE_SIZE),
            ((WINDOW_WIDTH - GRAIN_SIDE_SIZE) as f32, 0.0)
        );
        assert_eq!(
            GraphicController::map_to_window_dimensions(0, MAP_HEIGHT - 1, GRAIN_SIDE_SIZE),
            (0.0, (WINDOW_HEIGHT - GRAIN_SIDE_SIZE) as f32)
        );
        assert_eq!(
            GraphicController::map_to_window_dimensions(
                MAP_WIDTH - 1,
                MAP_HEIGHT - 1,
                GRAIN_SIDE_SIZE
            ),
            (
                (WINDOW_WIDTH - GRAIN_SIDE_SIZE) as f32,
                (WINDOW_HEIGHT - GRAIN_SIDE_SIZE) as f32
            )
        );
    }

    #[test]
    fn normalize_color() {
        for r_color in RED_VAR {
            assert_eq!(GraphicController::normalize_color(r_color), RED);
        }
        for b_color in BLUE_VAR {
            assert_eq!(GraphicController::normalize_color(b_color), BLUE);
        }
        for g_color in GREEN_VAR {
            assert_eq!(GraphicController::normalize_color(g_color), GREEN);
        }
        for y_color in YELLOW_VAR {
            assert_eq!(GraphicController::normalize_color(y_color), YELLOW);
        }
        for other_color in [WHITE, BACKGROUND_COLOR] {
            assert_eq!(
                GraphicController::normalize_color(other_color),
                BACKGROUND_COLOR
            );
        }
    }
}
