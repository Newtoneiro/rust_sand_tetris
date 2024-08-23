use macroquad::color::Color;

use crate::constants::colors::BACKGROUND_COLOR;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    x: usize,
    y: usize,
    color: Color,
}

impl Field {
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        Field { x, y, color }
    }

    pub fn do_draw(&self) -> bool {
        self.color != BACKGROUND_COLOR
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }
}

#[cfg(test)]
mod test {
    use macroquad::color::RED;

    use super::*;

    #[test]
    fn create_field() {
        let field: Field = Field::new(1, 2, RED);

        assert_eq!(field.get_x(), 1);
        assert_eq!(field.get_y(), 2);
        assert_eq!(field.get_color(), RED);
    }

    #[test]
    fn do_draw() {
        let field_dont: Field = Field::new(1, 2, BACKGROUND_COLOR);
        let field_do: Field = Field::new(1, 2, RED);

        assert_eq!(field_dont.do_draw(), false);
        assert_eq!(field_do.do_draw(), true);
    }
}