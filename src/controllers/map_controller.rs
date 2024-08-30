use ::rand::thread_rng;
use macroquad::color::Color;
use rand::seq::SliceRandom;

use crate::{
    controllers::graphic_controller::GraphicController,
    objects::{field::Field, map::Map},
};

pub struct MapController {
    map: Map,
    current_group_id: u32,
    block_chunk_side: i32,
}

#[derive(PartialEq, Debug)]
pub enum ColisionType {
    BorderColision,
    SandColision,
    NoColision,
}

impl MapController {
    pub fn new(map_width: i32, map_height: i32, block_chunk_side: i32) -> Self {
        let map = Map::new(map_width, map_height);

        MapController {
            map,
            current_group_id: 1,
            block_chunk_side,
        }
    }

    pub fn is_game_over(&self, schema: &Vec<(i8, i8)>, center_pos: (i32, i32)) -> bool {
        self.is_block_coliding_upper_border(schema, center_pos)
    }

    pub fn clear(&mut self) {
        self.current_group_id = 1;
        self.map.clear();
    }

    pub fn tick_and_get_score_fields(&mut self) -> Vec<(i32, i32)> {
        self.map.tick_and_get_score_fields()
    }

    pub fn demolish_fields(&mut self, fields: &Vec<(i32, i32)>) {
        self.map.demolish_fields(fields);
    }

    pub fn get_fields_to_draw(&self) -> Vec<&Field> {
        self.map.filter_fields(|field: &Field| field.do_draw())
    }

    pub fn get_shuffled_fields(&self, fields_coords: &Vec<(i32, i32)>) -> Vec<&Field> {
        let mut fields_to_demolish = Vec::new();
        for (x, y) in fields_coords {
            fields_to_demolish.push(self.map.get_field(*x, *y).unwrap());
        }
        fields_to_demolish.shuffle(&mut thread_rng());
        fields_to_demolish
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
        let upper_border = center_pos.1 + (upper_most_box.1 as i32 + 1) * self.block_chunk_side;

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
        let bottom_border = center_pos.1 + (bottom_most_box.1 as i32 + 1) * self.block_chunk_side;

        bottom_border > self.map.get_height()
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

        let left_border = center_pos.0 + bottom_most_box.0 as i32 * self.block_chunk_side;

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

        let right_border = center_pos.0 + (right_most_box.0 as i32 + 1) * self.block_chunk_side;

        right_border > self.map.get_width()
    }

    fn is_block_coliding_with_sand(&self, schema: &Vec<(i8, i8)>, center_pos: (i32, i32)) -> bool {
        for (x, y) in self.get_fields_from_schema(schema, center_pos) {
            let result: bool = match self.map.get_field(x, y) {
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
            self.map.change_field(x, y, color, self.current_group_id);
        }
        self.current_group_id += 1;
    }

    fn get_fields_from_schema(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> Vec<(i32, i32)> {
        let mut output = Vec::new();

        for block_box in schema {
            let x: i32 = center_pos.0 + block_box.0 as i32 * self.block_chunk_side;
            let y: i32 = center_pos.1 + block_box.1 as i32 * self.block_chunk_side;
            for x_offset in 0..=self.block_chunk_side {
                for y_offset in 0..=self.block_chunk_side {
                    output.push((x + x_offset, y + y_offset));
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod test {
    use crate::constants::colors::RED;

    use super::*;

    #[test]
    fn get_fields_to_draw() {
        let mut mc: MapController = MapController::new(200, 400, 10);
        mc.map.change_field(40, 20, RED, 0);
        mc.map.change_field(30, 15, RED, 0);
        mc.map.change_field(100, 0, RED, 0);

        let fields_to_draw: Vec<&Field> = mc.get_fields_to_draw();

        assert_eq!(fields_to_draw.len(), 3);
        assert!(fields_to_draw.contains(&&Field::new(40, 20, RED, 0)));
        assert!(fields_to_draw.contains(&&Field::new(30, 15, RED, 0)));
        assert!(fields_to_draw.contains(&&Field::new(100, 0, RED, 0)));
    }

    #[test]
    fn get_fields_to_draw_empty() {
        let mc: MapController = MapController::new(200, 400, 10);

        let fields_to_draw: Vec<&Field> = mc.get_fields_to_draw();

        assert_eq!(fields_to_draw.len(), 0);
    }

    // #[test]
    // fn is_game_over() {
    //     let mc: MapController = MapController::new(10, 10, 1);
    //     let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);

    //     assert!(!mc.is_game_over(&test_schema, (0, 0)));
    //     assert!(!mc.is_game_over(&test_schema, (0, 1)));
    //     assert!(mc.is_game_over(&test_schema, (0, -2 * 1)));
    // }

    // #[test]
    // fn spawn_block() {
    //     let mc: MapController = MapController::new(10, 10, 1);
    //     let test_schema: Vec<(i32, i32)> = Vec::from([(0, 0)]);

    //     mc.spawn_block(test_schema, WHITE);

    //     assert_eq!(mc.get_field(0, 0).unwrap().get_color(), WHITE);
    //     assert_eq!(mc.get_field(1, 1).unwrap().get_color(), WHITE);
    // }
}
