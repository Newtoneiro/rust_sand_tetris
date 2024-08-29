use macroquad::color::Color;
use std::collections::VecDeque;
use rand::{
    seq::SliceRandom,
    thread_rng,
    Rng,
};

use crate::{
    constants::{
        block_constants::BLOCK_CHUNK_SIDE,
        colors::BACKGROUND_COLOR,
    },
    controllers::graphic_controller::GraphicController,
    objects::field::Field,
};

pub struct Map {
    width: i32,
    height: i32,
    grid: Vec<Vec<Field>>,
    current_group_id: u32,
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
            current_group_id: 1,
        }
    }

    fn create_grid(width: i32, height: i32) -> Vec<Vec<Field>> {
        let mut grid: Vec<Vec<Field>> = Vec::new();
        for y in 0..height {
            grid.push(Vec::new());
            for x in 0..width {
                grid[y as usize].push(Field::new(x, y, BACKGROUND_COLOR, 0));
            }
        }
        grid
    }

    pub fn get_field(&self, x: i32, y: i32) -> Option<&Field> {
        if !self.check_coords_in_bounds(x, y) {
            return None;
        }
        Some(&self.grid[y as usize][x as usize])
    }

    pub fn change_field(&mut self, x: i32, y: i32, new_color: Color, new_group_id: u32) {
        if !self.check_coords_in_bounds(x, y) {
            return ();
        }
        self.grid[y as usize][x as usize].set_color(new_color);
        self.grid[y as usize][x as usize].set_group_id(new_group_id);
    }

    fn check_coords_in_bounds(&self, x: i32, y: i32) -> bool {
        if (0 <= x && x < self.width) && (0 <= y && y < self.height) {
            return true;
        }
        false
    }

    pub fn get_fields_to_draw(&self) -> Vec<&Field> {
        let mut output: Vec<&Field> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_field(x, y).unwrap().do_draw() {
                    output.push(self.get_field(x, y).unwrap())
                }
            }
        }
        output
    }

    pub fn tick_and_get_score_fields(&mut self) -> Vec<(i32, i32)> {
        for y in (0..self.height).rev() {
            for x in self.get_random_row_order() {
                if !self.get_field(x, y).unwrap().is_empty() {
                    let (new_x, new_y) = self.get_new_pos(x, y);
                    if (new_x, new_y) != (x, y) {
                        let old_pos_field = self.get_field(x, y).unwrap();
                        let field_color = old_pos_field.get_color();
                        let group_id = old_pos_field.get_group_id();

                        let new_group_id = self.get_new_group((new_x, new_y), (x, y), group_id);

                        self.change_field(new_x, new_y, field_color, new_group_id);
                        self.change_field(x, y, BACKGROUND_COLOR, 0);

                        let groups = Vec::from([new_group_id, group_id]); // Combine them because not every group from the block has yet been converted

                        let fields_for_demolishion = self.get_fields_for_demolishion(&groups);
                        if fields_for_demolishion.len() > 0 {
                            return fields_for_demolishion;
                        }
                    }
                }
            }
        }

        Vec::new()
    }

    fn get_new_group(
        &mut self,
        new_pos: (i32, i32),
        old_pos: (i32, i32),
        current_group: u32,
    ) -> u32 {
        let mut adjacent_groups = self.get_adjacent_groups(new_pos, old_pos);
        adjacent_groups.retain(|&group| group != current_group);

        let new_group = match adjacent_groups.len() {
            1 => {
                self.change_group_bfs(old_pos.0, old_pos.1, adjacent_groups[0]);
                adjacent_groups[0]
            }
            _ => current_group,
        };

        new_group
    }

    fn get_adjacent_groups(&mut self, new_pos: (i32, i32), old_pos: (i32, i32)) -> Vec<u32> {
        let mut output = Vec::new();
        for neighbour in self.get_field_neighbors(new_pos.0, new_pos.1) {
            if self.check_coords_in_bounds(neighbour.0, neighbour.1) {
                if self.is_valid_neighbour(new_pos, old_pos, (neighbour.0, neighbour.1)) {
                    let neighbour_group_id = self
                        .get_field(neighbour.0, neighbour.1)
                        .unwrap()
                        .get_group_id();
                    if !output.contains(&neighbour_group_id) {
                        output.push(neighbour_group_id);
                    }
                }
            }
        }
        output
    }

    fn is_valid_neighbour(
        &self,
        new_pos: (i32, i32),
        old_pos: (i32, i32),
        neighbour_pos: (i32, i32),
    ) -> bool {
        let new_field = self.get_field(new_pos.0, new_pos.1).unwrap();
        let old_field = self.get_field(old_pos.0, old_pos.1).unwrap();
        let neighbour_field = self.get_field(neighbour_pos.0, neighbour_pos.1).unwrap();

        neighbour_field.get_group_id() != 0
            && new_field.get_group_id() != neighbour_field.get_group_id()
            && GraphicController::normalize_color(old_field.get_color())
                == GraphicController::normalize_color(neighbour_field.get_color())
    }

    fn change_group_bfs(&mut self, x: i32, y: i32, new_group_id: u32) {
        if self.get_group_size(self.get_field(x, y).unwrap().get_group_id())
            > self.get_group_size(new_group_id)
        {
            return ();
        }

        let mut checked = Vec::new();
        let mut queue = VecDeque::from([(x, y)]);
        while queue.len() > 0 {
            let (cur_x, cur_y) = queue.pop_back().unwrap();
            for neighbour in self.get_field_neighbors(cur_x, cur_y) {
                self.grid[cur_y as usize][cur_x as usize].set_group_id(new_group_id);
                checked.push((cur_x, cur_y));
                if !checked.contains(&neighbour)
                    && self.is_valid_neighbour_for_bfs((cur_x, cur_y), neighbour, new_group_id)
                {
                    queue.push_back(neighbour);
                }
            }
        }
    }

    fn is_valid_neighbour_for_bfs(
        &self,
        pos: (i32, i32),
        neighbour_pos: (i32, i32),
        new_group_id: u32,
    ) -> bool {
        let field = self.get_field(pos.0, pos.1).unwrap();
        let neighbour_field = self.get_field(neighbour_pos.0, neighbour_pos.1).unwrap();

        neighbour_field.get_group_id() != 0
            && neighbour_field.get_group_id() != new_group_id
            && GraphicController::normalize_color(field.get_color())
                == GraphicController::normalize_color(neighbour_field.get_color())
    }

    fn get_fields_for_demolishion(&mut self, group_ids: &Vec<u32>) -> Vec<(i32, i32)> {
        if self.is_row_complete(group_ids) {
            return self.get_fields_for_groups(group_ids);
        }
        Vec::new()
    }

    fn is_row_complete(&self, group_ids: &Vec<u32>) -> bool {
        let mut touches_left_wall = false;
        let mut touches_right_wall = false;

        for y in 0..self.height {
            if group_ids.contains(&self.get_field(0, y).unwrap().get_group_id()) {
                touches_left_wall = true;
            }
            if group_ids.contains(&self.get_field(self.width - 1, y).unwrap().get_group_id()) {
                touches_right_wall = true;
            }
        }

        touches_left_wall && touches_right_wall
    }

    pub fn demolish_fields(&mut self, fields_coords: &Vec<(i32, i32)>) {
        for (x, y) in fields_coords {
            self.change_field(*x, *y, BACKGROUND_COLOR, 0);
        }
    }

    fn get_fields_for_groups(&mut self, group_ids: &Vec<u32>) -> Vec<(i32, i32)> {
        let mut output = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if group_ids.contains(&self.get_field(x, y).unwrap().get_group_id()) {
                    output.push((x, y));
                }
            }
        }

        output
    }

    fn get_group_size(&mut self, group_id: u32) -> usize {
        let mut group_size: usize = 0;
        for y in 0..self.height {
            group_size += Vec::from(self.grid[y as usize].clone())
                .iter()
                .filter(|field| field.get_group_id() == group_id)
                .count();
        }
        group_size
    }

    fn get_field_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut output = Vec::new();
        for (n_x, n_y) in [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
            (1, 1),
        ] {
            if self.check_coords_in_bounds(x + n_x, y + n_y) {
                output.push((x + n_x, y + n_y));
            }
        }
        output
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

        bottom_border > self.height
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

        right_border > self.width
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
            self.change_field(x, y, color, self.current_group_id);
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
        self.current_group_id = 1;
    }
}

