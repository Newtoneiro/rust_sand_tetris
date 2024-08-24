use crate::{block::{Block, BlockType}, constants::{block_constants::{BLOCK_CHUNK_SIDE, BLOCK_STARTING_POS}, colors::RED, map_constants::{MAP_HEIGHT, MAP_WIDTH}}, map::Map};
use bounded_vec_deque::BoundedVecDeque;
use macroquad::color::Color;
use rand::Rng;

pub struct BlockController {
    block_center_pos: (i32, i32),
    block_queue: BoundedVecDeque<Block>,
    color_queue: BoundedVecDeque<Color>,
}

impl BlockController {
    pub fn new() -> BlockController {
        BlockController {
            block_center_pos: BLOCK_STARTING_POS,
            block_queue: BoundedVecDeque::new(3),
            color_queue: BoundedVecDeque::new(3),
        }
    }

    pub fn get_new_block(&mut self) {
        self.block_queue.push_front(
            BlockController::generate_random_block()
        );
        self.color_queue.push_front(
            BlockController::generate_random_color()
        );
        self.block_center_pos = BLOCK_STARTING_POS;
    }

    fn generate_random_block() -> Block {
        let block_type = match rand::thread_rng().gen_range(0..=1) {
            _ => BlockType::LBlock,
        };

        Block::new(block_type)
    }

    fn generate_random_color() -> Color {
        match rand::thread_rng().gen_range(0..=1) {
            _ => RED,
        }
    }

    pub fn move_down(&mut self, map: &Map) -> bool {
        if map.can_move_down(&self.get_current_block().get_schema(), self.block_center_pos) {
            self.block_center_pos.1 += 1;
            return true
        }

        return false
    }

    pub fn move_right(&mut self, map: &Map) -> bool {
        if map.can_move_right(&self.get_current_block().get_schema(), self.block_center_pos) {
            self.block_center_pos.0 += 1;
            return true
        }

        return false
    }

    pub fn move_left(&mut self, map: &Map) -> bool {
        if map.can_move_left(&self.get_current_block().get_schema(), self.block_center_pos) {
            self.block_center_pos.0 -= 1;
            return true
        }

        return false
    }

    pub fn rotate_clockwise(&mut self, map: &Map) -> bool {
        if map.can_rotate(&self.get_current_block_rotated_clockwise().get_schema(), self.block_center_pos) {
            self.get_current_block_mut().rotate_clockwise();
            return true
        }

        return false
    }

    pub fn rotate_counter_clockwise(&mut self, map: &Map) -> bool {
        if map.can_rotate(&self.get_current_block_rotated_c_clockwise().get_schema(), self.block_center_pos) {
            self.get_current_block_mut().rotate_counter_clockwise();
            return true
        }

        return false
    }

    fn get_current_block(&self) -> &Block {
        self.block_queue.get(0).unwrap()
    }

    fn get_current_block_mut(&mut self) -> &mut Block {
        self.block_queue.get_mut(0).unwrap()
    }

    fn get_current_block_rotated_clockwise(&self) -> Block {
        let mut rotated_block = (*self.block_queue.get(0).unwrap()).clone();
        rotated_block.rotate_clockwise();
    
        return rotated_block;
    }

    fn get_current_block_rotated_c_clockwise(&self) -> Block {
        let mut rotated_block = (*self.block_queue.get(0).unwrap()).clone();
        rotated_block.rotate_counter_clockwise();
    
        return rotated_block;
    }

    fn get_current_color(&self) -> &Color {
        self.color_queue.get(0).unwrap()
    }

    pub fn get_block_to_draw(&self) -> (Vec<(i32, i32)>, Color) {
        let mut output: Vec<(i32, i32)> = Vec::new();
        let color: Color = self.get_current_color().clone();

        for block_box in self.get_current_block().get_schema() {
            output.push((
                self.block_center_pos.0 + block_box.0 as i32 * BLOCK_CHUNK_SIDE,
                self.block_center_pos.1 + block_box.1 as i32 * BLOCK_CHUNK_SIDE,
            ))
        }
        (output, color)
    }

    pub fn tick(&mut self, map: &mut Map) {
        let block_moving = self.move_down(map);
        if !block_moving {
            map.spawn_block(
                &self.get_current_block().get_schema(),
                self.get_current_color(),
                self.block_center_pos,
            );
            self.get_new_block();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_block_controller() {
        let bc: BlockController = BlockController::new();

        assert_eq!(bc.block_queue.len(), 0);
        assert_eq!(bc.color_queue.len(), 0);
        assert_eq!(bc.block_center_pos, BLOCK_STARTING_POS);
    }

    #[test]
    fn block_controller_move_down() {
        let mut bc: BlockController = BlockController::new();
        let map: Map = Map::new(200, 400);
        let starting_pos = BLOCK_STARTING_POS;

        bc.move_down(&map);

        assert_eq!(bc.block_center_pos, (starting_pos.0, starting_pos.1 + 1));
    }

    #[test]
    fn get_new_block() {
        let mut bc: BlockController = BlockController::new();

        bc.get_new_block();
        assert_eq!(bc.block_queue.len(), 1);
        bc.get_new_block();
        assert_eq!(bc.block_queue.len(), 2);
        bc.get_new_block();
        assert_eq!(bc.block_queue.len(), 3);
        bc.get_new_block();
        assert_eq!(bc.block_queue.len(), 3);
    }

    #[test]
    fn block_controller_move_right() {
        let mut bc: BlockController = BlockController::new();
        let map: Map = Map::new(200, 400);
        let starting_pos = BLOCK_STARTING_POS;

        bc.move_right(&map);

        assert_eq!(bc.block_center_pos, (starting_pos.0 + 1, starting_pos.1));
    }

    #[test]
    fn block_controller_move_left() {
        let mut bc: BlockController = BlockController::new();
        let map: Map = Map::new(200, 400);
        let starting_pos = BLOCK_STARTING_POS;

        bc.move_left(&map);

        assert_eq!(bc.block_center_pos, (starting_pos.0 - 1, starting_pos.1));
    }
}