use crate::{block::{Block, BlockType}, constants::{block_constants::{BLOCK_CHUNK_SIDE, BLOCK_STARTING_POS}, colors::RED, map_constants::{MAP_HEIGHT, MAP_WIDTH}}};
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

    pub fn move_down(&mut self) {
        if self.bottom_of_cur_block() <= MAP_HEIGHT {
            self.block_center_pos.1 += 1
        }
    }

    fn bottom_of_cur_block(&self) -> i32 {
        if self.block_queue.len() == 0 {
            return self.block_center_pos.1
        }

        let bottom_most_box: (i8, i8) = self.block_queue.get(0).unwrap().get_schema()
            .into_iter()
            .max_by_key(|schema_box| {schema_box.1})
            .unwrap();

        self.block_center_pos.1 + (bottom_most_box.1 as i32 + 1) * BLOCK_CHUNK_SIDE
    }

    pub fn move_right(&mut self) {
        if self.right_of_cur_block() <= MAP_WIDTH {
            self.block_center_pos.0 += 1
        }
    }

    fn right_of_cur_block(&self) -> i32 {
        if self.block_queue.len() == 0 {
            return self.block_center_pos.1
        }

        let right_most_box: (i8, i8) = self.block_queue.get(0).unwrap().get_schema()
            .into_iter()
            .max_by_key(|schema_box| {schema_box.0})
            .unwrap();

        self.block_center_pos.0 + (right_most_box.0 as i32 + 1) * BLOCK_CHUNK_SIDE
    }

    pub fn move_left(&mut self) {
        if self.left_of_cur_block() >= 0 {
            self.block_center_pos.0 -= 1
        }
    }

    fn left_of_cur_block(&self) -> i32 {
        if self.block_queue.len() == 0 {
            return self.block_center_pos.1
        }

        let bottom_most_box: (i8, i8) = self.get_current_block().get_schema()
            .into_iter()
            .min_by_key(|schema_box| {schema_box.0})
            .unwrap();

        self.block_center_pos.0 + bottom_most_box.0 as i32 * BLOCK_CHUNK_SIDE
    }

    fn get_current_block(&self) -> &Block {
        self.block_queue.get(0).unwrap()
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

    pub fn tick(&mut self) {
        self.move_down();
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
        let starting_pos = BLOCK_STARTING_POS;

        bc.move_down();

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
        let starting_pos = BLOCK_STARTING_POS;

        bc.move_right();

        assert_eq!(bc.block_center_pos, (starting_pos.0 + 1, starting_pos.1));
    }

    #[test]
    fn block_controller_move_left() {
        let mut bc: BlockController = BlockController::new();
        let starting_pos = BLOCK_STARTING_POS;

        bc.move_left();

        assert_eq!(bc.block_center_pos, (starting_pos.0 - 1, starting_pos.1));
    }
}