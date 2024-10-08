use macroquad::color::Color;

use crate::{
    constants::{colors::BACKGROUND_COLOR, TetrisConstants},
    controllers::graphic_controller::GraphicController,
    objects::{field::Field, map::Map},
    utils::tetris_rng::TetrisRng,
};

pub struct MapController<'a> {
    map: Map,
    current_group_id: u32,
    constants: &'a TetrisConstants,
}

#[derive(PartialEq, Debug)]
pub enum ColisionType {
    BorderColision,
    SandColision,
    NoColision,
}

impl<'a> MapController<'a> {
    pub fn new(constants: &'a TetrisConstants) -> Self {
        let map = Map::new(constants.map_width, constants.map_height);

        MapController {
            map,
            current_group_id: 1,
            constants,
        }
    }

    pub fn is_game_over(&self, schema: &Vec<(i8, i8)>, center_pos: (i32, i32)) -> bool {
        self.is_block_coliding_upper_border(schema, center_pos)
    }

    pub fn clear(&mut self) {
        self.current_group_id = 1;
        self.map.clear();
    }

    pub fn tick_and_get_score_fields(&mut self, rng: &mut impl TetrisRng) -> Vec<(i32, i32)> {
        self.map.tick_and_get_score_fields(rng)
    }

    pub fn demolish_fields(&mut self, fields: &Vec<(i32, i32)>) {
        for (x, y) in fields {
            self.map.change_field(*x, *y, BACKGROUND_COLOR, 0);
        }
    }

    pub fn get_fields_to_draw(&self) -> Vec<&Field> {
        self.map.filter_fields(|field: &Field| field.do_draw())
    }

    pub fn get_shuffled_fields(
        &self,
        fields_coords: &Vec<(i32, i32)>,
        rng: &mut impl TetrisRng,
    ) -> Vec<&Field> {
        let mut fields_to_demolish = Vec::new();
        for (x, y) in fields_coords {
            fields_to_demolish.push(self.map.get_field(*x, *y).unwrap());
        }
        rng.shuffle_fields(&mut fields_to_demolish);

        fields_to_demolish
    }

