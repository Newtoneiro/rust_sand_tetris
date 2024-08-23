use macroquad::prelude::*;
use crate::constants::colors::BACKGROUND_COLOR;
use crate::field::Field;
use crate::constants::map_constants::GRAIN_SIDE_SIZE;

pub struct GraphicController {

}

impl GraphicController {
    pub fn new() -> Self {
        GraphicController {}
    }

    pub async fn draw_grid(grid: &Vec<Vec<Field>>) {
        GraphicController::draw_background();
        for row in grid {
            for field in row {
                GraphicController::draw_field(&field);
            }
        }
        GraphicController::flush().await;
    }

    fn draw_background() {
        clear_background(BACKGROUND_COLOR);
    }

    fn draw_field(field: &Field) {
        draw_rectangle(
            field.get_x() as f32,
            field.get_y() as f32,
            GRAIN_SIDE_SIZE as f32,
            GRAIN_SIDE_SIZE as f32,
            field.get_color(),
        );
    }

    async fn flush() {
        next_frame().await;
    }
}