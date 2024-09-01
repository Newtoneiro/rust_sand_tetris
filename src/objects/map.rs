use macroquad::color::Color;
use std::collections::VecDeque;

use crate::{
    constants::colors::BACKGROUND_COLOR, controllers::graphic_controller::GraphicController,
    objects::field::Field, utils::tetris_rng::TetrisRng,
};

pub struct Map {
    width: i32,
    height: i32,
    grid: Vec<Vec<Field>>,
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

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
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

    pub fn get_field_group_id(&self, x: i32, y: i32) -> Option<u32> {
        if !self.check_coords_in_bounds(x, y) {
            return None;
        }
        Some(self.get_field(x, y).unwrap().get_group_id())
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

    pub fn filter_fields(&self, func: impl Fn(&Field) -> bool) -> Vec<&Field> {
        let mut output: Vec<&Field> = Vec::new();
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                let field: &Field = self.get_field(x, y).unwrap();
                if func(&field) {
                    output.push(&field)
                }
            }
        }

        output
    }

    pub fn tick_and_get_score_fields(&mut self, rng: &mut impl TetrisRng) -> Vec<(i32, i32)> {
        for (x, y) in self.get_fields_coords_bottom_up(rng) {
            if !self.get_field(x, y).unwrap().is_empty() {
                let (new_x, new_y) = self.get_new_pos(x, y, rng);
                if (new_x, new_y) != (x, y) {
                    let old_field_pos = self.get_field(x, y).unwrap();
                    let field_color = old_field_pos.get_color();
                    let group_id = old_field_pos.get_group_id();

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
                if self.is_valid_neighbour(old_pos, (neighbour.0, neighbour.1)) {
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

    fn is_valid_neighbour(&self, pos: (i32, i32), neighbour_pos: (i32, i32)) -> bool {
        let field = self.get_field(pos.0, pos.1).unwrap();
        let neighbour_field = self.get_field(neighbour_pos.0, neighbour_pos.1).unwrap();

        neighbour_field.get_group_id() != 0                                         // Field has a group
            && field.get_group_id() != neighbour_field.get_group_id()               // The group of neighbour is not the same
            && GraphicController::normalize_color(field.get_color())
                == GraphicController::normalize_color(neighbour_field.get_color())
        // The color family is the same
    }

    fn change_group_bfs(&mut self, x: i32, y: i32, new_group_id: u32) {
        if self.get_group_size(self.get_field_group_id(x, y).unwrap())
            > self.get_group_size(new_group_id)
        {
            return (); // We don't want to change bigger groups
        }

        let mut checked = Vec::new();
        let mut queue = VecDeque::from([(x, y)]);
        while queue.len() > 0 {
            let (cur_x, cur_y) = queue.pop_back().unwrap();
            for neighbour in self.get_field_neighbors(cur_x, cur_y) {
                self.grid[cur_y as usize][cur_x as usize].set_group_id(new_group_id);
                checked.push((cur_x, cur_y));
                if !checked.contains(&neighbour)
                    && self.is_valid_neighbour((cur_x, cur_y), neighbour)
                {
                    queue.push_back(neighbour);
                }
            }
        }
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
            if group_ids.contains(&self.get_field_group_id(0, y).unwrap()) {
                touches_left_wall = true;
            }
            if group_ids.contains(&self.get_field_group_id(self.width - 1, y).unwrap()) {
                touches_right_wall = true;
            }
        }

        touches_left_wall && touches_right_wall
    }

    pub fn get_fields_for_groups(&mut self, group_ids: &Vec<u32>) -> Vec<(i32, i32)> {
        let output: Vec<(i32, i32)> = self
            .filter_fields(|field: &Field| group_ids.contains(&field.get_group_id()))
            .into_iter()
            .map(|field: &Field| -> (i32, i32) { (field.get_x(), field.get_y()) })
            .collect();

        output
    }

    fn get_group_size(&mut self, group_id: u32) -> usize {
        let output = self
            .filter_fields(|field: &Field| field.get_group_id() == group_id)
            .len();

        output
    }

    fn get_field_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let output: Vec<(i32, i32)> = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
            (1, 1),
        ]
        .iter()
        .filter(|(n_x, n_y)| self.check_coords_in_bounds(x + n_x, y + n_y))
        .map(|(n_x, n_y)| (x + n_x, y + n_y))
        .collect();

        output
    }

    fn get_fields_coords_bottom_up(&self, rng: &mut impl TetrisRng) -> Vec<(i32, i32)> {
        let mut output = Vec::new();

        for y in (0..self.get_height()).rev() {
            for x in rng.get_random_row_order(self.width) {
                output.push((x, y));
            }
        }

        output
    }

    fn get_new_pos(&self, x: i32, y: i32, rng: &mut impl TetrisRng) -> (i32, i32) {
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
                    let go_right: bool = rng.gen_do_go_right();
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

    pub fn clear(&mut self) {
        self.grid = Map::create_grid(self.width, self.height);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        constants::colors::{BLUE, RED, YELLOW, YELLOW_DARK},
        utils::tetris_rng::{MockTetrisRng, ThreadTetrisRng},
    };

    use super::*;

    #[test]
    fn create_map() {
        let map = Map::new(200, 400);
        let map: Map = map;

        assert_eq!(map.get_width(), 200);
        assert_eq!(map.get_height(), 400);
        assert_eq!(map.grid.len(), 400);
        assert_eq!(map.grid.get(0).unwrap().len(), 200);
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
    fn get_fields_group_id() {
        let mut map: Map = Map::new(200, 400);

        map.change_field(10, 20, RED, 1);

        assert_eq!(map.get_field_group_id(0, 0).unwrap(), 0);
        assert_eq!(map.get_field_group_id(10, 20).unwrap(), 1);
        assert_eq!(map.get_field_group_id(-1, 0), None);
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
    fn map_fields() {
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 0, RED, 1);

        assert_eq!(map.filter_fields(|field: &Field| field.do_draw()).len(), 1);
        assert_eq!(
            map.filter_fields(|field: &Field| field.get_group_id() == 1)
                .len(),
            1
        );
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
        /*
            [Ys]*Y*      Ys -> yellow with the same group
            [Y][Yd][B]   Yd -> Dark yellow
        */
        map.change_field(0, 9, YELLOW, 1);
        map.change_field(1, 9, YELLOW_DARK, 2);
        map.change_field(2, 9, BLUE, 3);
        map.change_field(0, 8, YELLOW, 3);

        map.change_field(1, 8, YELLOW, 3);

        assert!(map.is_valid_neighbour((1, 8), (0, 9)));
        assert!(map.is_valid_neighbour((1, 8), (1, 9)));
        assert!(!map.is_valid_neighbour((1, 8), (2, 9)));
        assert!(!map.is_valid_neighbour((1, 8), (2, 8))); // Empty field
        assert!(!map.is_valid_neighbour((1, 8), (0, 8))); // Same group
    }

    #[test]
    fn change_group_bfs() {
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 9, YELLOW, 1);
        map.change_field(1, 9, YELLOW, 1);
        map.change_field(0, 8, YELLOW, 2);
        map.change_field(0, 7, YELLOW, 2);

        map.change_group_bfs(0, 9, 2);

        assert_eq!(map.get_field_group_id(0, 9).unwrap(), 2);
        assert_eq!(map.get_field_group_id(1, 9).unwrap(), 2);
        assert_eq!(map.get_field_group_id(0, 8).unwrap(), 2);
        assert_eq!(map.get_field_group_id(0, 7).unwrap(), 2);
    }

    #[test]
    fn dont_change_group_bfs_for_bigger_group() {
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 9, YELLOW, 1);
        map.change_field(1, 9, YELLOW, 1);
        map.change_field(2, 9, YELLOW, 1);
        map.change_field(0, 8, YELLOW, 2);
        map.change_field(0, 7, YELLOW, 2);

        map.change_group_bfs(0, 9, 2);

        assert_eq!(map.get_field_group_id(0, 9).unwrap(), 1);
        assert_eq!(map.get_field_group_id(1, 9).unwrap(), 1);
        assert_eq!(map.get_field_group_id(2, 9).unwrap(), 1);
        assert_eq!(map.get_field_group_id(0, 8).unwrap(), 2);
        assert_eq!(map.get_field_group_id(0, 7).unwrap(), 2);
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
            0|#   #
            1|#   #
            2|# # #
              0 1 2
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
            0|#   #
            1|#   #
            2|# # #
              0 1 2
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
    fn get_fields_coords_bottom_up() {
        let map: Map = Map::new(10, 10);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();

        let fields = map.get_fields_coords_bottom_up(&mut rng);

        assert_eq!(fields.len(), 10 * 10);
        for (i, row) in fields.chunks(10).enumerate() {
            assert!(row.iter().all(|(_, y)| *y == 9 - i as i32)); // Rows go in order -> last, last - 1 ... 0
            for x in 0..10 {
                assert!(row.contains(&(x, 9 - i as i32)));
            }
        }
    }

    #[test]
    fn get_new_pos_to_right() {
        let mut map: Map = Map::new(10, 10);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        /*
            6| . ---- Block drops here
            7|
            8|[x]
            9|[x][x][x]
               0  1  2
        */
        map.change_field(0, 9, YELLOW, 1);
        map.change_field(1, 9, YELLOW, 1);
        map.change_field(2, 9, YELLOW, 1);
        map.change_field(0, 8, YELLOW, 1);

        map.change_field(0, 6, YELLOW, 3); // Track this block

        assert_eq!(map.get_new_pos(0, 6, &mut rng), (0, 7),);
        map.tick_and_get_score_fields(&mut rng);
        assert_eq!(map.get_new_pos(0, 7, &mut rng), (1, 8));
        map.tick_and_get_score_fields(&mut rng);
        assert_eq!(map.get_new_pos(1, 8, &mut rng), (1, 8));
    }

    #[test]
    fn get_new_pos_to_left() {
        let mut map: Map = Map::new(10, 10);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        /*
            6|       . ---- Block drops here
            7|         [x]
            8|      [x][x]
            9|[x][x][x][x]
               0  1  2  3
        */
        map.change_field(0, 9, YELLOW, 1);
        map.change_field(1, 9, YELLOW, 1);
        map.change_field(2, 9, YELLOW, 1);
        map.change_field(2, 8, YELLOW, 1);
        map.change_field(3, 9, YELLOW, 1);
        map.change_field(3, 8, YELLOW, 1);
        map.change_field(3, 7, YELLOW, 1);

        map.change_field(2, 6, YELLOW, 3); // Track this block

        assert_eq!(map.get_new_pos(2, 6, &mut rng), (2, 7));
        map.tick_and_get_score_fields(&mut rng);
        assert_eq!(map.get_new_pos(2, 7, &mut rng), (1, 8));
        map.tick_and_get_score_fields(&mut rng);
        assert_eq!(map.get_new_pos(1, 8, &mut rng), (1, 8));
    }

    #[test]
    fn get_new_pos_drop_right() {
        let mut map: Map = Map::new(10, 10);
        let mut rng: MockTetrisRng = MockTetrisRng::new();
        rng.set_go_right(true);

        /*
            7|    . ---- Block drops here
            8|   [x]
            9|[x][x][x]
               0  1  2
        */

        map.change_field(0, 9, YELLOW, 1);
        map.change_field(1, 9, YELLOW, 1);
        map.change_field(2, 9, YELLOW, 1);
        map.change_field(1, 8, YELLOW, 1);

        map.change_field(1, 7, YELLOW, 3); // Track this block

        assert_eq!(map.get_new_pos(1, 7, &mut rng), (2, 8));
    }

    #[test]
    fn get_new_pos_drop_left() {
        let mut map: Map = Map::new(10, 10);
        let mut rng: MockTetrisRng = MockTetrisRng::new();
        rng.set_go_right(false);

        /*
            7|    . ---- Block drops here
            8|   [x]
            9|[x][x][x]
               0  1  2
        */

        map.change_field(0, 9, YELLOW, 1);
        map.change_field(1, 9, YELLOW, 1);
        map.change_field(2, 9, YELLOW, 1);
        map.change_field(1, 8, YELLOW, 1);

        map.change_field(1, 7, YELLOW, 3); // Track this block

        assert_eq!(map.get_new_pos(1, 7, &mut rng), (0, 8));
    }

    #[test]
    fn get_new_pos_stuck() {
        let mut map: Map = Map::new(10, 10);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        /*
            7|    . ---- Block drops here
            8|[x]   [x]
            9|[x][x][x]
               0  1  2
        */
        map.change_field(0, 9, YELLOW, 1);
        map.change_field(1, 9, YELLOW, 1);
        map.change_field(2, 9, YELLOW, 1);
        map.change_field(0, 8, YELLOW, 1);
        map.change_field(2, 8, YELLOW, 1);

        map.change_field(1, 7, YELLOW, 3); // Track this block

        assert_eq!(map.get_new_pos(1, 7, &mut rng), (1, 8));
        map.tick_and_get_score_fields(&mut rng);
        assert_eq!(map.get_new_pos(1, 8, &mut rng), (1, 8));
    }

    #[test]
    fn get_new_pos_both_sides_touch_wall() {
        let mut map: Map = Map::new(1, 10);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        map.change_field(0, 9, YELLOW, 1);
        /*
            7| . | --- Block drops here
            8|   |
            9|[x]|
               0
        */

        map.change_field(0, 7, YELLOW, 2); // Track this block

        assert_eq!(map.get_new_pos(0, 7, &mut rng), (0, 8));
        map.tick_and_get_score_fields(&mut rng);
        assert_eq!(map.get_new_pos(0, 8, &mut rng), (0, 8));
    }

    #[test]
    fn grains_move_down() {
        let mut map: Map = Map::new(200, 400);

        map.change_field(40, 20, RED, 0);

        map.tick_and_get_score_fields(&mut ThreadTetrisRng::new());

        assert_eq!(map.filter_fields(|field: &Field| field.do_draw()).len(), 1);
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

        map.tick_and_get_score_fields(&mut ThreadTetrisRng::new());

        assert_eq!(map.filter_fields(|field: &Field| field.do_draw()).len(), 2);
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

        map.tick_and_get_score_fields(&mut ThreadTetrisRng::new());

        assert_eq!(map.filter_fields(|field: &Field| field.do_draw()).len(), 2);
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

        map.tick_and_get_score_fields(&mut ThreadTetrisRng::new());

        assert_eq!(map.filter_fields(|field: &Field| field.do_draw()).len(), 4);
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

        map.tick_and_get_score_fields(&mut ThreadTetrisRng::new());

        assert_eq!(map.filter_fields(|field: &Field| field.do_draw()).len(), 2);
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

        map.tick_and_get_score_fields(&mut ThreadTetrisRng::new());

        assert_eq!(map.filter_fields(|field: &Field| field.do_draw()).len(), 4);
        assert_eq!(map.get_field(8, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(8, 8).unwrap().get_color(), BACKGROUND_COLOR);
        assert_eq!(map.get_field(9, 9).unwrap().get_color(), RED);
        assert_eq!(map.get_field(9, 8).unwrap().get_color(), RED);
        assert_eq!(map.get_field(7, 9).unwrap().get_color(), RED);
    }

    #[test]
    fn clear() {
        let mut map: Map = Map::new(10, 10);

        map.change_field(0, 0, RED, 0);

        map.clear();

        assert_eq!(map.get_field(0, 0).unwrap().get_color(), BACKGROUND_COLOR);
    }
}
