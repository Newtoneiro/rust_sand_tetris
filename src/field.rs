use macroquad::color::Color;

use crate::constants::colors::BACKGROUND_COLOR;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    x: i32,
    y: i32,
    color: Color,
    group_id: u32,
}

impl Field {
    pub fn new(x: i32, y: i32, color: Color, group_id: u32) -> Self {
        Field {
            x,
            y,
            color,
            group_id,
        }
    }

    pub fn do_draw(&self) -> bool {
        self.color != BACKGROUND_COLOR
    }

    pub fn is_empty(&self) -> bool {
        self.color == BACKGROUND_COLOR
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }

    pub fn get_group_id(&self) -> u32 {
        self.group_id
    }

    pub fn set_group_id(&mut self, new_group_id: u32) {
        self.group_id = new_group_id;
    }
}

#[cfg(test)]
mod test {
    use macroquad::color::RED;

    use super::*;

    #[test]
    fn create_field() {
        let field: Field = Field::new(1, 2, RED, 0);

        assert_eq!(field.get_x(), 1);
        assert_eq!(field.get_y(), 2);
        assert_eq!(field.get_color(), RED);
        assert_eq!(field.get_group_id(), 0);
    }

    #[test]
    fn do_draw() {
        let field_dont: Field = Field::new(1, 2, BACKGROUND_COLOR, 0);
        let field_do: Field = Field::new(1, 2, RED, 0);

        assert_eq!(field_dont.do_draw(), false);
        assert_eq!(field_do.do_draw(), true);
    }
}