    pub fn can_block_move_down(
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

    pub fn can_block_move_left(
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

    pub fn can_block_move_right(
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

    pub fn can_block_rotate(
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
        self.is_block_coliding_upper_border(schema, center_pos)
            || self.is_block_coliding_bottom_border(schema, center_pos)
            || self.is_block_coliding_left_border(schema, center_pos)
            || self.is_block_coliding_right_border(schema, center_pos)
    }

    fn is_block_coliding_upper_border(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> bool {
        let upper_most_box: (i8, i8) =
            *schema.iter().min_by_key(|schema_box| schema_box.1).unwrap();
        let upper_border =
            center_pos.1 + (upper_most_box.1 as i32) * self.constants.block_chunk_side;

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
        let bottom_border =
            center_pos.1 + (bottom_most_box.1 as i32 + 1) * self.constants.block_chunk_side;

        bottom_border + 1 > self.map.get_height()
    }

    fn is_block_coliding_left_border(
        &self,
        schema: &Vec<(i8, i8)>,
        center_pos: (i32, i32),
    ) -> bool {
        let left_most_box: (i8, i8) = *schema
            .into_iter()
            .min_by_key(|schema_box| schema_box.0)
            .unwrap();

        let left_border = center_pos.0 + left_most_box.0 as i32 * self.constants.block_chunk_side;

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

        let right_border =
            center_pos.0 + (right_most_box.0 as i32 + 1) * self.constants.block_chunk_side;

        right_border + 1 > self.map.get_width()
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
        for (x, y, color) in
            GraphicController::get_skin_for_schema(schema, color, self.constants.block_chunk_side)
        {
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
            for x_offset in 0..self.constants.block_chunk_side {
                for y_offset in 0..self.constants.block_chunk_side {
                    let x: i32 =
                        center_pos.0 + (block_box.0 as i32 * self.constants.block_chunk_side);
                    let y: i32 =
                        center_pos.1 + (block_box.1 as i32 * self.constants.block_chunk_side);
                    output.push((x + x_offset, y + y_offset));
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod test {
    use crate::{
        constants::colors::{BACKGROUND_COLOR, RED, WHITE},
        utils::tetris_rng::ThreadTetrisRng,
    };

    use super::*;

    fn get_test_constants(width: i32, height: i32) -> TetrisConstants {
        TetrisConstants {
            map_width: width,
            map_height: height,
            block_chunk_side: 1,
            grain_side_size: 1,
            preview_block_chunk_side: 1,
            block_starting_pos: (5, 0),
        }
    }

    #[test]
    fn is_game_over() {
        let constants = get_test_constants(3, 3);
        let mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);
        /*     3
            0| 1       |
            1|         |
            2|    2    |
               0  1  2
        */

        assert!(!mc.is_game_over(&test_schema, (0, 0)));
        assert!(!mc.is_game_over(&test_schema, (1, 2)));
        assert!(mc.is_game_over(&test_schema, (0, -1)));
    }

    #[test]
    fn clear() {
        let constants = get_test_constants(3, 3);
        let mut mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i32, i32)> = Vec::from([(0, 0)]);

        mc.current_group_id = 10;
        mc.spawn_block(test_schema, RED);

        mc.clear();

        assert_eq!(mc.current_group_id, 1);
        assert_eq!(
            mc.map.get_field(0, 0).unwrap().get_color(),
            BACKGROUND_COLOR
        );
    }

    #[test]
    fn tick_and_get_score_fields() {
        let constants = get_test_constants(4, 4);
        let mut mc: MapController = MapController::new(&constants);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();

        mc.map.change_field(0, 3, RED, 1);
        mc.map.change_field(1, 3, RED, 1);
        mc.map.change_field(2, 2, RED, 1);
        mc.map.change_field(3, 3, RED, 1);
        /*
            0|            |
            1|            |
            2|      [x]   |
            3|[x][x]   [x]|
               0  1  2  3
        */

        let score_fields = mc.tick_and_get_score_fields(&mut rng);

        assert_eq!(score_fields.len(), 4);
        for score_point in [(0, 3), (1, 3), (2, 3), (3, 3)] {
            assert!(score_fields.contains(&score_point));
        }
    }

    #[test]
    fn demolish_fields() {
        let constants = get_test_constants(10, 10);
        let mut mc: MapController = MapController::new(&constants);

        for x in 0..10 {
            mc.map.change_field(x, 0, RED, 1);
        }
        let fields = mc.map.get_fields_for_groups(&Vec::from([1]));
        mc.demolish_fields(&fields);

        for (x, y) in fields {
            assert_eq!(
                mc.map.get_field(x, y).unwrap().get_color(),
                BACKGROUND_COLOR
            );
            assert_eq!(mc.map.get_field_group_id(x, y).unwrap(), 0);
        }
    }

    #[test]
    fn get_fields_to_draw() {
        let constants = get_test_constants(200, 400);
        let mut mc: MapController = MapController::new(&constants);
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
        let constants = get_test_constants(200, 400);
        let mc: MapController = MapController::new(&constants);

        let fields_to_draw: Vec<&Field> = mc.get_fields_to_draw();

        assert_eq!(fields_to_draw.len(), 0);
    }

    #[test]
    fn get_shuffled_fields() {
        let constants = get_test_constants(10, 10);
        let mc: MapController = MapController::new(&constants);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();

        let fields_coords: Vec<(i32, i32)> = Vec::from([(0, 0), (5, 5), (9, 9)]);

        let shuffled_fields = mc.get_shuffled_fields(&fields_coords, &mut rng);
        assert_eq!(shuffled_fields.len(), 3);
        for (x, y) in fields_coords {
            assert!(shuffled_fields.contains(&mc.map.get_field(x, y).unwrap()));
        }
    }

    #[test]
    fn can_block_move_down() {
        let constants = get_test_constants(4, 4);
        let mut mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);
        mc.map.change_field(0, 3, RED, 1);
        mc.map.change_field(0, 2, RED, 1);
        /*
            0|    1       |
            1| 3          |
            2|[x]         |
            3|[x] 2       |
               0  1  2  3
        */

        let (can_move, collision_type) = mc.can_block_move_down(&test_schema, (1, 0)); // 1
        assert!(can_move);
        assert_eq!(collision_type, ColisionType::NoColision);

        let (can_move, collision_type) = mc.can_block_move_down(&test_schema, (1, 3)); // 2
        assert!(!can_move);
        assert_eq!(collision_type, ColisionType::BorderColision);

        let (can_move, collision_type) = mc.can_block_move_down(&test_schema, (0, 1)); // 3
        assert!(!can_move);
        assert_eq!(collision_type, ColisionType::SandColision);
    }

    #[test]
    fn can_block_move_left() {
        let constants = get_test_constants(3, 3);
        let mut mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);
        mc.map.change_field(0, 2, RED, 1);
        /*
            0|    1    |
            1| 2       |
            2|[x] 3    |
               0  1  2
        */

        let (can_move, collision_type) = mc.can_block_move_left(&test_schema, (1, 0)); // 1
        assert!(can_move);
        assert_eq!(collision_type, ColisionType::NoColision);

        let (can_move, collision_type) = mc.can_block_move_left(&test_schema, (0, 1)); // 2
        assert!(!can_move);
        assert_eq!(collision_type, ColisionType::BorderColision);

        let (can_move, collision_type) = mc.can_block_move_left(&test_schema, (1, 2)); // 3
        assert!(!can_move);
        assert_eq!(collision_type, ColisionType::SandColision);
    }

    #[test]
    fn can_block_move_right() {
        let constants = get_test_constants(3, 3);
        let mut mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);
        mc.map.change_field(1, 2, RED, 1);
        /*
            0| 1     3 |
            1|         |
            2| 2 [x]   |
               0  1  2
        */

        let (can_move, collision_type) = mc.can_block_move_right(&test_schema, (0, 0)); // 1
        assert!(can_move);
        assert_eq!(collision_type, ColisionType::NoColision);

        let (can_move, collision_type) = mc.can_block_move_right(&test_schema, (0, 2)); // 2
        assert!(!can_move);
        assert_eq!(collision_type, ColisionType::SandColision);

        let (can_move, collision_type) = mc.can_block_move_right(&test_schema, (2, 0)); // 3
        assert!(!can_move);
        assert_eq!(collision_type, ColisionType::BorderColision);
    }

    #[test]
    fn can_block_rotate() {
        let constants = get_test_constants(4, 4);
        let mut mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0), (1, 0)]);
        mc.map.change_field(2, 3, RED, 1);
        mc.map.change_field(2, 2, RED, 1);
        /*
            0|    1       |
            1|          3 |
            2|    2 [x]   |
            3|      [x]   |
               0  1  2  3
        */

        let (can_move, collision_type) = mc.can_block_rotate(&test_schema, (1, 0)); // 1
        assert!(can_move);
        assert_eq!(collision_type, ColisionType::NoColision);

        let (can_move, collision_type) = mc.can_block_rotate(&test_schema, (1, 2)); // 2
        assert!(!can_move);
        assert_eq!(collision_type, ColisionType::SandColision);

        let (can_move, collision_type) = mc.can_block_rotate(&test_schema, (3, 1)); // 3
        assert!(!can_move);
        assert_eq!(collision_type, ColisionType::BorderColision);
    }

    #[test]
    fn is_block_coliding_with_any_border() {
        let constants = get_test_constants(4, 4);
        let mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);

        assert!(mc.is_block_coliding_with_any_border(&test_schema, (0, -1)));
        assert!(mc.is_block_coliding_with_any_border(&test_schema, (0, 4)));
        assert!(mc.is_block_coliding_with_any_border(&test_schema, (-1, 0)));
        assert!(mc.is_block_coliding_with_any_border(&test_schema, (4, 0)));

        assert!(!mc.is_block_coliding_with_any_border(&test_schema, (0, 0)));
        assert!(!mc.is_block_coliding_with_any_border(&test_schema, (1, 2)));
        assert!(!mc.is_block_coliding_with_any_border(&test_schema, (2, 2)));
    }

