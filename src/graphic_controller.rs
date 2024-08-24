use macroquad::prelude::*;
use crate::constants::colors::BACKGROUND_COLOR;
use crate::field::Field;
use crate::constants::map_constants::GRAIN_SIDE_SIZE;

pub struct GraphicController {}

impl GraphicController {
    pub async fn draw_fields(fields: Vec<Field>) {
        GraphicController::draw_background();

        for field in fields {
            GraphicController::draw_field(&field);
        }

        GraphicController::flush().await;
    }

    fn draw_background() {
        clear_background(BACKGROUND_COLOR);
    }

    fn draw_field(field: &Field) {
        let (win_x, win_y) = GraphicController::map_to_window_dimensions(field);
        draw_rectangle(
            win_x,
            win_y,
            GRAIN_SIDE_SIZE as f32,
            GRAIN_SIDE_SIZE as f32,
            field.get_color(),
        );
    }

    fn map_to_window_dimensions(field: &Field) -> (f32, f32) {
        (
            (field.get_x() * GRAIN_SIDE_SIZE) as f32,
            (field.get_y() * GRAIN_SIDE_SIZE) as f32,
        )
    }

    async fn flush() {
        next_frame().await;
    }
}


#[cfg(test)]
mod test {
    use crate::constants::{map_constants::{MAP_HEIGHT, MAP_WIDTH}, window_constants::{WINDOW_HEIGHT, WINDOW_WIDTH}};

    use super::*;

    #[test]
    fn test_map_to_window_dimensions() {
        let field_tl = Field::new(0, 0, RED);
        let field_tr = Field::new(MAP_WIDTH - 1, 0, RED);
        let field_dl = Field::new(0, MAP_HEIGHT - 1, RED);
        let field_dr = Field::new(MAP_WIDTH - 1, MAP_HEIGHT - 1, RED);

        assert_eq!(GraphicController::map_to_window_dimensions(&field_tl), (0.0, 0.0));
        assert_eq!(GraphicController::map_to_window_dimensions(&field_tr), ((WINDOW_WIDTH - GRAIN_SIDE_SIZE) as f32, 0.0));
        assert_eq!(GraphicController::map_to_window_dimensions(&field_dl), (0.0, (WINDOW_HEIGHT - GRAIN_SIDE_SIZE) as f32));
        assert_eq!(GraphicController::map_to_window_dimensions(&field_dr), ((WINDOW_WIDTH - GRAIN_SIDE_SIZE) as f32, (WINDOW_HEIGHT - GRAIN_SIDE_SIZE) as f32));
    }
}