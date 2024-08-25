use macroquad::color::Color;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::constants::block_constants::BLOCK_CHUNK_SIDE;
use crate::constants::colors::BACKGROUND_COLOR;
use crate::constants::colors::RED_VAR;
use crate::constants::map_constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::field::Field;
use crate::graphic_controller::GraphicController;

pub struct Map {
    width: i32,
    height: i32,
    grid: Vec<Vec<Field>>,
}

#[derive(PartialEq)]
pub enum ColisionType {
    BorderColision,
    SandColision,
    NoColision,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let grid = Map::create_grid(width, height);

        Map {
            width,
            height,
            grid,
        }
    }

    fn create_grid(width: i32, height: i32) -> Vec<Vec<Field>> {
        let mut grid: Vec<Vec<Field>> = Vec::new();
        for y in 0..height {
            grid.push(Vec::new());
            for x in 0..width {
                grid[y as usize].push(Field::new(x, y, BACKGROUND_COLOR));
            }
        }
        grid
    }

    pub fn add_field(&mut self, x: i32, y: i32, color: Color) {
        match self.get_field(x, y) {
            Some(_) => self.grid[y as usize][x as usize].set_color(color),
            None => (),
        };
    }

    pub fn get_field(&self, x: i32, y: i32) -> Option<&Field> {
        if !self.check_coords_in_bounds(x, y) {
            return None;
        }
        Some(&self.grid[y as usize][x as usize])
    }

    pub fn change_field_color(&mut self, x: i32, y: i32, new_color: Color) {
        if !self.check_coords_in_bounds(x, y) {
            return ();
        }
        self.grid[y as usize][x as usize].set_color(new_color);
    }

    fn check_coords_in_bounds(&self, x: i32, y: i32) -> bool {
        if (0 <= x && x < self.width) && (0 <= y && y < self.height) {
            return true;
        }
        false
    }

    pub fn get_fields_to_draw(&self) -> Vec<Field> {
        let mut output: Vec<Field> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_field(x, y).unwrap().do_draw() {
                    output.push(self.get_field(x, y).unwrap().clone())
                }
            }
        }
        output
    }

    pub fn tick(&mut self) {
        let row_order: Vec<i32> = self.get_random_row_order();
        for y in (0..self.height).rev() {
            for x in (&row_order).into_iter() {
                if !self.grid[y as usize][*x as usize].is_empty() {
                    let (new_x, new_y) = self.get_new_pos(*x, y);
                    if (new_x, new_y) != (*x, y) {
                        let field_color = self.get_field(*x, y).unwrap().get_color();
                        self.change_field_color(new_x, new_y, field_color);
                        self.change_field_color(*x, y, BACKGROUND_COLOR);
                    }
                }
            }
        }
    }

    fn get_random_row_order(&self) -> Vec<i32> {
        let mut row_order: Vec<i32> = (0..self.width).collect();
        row_order.shuffle(&mut thread_rng());

        row_order
    }

    fn get_new_pos(&self, x: i32, y: i32) -> (i32, i32) {
        let field_down = self.get_field(x, y + 1);
        let field_down_left = self.get_field(x - 1, y + 1);
        let field_down_right = self.get_field(x + 1, y + 1);

        match field_down {
            Some(field) if field.is_empty() => (x, y + 1), // No grain below
            Some(_) if field_down_left.is_some() && field_down_right.is_some() => {
                // If grain below
                if field_down_left.unwrap().is_empty() && !field_down_right.unwrap().is_empty() {
                    // If right down field empty
                    (x - 1, y + 1)
                } else if !field_down_left.unwrap().is_empty()
                    && field_down_right.unwrap().is_empty()
                {
                    // If left down field empty
                    (x + 1, y + 1)
                } else if field_down_left.unwrap().is_empty()
                    && field_down_right.unwrap().is_empty()
                {
                    // If both sides empty, choose random
                    let go_right: bool = rand::thread_rng().gen_range(0..=1) == 0;
                    match go_right {
                        true => (x + 1, y + 1),
                        false => (x - 1, y + 1),
                    }
                } else {
                    // Both down sides not empty
                    (x, y)
                }
            }
            Some(_) if field_down_left.is_none() && field_down_right.is_some() => {
                // left out of bounds
                if field_down_right.unwrap().is_empty() {
                    (x + 1, y + 1)
                } else {
                    (x, y)
                }
            }
            Some(_) if field_down_left.is_some() && field_down_right.is_none() => {
                // right out of bounds
                if field_down_left.unwrap().is_empty() {
                    (x - 1, y + 1)
                } else {
                    (x, y)
                }
            }
            Some(_) if field_down_left.is_none() && field_down_right.is_none() => {
                // both out of bounds
                (x, y)
            }
            Some(_) | None => (x, y),
        }
    }

    pub fn can_move_down(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> (bool, ColisionType) {
        if self.is_block_coliding_bottom_border(schema, (center_pos.0, center_pos.1 + 1)) {
            return (false, ColisionType::BorderColision);
        } else if self.is_block_coliding_with_sand(schema, (center_pos.0, center_pos.1 + 1)) {
            return (false, ColisionType::SandColision);
        }

        (true, ColisionType::NoColision)
    }

    pub fn can_move_left(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> (bool, ColisionType) {
        if self.is_block_coliding_left_border(schema, (center_pos.0 - 1, center_pos.1)) {
            return (false, ColisionType::BorderColision);
        } else if self.is_block_coliding_with_sand(schema, (center_pos.0 - 1, center_pos.1)) {
            return (false, ColisionType::SandColision);
        }

        (true, ColisionType::NoColision)
    }

    pub fn can_move_right(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> (bool, ColisionType) {
        if self.is_block_coliding_right_border(schema, (center_pos.0 + 1, center_pos.1)) {
            return (false, ColisionType::BorderColision);
        } else if self.is_block_coliding_with_sand(schema, (center_pos.0 + 1, center_pos.1)) {
            return (false, ColisionType::SandColision);
        }

        (true, ColisionType::NoColision)
    }

    pub fn can_rotate(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> (bool, ColisionType) {
        if self.is_block_coliding_with_any_border(schema, center_pos) {
            return (false, ColisionType::BorderColision);
        } else if self.is_block_coliding_with_sand(schema, center_pos) {
            return (false, ColisionType::SandColision);
        }

        (true, ColisionType::NoColision)
    }

    fn is_block_coliding_with_any_border(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> bool {
        self.is_block_coliding_bottom_border(schema, center_pos)
            || self.is_block_coliding_left_border(schema, center_pos)
            || self.is_block_coliding_right_border(schema, center_pos)
    }

    fn is_block_coliding_upper_border(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> bool {
        let upper_most_box: (i8, i8) = *schema
            .into_iter()
            .min_by_key(|schema_box| schema_box.1)
            .unwrap();
        let upper_border = center_pos.1 + (upper_most_box.1 as i32 + 1) * BLOCK_CHUNK_SIDE;

        upper_border < 0
    }

    fn is_block_coliding_bottom_border(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> bool {
        let bottom_most_box: (i8, i8) = *schema
            .into_iter()
            .max_by_key(|schema_box| schema_box.1)
            .unwrap();
        let bottom_border = center_pos.1 + (bottom_most_box.1 as i32 + 1) * BLOCK_CHUNK_SIDE;

        bottom_border > MAP_HEIGHT
    }

    fn is_block_coliding_left_border(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> bool {
        let bottom_most_box: (i8, i8) = *schema
            .into_iter()
            .min_by_key(|schema_box| schema_box.0)
            .unwrap();

        let left_border = center_pos.0 + bottom_most_box.0 as i32 * BLOCK_CHUNK_SIDE;

        left_border < 0
    }

    fn is_block_coliding_right_border(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> bool {
        let right_most_box: (i8, i8) = *schema
            .into_iter()
            .max_by_key(|schema_box| schema_box.0)
            .unwrap();

        let right_border = center_pos.0 + (right_most_box.0 as i32 + 1) * BLOCK_CHUNK_SIDE;

        right_border > MAP_WIDTH
    }

    fn is_block_coliding_with_sand(&self, schema: &Vec<(i8, i8)>, center_pos: (i32, i32)) -> bool {
        for (x, y) in self.get_fields_from_schema(schema, center_pos) {
            let result: bool = match self.get_field(x, y) {
                Some(field) => !field.is_empty(),
                None => false,
            };
            if result {
                return true;
            }
        }

        false
    }

    pub fn spawn_block(&mut self, schema: Vec<(i32, i32)>, color: Color) {
        for (x, y, color) in GraphicController::get_skin_for_schema(schema, color) {
            self.add_field(x, y, color);
        }
    }

    fn get_fields_from_schema(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> Vec<(i32, i32)> {
        let mut output = Vec::new();

        for block_box in schema {
            let x: i32 = center_pos.0 + block_box.0 as i32 * BLOCK_CHUNK_SIDE;
            let y: i32 = center_pos.1 + block_box.1 as i32 * BLOCK_CHUNK_SIDE;
            for x_offset in 0..=BLOCK_CHUNK_SIDE {
                for y_offset in 0..=BLOCK_CHUNK_SIDE {
                    output.push((x + x_offset, y + y_offset));
                }
            }
        }

        output
    }

    pub fn is_game_over(&self, schema: &Vec<(i8, i8)>, center_pos: (i32, i32)) -> bool {
        self.is_block_coliding_upper_border(schema, center_pos)
    }

    pub fn clear(&mut self) {
        self.grid = Map::create_grid(self.width, self.height);
    }
}

#[cfg(test)]
mod test {
    use macroquad::color::RED;

    use super::*;

    #[test]
    fn create_map() {
        let map: Map = Map::new(200, 400);

        assert_eq!(map.width, 200);
        assert_eq!(map.height, 400);
        assert_eq!(map.grid.len(), 400);
        assert_eq!(map.grid.get(0).unwrap().len(), 200);
    }

    #[test]
    fn get_field() {
        let map: Map = Map::new(200, 400);

        let field_middle: &Field = map.get_field(20, 40).unwrap();
        let field_min: &Field = map.get_field(0, 0).unwrap();
        let field_max: &Field = map.get_field(199, 399).unwrap();

        assert_eq!(field_middle.get_x(), 20);
        assert_eq!(field_middle.get_y(), 40);
        assert_eq!(field_min.get_x(), 0);
        assert_eq!(field_min.get_y(), 0);
        assert_eq!(field_max.get_x(), 199);
        assert_eq!(field_max.get_y(), 399);
    }

    #[test]
    fn get_field_out_of_bounds() {
        let map: Map = Map::new(200, 400);

        for (x, y) in [(200, 0), (201, 40), (0, 400), (100, 401)] {
            let field_middle = map.get_field(x, y);

            assert!(field_middle.is_none());
        }
    }

    #[test]
    fn get_fields_to_draw() {
        let mut map: Map = Map::new(200, 400);
        map.change_field_color(40, 20, RED);
        map.change_field_color(30, 15, RED);
        map.change_field_color(100, 0, RED);

        let fields_to_draw: Vec<Field> = map.get_fields_to_draw();

        assert_eq!(fields_to_draw.len(), 3);
        assert!(fields_to_draw.contains(&Field::new(40, 20, RED)));
        assert!(fields_to_draw.contains(&Field::new(30, 15, RED)));
        assert!(fields_to_draw.contains(&Field::new(100, 0, RED)));
    }

    #[test]
    fn grains_move_down() {
        let mut map: Map = Map::new(200, 400);
        map.change_field_color(40, 20, RED);

        map.tick();

        assert_eq!(map.get_fields_to_draw().len(), 1);
        assert_eq!(map.get_field(40, 20).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(40, 21).unwrap().get_color(), RED);
    }

    #[test]
    fn grains_move_down_to_side() {
        //    #      ->
        //    #      ->     ##

        let mut map: Map = Map::new(10, 10);
        map.change_field_color(5, 9, RED); // Grain under
        map.change_field_color(5, 8, RED); // Grain above

        map.tick();

        assert_eq!(map.get_fields_to_draw().len(), 2);
        assert_eq!(map.get_field(5, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(5, 8).unwrap().get_color(), BACKGROUND_COLOR);

        let possible_positions: [Color; 2] = [
            map.get_field(4, 9).unwrap().get_color(),
            map.get_field(6, 9).unwrap().get_color(),
        ];
        assert!(
            (possible_positions[0] == RED && possible_positions[1] == BACKGROUND_COLOR)
                || (possible_positions[1] == RED && possible_positions[0] == BACKGROUND_COLOR)
        );
    }

    #[test]
    fn grains_move_down_to_side_left_wall() {
        let mut map: Map = Map::new(10, 10);
        map.change_field_color(0, 9, RED); // Grain under
        map.change_field_color(0, 8, RED); // Grain above

        map.tick();

        assert_eq!(map.get_fields_to_draw().len(), 2);
        assert_eq!(map.get_field(0, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(0, 8).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(1, 9).unwrap().get_color(), RED);
    }

    #[test]
    fn grains_move_down_to_side_left_blocked() {
        let mut map: Map = Map::new(10, 10);
        map.change_field_color(1, 9, RED); // Grain under
        map.change_field_color(1, 8, RED); // Grain above
        map.change_field_color(0, 9, RED); // left Grain above
        map.change_field_color(0, 8, RED); // left Grain above

        map.tick();

        assert_eq!(map.get_fields_to_draw().len(), 4);
        assert_eq!(map.get_field(1, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(1, 8).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(0, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(0, 8).unwrap().get_color(), RED);
        assert_eq!(map.get_field(2, 9).unwrap().get_color(), RED);
    }

    #[test]
    fn grains_move_down_to_side_right_wall() {
        let mut map: Map = Map::new(10, 10);
        map.change_field_color(9, 9, RED); // Grain under
        map.change_field_color(9, 8, RED); // Grain above

        map.tick();

        assert_eq!(map.get_fields_to_draw().len(), 2);
        assert_eq!(map.get_field(9, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(9, 8).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(8, 9).unwrap().get_color(), RED);
    }

    #[test]
    fn grains_move_down_to_side_right_blocked() {
        let mut map: Map = Map::new(10, 10);
        map.change_field_color(8, 9, RED); // Grain under
        map.change_field_color(8, 8, RED); // Grain above
        map.change_field_color(9, 9, RED); // right Grain above
        map.change_field_color(9, 8, RED); // right Grain above

        map.tick();

        assert_eq!(map.get_fields_to_draw().len(), 4);
        assert_eq!(map.get_field(8, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(8, 8).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(9, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(9, 8).unwrap().get_color(), RED);
        assert_eq!(map.get_field(7, 9).unwrap().get_color(), RED);
    }
}