#[cfg(test)]
mod test {
    use crate::constants::colors::{
        BLUE, RED, YELLOW, YELLOW_DARK
    };

    use super::*;

    #[test]
    fn create_map() {
        let map = Map::new(200, 400);
        let map: Map = map;

        assert_eq!(map.width, 200);
        assert_eq!(map.height, 400);
        assert_eq!(map.grid.len(), 400);
        assert_eq!(map.grid.get(0).unwrap().len(), 200);
        assert_eq!(map.current_group_id, 1);
    }

    #[test]
    fn create_grid() {
        let grid = Map::create_grid(10, 10);

        assert_eq!(grid.len(), 10);
        assert_eq!(grid.get(0).unwrap().len(), 10);

        for (x, y) in [(0, 0), (2, 5), (9, 9)] {
            let field = grid.get(y).unwrap().get(x).unwrap();
            assert_eq!(field.get_x(), x as i32);
            assert_eq!(field.get_y(), y as i32);
            assert_eq!(field.get_color(), BACKGROUND_COLOR);
            assert_eq!(field.get_group_id(), 0);
        }
    }

    #[test]
    fn get_field() {
        let map: Map = Map::new(200, 400);

        let field_middle: &Field = map.get_field(20, 40).unwrap();
        let field_min: &Field = map.get_field(0, 0).unwrap();
        let field_max: &Field = map.get_field(199, 399).unwrap();

        assert_eq!(field_middle.get_x(), 20);
        assert_eq!(field_middle.get_y(), 40);
        assert_eq!(field_middle.get_color(), BACKGROUND_COLOR);
        assert_eq!(field_middle.get_group_id(), 0);
        assert_eq!(field_min.get_x(), 0);
        assert_eq!(field_min.get_y(), 0);
        assert_eq!(field_max.get_x(), 199);
        assert_eq!(field_max.get_y(), 399);
    }