    #[test]
    fn is_block_coliding_upper_border() {
        let constants = get_test_constants(4, 4);
        let mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);

        assert!(mc.is_block_coliding_upper_border(&test_schema, (0, -1)));
        assert!(mc.is_block_coliding_upper_border(&test_schema, (3, -1)));

        assert!(!mc.is_block_coliding_upper_border(&test_schema, (0, 0)));
        assert!(!mc.is_block_coliding_upper_border(&test_schema, (3, 0)));
    }

    #[test]
    fn is_block_coliding_right_border() {
        let constants = get_test_constants(4, 4);
        let mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);

        assert!(mc.is_block_coliding_right_border(&test_schema, (3, 0)));
        assert!(mc.is_block_coliding_right_border(&test_schema, (3, 3)));

        assert!(!mc.is_block_coliding_right_border(&test_schema, (2, 0)));
        assert!(!mc.is_block_coliding_right_border(&test_schema, (2, 3)));
    }

    #[test]
    fn is_block_coliding_bottom_border() {
        let constants = get_test_constants(4, 4);
        let mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);

        assert!(mc.is_block_coliding_bottom_border(&test_schema, (0, 3)));
        assert!(mc.is_block_coliding_bottom_border(&test_schema, (3, 3)));

        assert!(!mc.is_block_coliding_bottom_border(&test_schema, (0, 2)));
        assert!(!mc.is_block_coliding_bottom_border(&test_schema, (3, 2)));
    }

    #[test]
    fn is_block_coliding_left_border() {
        let constants = get_test_constants(4, 4);
        let mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);

        assert!(mc.is_block_coliding_left_border(&test_schema, (-1, 0)));
        assert!(mc.is_block_coliding_left_border(&test_schema, (-1, 3)));

        assert!(!mc.is_block_coliding_left_border(&test_schema, (0, 0)));
        assert!(!mc.is_block_coliding_left_border(&test_schema, (0, 3)));
    }

    #[test]
    fn is_block_coliding_with_sand() {
        let constants = get_test_constants(4, 4);
        let mut mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0)]);
        mc.map.change_field(2, 3, RED, 1);
        mc.map.change_field(2, 2, RED, 1);
        /*
            0|            |
            1|       1    |
            2|    2 [3]   |
            3|      [4] 5 |
               0  1  2  3
        */

        assert!(!mc.is_block_coliding_with_sand(&test_schema, (2, 1)));
        assert!(!mc.is_block_coliding_with_sand(&test_schema, (1, 2)));
        assert!(mc.is_block_coliding_with_sand(&test_schema, (2, 2)));
        assert!(mc.is_block_coliding_with_sand(&test_schema, (2, 3)));
        assert!(!mc.is_block_coliding_with_sand(&test_schema, (3, 3)));
    }

    #[test]
    fn spawn_block() {
        let mut constants = get_test_constants(10, 10);
        constants.block_chunk_side = 2;
        let mut mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i32, i32)> = Vec::from([(0, 0), (0, 2)]);

        mc.spawn_block(test_schema, WHITE);

        assert_eq!(mc.current_group_id, 2);
        assert_eq!(mc.map.get_field(0, 0).unwrap().get_color(), WHITE);
        assert_eq!(mc.map.get_field(0, 1).unwrap().get_color(), WHITE);
        assert_eq!(mc.map.get_field(1, 0).unwrap().get_color(), WHITE);
        assert_eq!(mc.map.get_field(1, 1).unwrap().get_color(), WHITE);
        assert_eq!(mc.map.get_field(0, 2).unwrap().get_color(), WHITE);
        assert_eq!(mc.map.get_field(0, 3).unwrap().get_color(), WHITE);
        assert_eq!(mc.map.get_field(1, 2).unwrap().get_color(), WHITE);
        assert_eq!(mc.map.get_field(1, 3).unwrap().get_color(), WHITE);
    }

    #[test]
    fn get_fields_from_schema() {
        let mut constants = get_test_constants(10, 10);
        constants.block_chunk_side = 2;
        let mc: MapController = MapController::new(&constants);
        let test_schema: Vec<(i8, i8)> = Vec::from([(0, 0), (0, 1), (1, 0)]);

        let fields = mc.get_fields_from_schema(&test_schema, (0, 0));

        assert_eq!(fields.len(), 12);
        for expected_field in [
            (0, 0),
            (0, 1),
            (1, 0),
            (1, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
            (2, 0),
            (2, 1),
            (3, 0),
            (3, 1),
        ] {
            assert!(fields.contains(&expected_field));
        }
    }
}