    #[test]
    fn get_field_out_of_bounds() {
        let map: Map = Map::new(200, 400);

        let field1: Option<&Field> = map.get_field(-1, 40);
        let field2: Option<&Field> = map.get_field(0, 400);
        
        assert!(field1.is_none());
        assert!(field2.is_none());
    }

    #[test]
    fn change_field() {
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 0, RED, 2);
        map.change_field(9, 6, RED, 3);

        let field1: &Field = map.get_field(0, 0).unwrap();
        let field2: &Field = map.get_field(9, 6).unwrap();

        assert_eq!(field1.get_color(), RED);
        assert_eq!(field1.get_group_id(), 2);
        assert_eq!(field2.get_color(), RED);
        assert_eq!(field2.get_group_id(), 3);
    }

    #[test]
    fn change_field_out_of_bounds() {
        let mut map: Map = Map::new(200, 400);

        map.change_field(-1, 0, RED, 2);
        map.change_field(0, 400, RED, 3);

        // changing field out of bounds has no effect.
    }

    #[test]
    fn check_coords_in_bounds() {
        let map: Map = Map::new(10, 10);

        for (x, y) in [(0, 0), (0, 9), (9, 0)] {
            assert!(map.check_coords_in_bounds(x, y));
        }

        for (x, y) in [(-1, 0), (0, 10), (10, 0), (11, 11)] {
            assert!(!map.check_coords_in_bounds(x, y));
        }
    }

    #[test]
    fn get_fields_to_draw() {
        let mut map: Map = Map::new(200, 400);
        map.change_field(40, 20, RED, 0);
        map.change_field(30, 15, RED, 0);
        map.change_field(100, 0, RED, 0);

        let fields_to_draw: Vec<&Field> = map.get_fields_to_draw();

        assert_eq!(fields_to_draw.len(), 3);
        assert!(fields_to_draw.contains(&&Field::new(40, 20, RED, 0)));
        assert!(fields_to_draw.contains(&&Field::new(30, 15, RED, 0)));
        assert!(fields_to_draw.contains(&&Field::new(100, 0, RED, 0)));
    }

    #[test]
    fn get_fields_to_draw_empty() {
        let map: Map = Map::new(200, 400);

        let fields_to_draw: Vec<&Field> = map.get_fields_to_draw();

        assert_eq!(fields_to_draw.len(), 0);
    }

    #[test]
    fn get_new_group_horizontaly() {
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 0, RED, 1);
        map.change_field(0, 1, RED, 2);
        let new_group = map.get_new_group((0, 1), (0, 1), 2);

        assert_eq!(new_group, 1);
    }
    
    #[test]
    fn get_new_group_diagonaly() {
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 0, RED, 1); // static Field
        map.change_field(1, 1, RED, 2); // adjacent Field for get_new_group (moved from (1, 2))
        let new_group = map.get_new_group((1, 1), (1, 1), 2);

        assert_eq!(new_group, 1);
    }

    #[test]
    fn get_new_group_no_change() {
        // No change when arround multiple groups
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 0, RED, 1);
        map.change_field(0, 2, RED, 2);
        map.change_field(1, 2, RED, 3);
        map.change_field(0, 1, RED, 4);
        let new_group = map.get_new_group((0, 1), (0, 1), 2);

        assert_eq!(new_group, 2);
    }

    #[test]
    fn get_adjacent_groups() {
        let mut map: Map = Map::new(10, 10);
        map.change_field(0, 0, YELLOW, 1);
        map.change_field(2, 0, YELLOW, 2);
        map.change_field(0, 1, BLUE, 3);
        
        map.change_field(1, 0, YELLOW, 4);

        let groups = map.get_adjacent_groups((1, 0), (1, 0));
        assert_eq!(groups.len(), 2);
        assert!(groups.contains(&1));
        assert!(groups.contains(&2));
    }

    #[test]
    fn is_valid_neighbour() {
        let mut map: Map = Map::new(10, 10);
        /*  [Ys]*Y*      Ys -> yellow with the same group
            [Y][Yd][B]   Yd -> Dark yellow
         */
        map.change_field(0, 0, YELLOW, 1);
        map.change_field(1, 0, YELLOW_DARK, 2);
        map.change_field(2, 0, BLUE, 3);
        map.change_field(0, 1, YELLOW, 3);

        map.change_field(1, 1, YELLOW, 3);

        assert!(map.is_valid_neighbour((1, 1), (1, 1), (0, 0)));
        assert!(map.is_valid_neighbour((1, 1), (1, 1), (1, 0)));
        assert!(!map.is_valid_neighbour((1, 1), (1, 1), (2, 0)));
        assert!(!map.is_valid_neighbour((1, 1), (1, 1), (2, 1))); // Empty field
        assert!(!map.is_valid_neighbour((1, 1), (1, 1), (0, 1))); // Same group
    }

    #[test]
    fn change_group_bfs() {
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 0, YELLOW, 1);
        map.change_field(1, 0, YELLOW, 1);
        map.change_field(0, 1, YELLOW, 2);
        map.change_field(0, 2, YELLOW, 2);

        map.change_group_bfs(0, 0, 2);

        assert_eq!(map.get_field(0, 0).unwrap().get_group_id(), 2);
        assert_eq!(map.get_field(1, 0).unwrap().get_group_id(), 2);
        assert_eq!(map.get_field(0, 1).unwrap().get_group_id(), 2);
        assert_eq!(map.get_field(0, 2).unwrap().get_group_id(), 2);
    }

    #[test]
    fn dont_change_group_bfs_for_bigger_group() {
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 0, YELLOW, 1);
        map.change_field(1, 0, YELLOW, 1);
        map.change_field(2, 0, YELLOW, 1);
        map.change_field(0, 1, YELLOW, 2);
        map.change_field(0, 2, YELLOW, 2);

        map.change_group_bfs(0, 0, 2);

        assert_eq!(map.get_field(0, 0).unwrap().get_group_id(), 1);
        assert_eq!(map.get_field(1, 0).unwrap().get_group_id(), 1);
        assert_eq!(map.get_field(2, 0).unwrap().get_group_id(), 1);
        assert_eq!(map.get_field(0, 1).unwrap().get_group_id(), 2);
        assert_eq!(map.get_field(0, 2).unwrap().get_group_id(), 2);
    }

    #[test]
    fn is_valid_neighbour_for_bfs() {
        let mut map: Map = Map::new(10, 10);
        /*  [Ys]*Y*      Ys -> yellow with the same group
            [Y][Yd][B]   Yd -> Dark yellow
         */
        map.change_field(0, 0, YELLOW, 1);
        map.change_field(1, 0, YELLOW_DARK, 2);
        map.change_field(2, 0, BLUE, 3);
        map.change_field(0, 1, YELLOW, 3);

        map.change_field(1, 1, YELLOW, 3);

        assert!(map.is_valid_neighbour_for_bfs((1, 1), (0, 0), 3));
        assert!(map.is_valid_neighbour_for_bfs((1, 1), (1, 0), 3));
        assert!(!map.is_valid_neighbour_for_bfs((1, 1), (2, 0), 3));
        assert!(!map.is_valid_neighbour_for_bfs((1, 1), (0, 1), 3)); // Same group
        assert!(!map.is_valid_neighbour_for_bfs((1, 1), (2, 1), 3)); // Empty field
    }

    #[test]
    fn get_fields_for_demolishion() {
        let mut map: Map = Map::new(10, 10);

        for x in 0..10 {
            map.change_field(x, 0, YELLOW, 1);
        }

        let fields_for_demolishion = map.get_fields_for_demolishion(&Vec::from([1]));
        assert_eq!(fields_for_demolishion.len(), 10);
        for x in 0..10 {
            assert!(fields_for_demolishion.contains(&(x, 0)));
        }
    }

    #[test]
    fn get_fields_for_demolishion_more_complicated() {
        let mut map: Map = Map::new(3, 3);
        /*
            # # #
            #   #
            #   #
         */

        map.change_field(0, 0, YELLOW, 1);
        map.change_field(0, 1, YELLOW, 1);
        map.change_field(0, 2, YELLOW, 1);
        map.change_field(1, 2, YELLOW, 1);
        map.change_field(2, 0, YELLOW, 1);
        map.change_field(2, 1, YELLOW, 1);
        map.change_field(2, 2, YELLOW, 1);

        let fields_for_demolishion = map.get_fields_for_demolishion(&Vec::from([1]));
        assert_eq!(fields_for_demolishion.len(), 7);
    }

    #[test]
    fn get_fields_for_demolishion_empty() {
        let mut map: Map = Map::new(10, 10);

        for x in 0..9 {
            map.change_field(x, 0, YELLOW, 1);
        }

        let fields_for_demolishion = map.get_fields_for_demolishion(&Vec::from([1]));
        assert_eq!(fields_for_demolishion.len(), 0);
    }

    #[test]
    fn is_row_complete() {
        let mut map: Map = Map::new(10, 10);

        for x in 0..10 {
            map.change_field(x, 0, YELLOW, 1);
        }
        for x in 0..9 {
            map.change_field(x, 1, YELLOW, 2);
        }

        assert!(map.is_row_complete(&Vec::from([1])));
        assert!(!map.is_row_complete(&Vec::from([2])));
    }

    #[test]
    fn is_row_complete_complicated() {
        let mut map: Map = Map::new(3, 3);
        /*
            # # #
            #   #
            #   #
         */

        map.change_field(0, 0, YELLOW, 1);
        map.change_field(0, 1, YELLOW, 1);
        map.change_field(0, 2, YELLOW, 1);
        map.change_field(1, 2, YELLOW, 1);
        map.change_field(2, 0, YELLOW, 1);
        map.change_field(2, 1, YELLOW, 1);
        map.change_field(2, 2, YELLOW, 1);

        assert!(map.is_row_complete(&Vec::from([1])))
    }

    #[test]
    fn demolish_fields() {
        let mut map: Map = Map::new(10, 10);

        for x in 0..10 {
            map.change_field(x, 0, YELLOW, 1);
        }
        let fields = map.get_fields_for_groups(&Vec::from([1]));
        map.demolish_fields(&fields);

        for (x, y) in fields {
            assert_eq!(map.get_field(x, y).unwrap().get_color(), BACKGROUND_COLOR);
            assert_eq!(map.get_field(x, y).unwrap().get_group_id(), 0);
        }
    }

    #[test]
    fn get_fields_for_groups() {
        let mut map: Map = Map::new(10, 10);

        for x in 0..10 {
            map.change_field(x, 0, YELLOW, 1);
        }
        let fields = map.get_fields_for_groups(&Vec::from([1]));
        let fields_empty = map.get_fields_for_groups(&Vec::from([2]));
        
        assert_eq!(fields.len(), 10);
        for x in 0..10 {
            assert!(fields.contains(&(x, 0)));
        }
        assert_eq!(fields_empty.len(), 0);
    }

    #[test]
    fn get_group_size() {
        let mut map: Map = Map::new(10, 10);

        for x in 0..10 {
            map.change_field(x, 0, YELLOW, 1);
        }

        assert_eq!(map.get_group_size(1), 10);
        assert_eq!(map.get_group_size(2), 0);
    }

    #[test]
    fn get_field_neighbors() {
        let map: Map = Map::new(10, 10);

        let neighbors = map.get_field_neighbors(1, 1);
        assert_eq!(neighbors.len(), 8);
        assert!(neighbors.contains(&(0, 0)));
        assert!(neighbors.contains(&(1, 0)));
        assert!(neighbors.contains(&(2, 0)));
        assert!(neighbors.contains(&(0, 1)));
        assert!(neighbors.contains(&(2, 1)));
        assert!(neighbors.contains(&(0, 2)));
        assert!(neighbors.contains(&(1, 2)));
        assert!(neighbors.contains(&(2, 2)));
    }

    #[test]
    fn get_field_neighbors_corner() {
        let map: Map = Map::new(10, 10);

        let neighbors = map.get_field_neighbors(0, 0);
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&(1, 0)));
        assert!(neighbors.contains(&(1, 1)));
        assert!(neighbors.contains(&(0, 1)));
    }

    #[test]
    fn get_random_row_order() {
        let map: Map = Map::new(10, 10);

        let order = map.get_random_row_order();

        assert_eq!(order.len(), 10);
        for i in 0..10 {
            assert!(order.contains(&i));
        }
    }

    #[test]
    fn grains_move_down() {
        let mut map: Map = Map::new(200, 400);
        map.change_field(40, 20, RED, 0);

        map.tick_and_get_score_fields();

        assert_eq!(map.get_fields_to_draw().len(), 1);
        assert_eq!(map.get_field(40, 20).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(40, 21).unwrap().get_color(), RED);
    }

    #[test]
    fn grains_move_down_to_side() {
        //    #      ->
        //    #      ->     ##

        let mut map: Map = Map::new(10, 10);
        map.change_field(5, 9, RED, 0); // Grain under
        map.change_field(5, 8, RED, 0); // Grain above

        map.tick_and_get_score_fields();

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
        map.change_field(0, 9, RED, 0); // Grain under
        map.change_field(0, 8, RED, 0); // Grain above

        map.tick_and_get_score_fields();

        assert_eq!(map.get_fields_to_draw().len(), 2);
        assert_eq!(map.get_field(0, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(0, 8).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(1, 9).unwrap().get_color(), RED);
    }

    #[test]
    fn grains_move_down_to_side_left_blocked() {
        let mut map: Map = Map::new(10, 10);
        map.change_field(1, 9, RED, 0); // Grain under
        map.change_field(1, 8, RED, 0); // Grain above
        map.change_field(0, 9, RED, 0); // left Grain above
        map.change_field(0, 8, RED, 0); // left Grain above

        map.tick_and_get_score_fields();

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
        map.change_field(9, 9, RED, 0); // Grain under
        map.change_field(9, 8, RED, 0); // Grain above

        map.tick_and_get_score_fields();

        assert_eq!(map.get_fields_to_draw().len(), 2);
        assert_eq!(map.get_field(9, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(9, 8).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(8, 9).unwrap().get_color(), RED);
    }

    #[test]
    fn grains_move_down_to_side_right_blocked() {
        let mut map: Map = Map::new(10, 10);
        map.change_field(8, 9, RED, 0); // Grain under
        map.change_field(8, 8, RED, 0); // Grain above
        map.change_field(9, 9, RED, 0); // right Grain above
        map.change_field(9, 8, RED, 0); // right Grain above

        map.tick_and_get_score_fields();

        assert_eq!(map.get_fields_to_draw().len(), 4);
        assert_eq!(map.get_field(8, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(8, 8).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(9, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(9, 8).unwrap().get_color(), RED);
        assert_eq!(map.get_field(7, 9).unwrap().get_color(), RED);
    }
}
