use crate::{block::{Block, BlockType}, constants::{block_constants::BLOCK_STARTING_POS, map_constants::{MAP_HEIGHT, MAP_WIDTH}}};
use bounded_vec_deque::BoundedVecDeque;
use rand::Rng;

struct BlockController {
    block_queue: BoundedVecDeque<Block>,
    block_center_pos: (i32, i32),
}

impl BlockController {
    pub fn new() -> BlockController {
        BlockController {
            block_queue: BoundedVecDeque::new(3),
            block_center_pos: BLOCK_STARTING_POS,
        }
    }

    fn get_new_block(&mut self) {
        self.block_queue.push_front(
            BlockController::generate_random_block()
        );
    }

    fn generate_random_block() -> Block {
        let block_type = match rand::thread_rng().gen_range(0..=1) {
            _ => BlockType::LBlock,
        };

        Block::new(block_type)
    }

    fn move_down(&mut self) {
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

        self.block_center_pos.1 + bottom_most_box.1 as i32
    }

    fn move_right(&mut self) {
        if self.right_of_cur_block() >= 0 {
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

        self.block_center_pos.0 + right_most_box.0 as i32
    }

    fn move_left(&mut self) {
        if self.left_of_cur_block() <= MAP_HEIGHT {
            self.block_center_pos.0 -= 1
        }
    }

    fn left_of_cur_block(&self) -> i32 {
        if self.block_queue.len() == 0 {
            return self.block_center_pos.1
        }

        let bottom_most_box: (i8, i8) = self.block_queue.get(0).unwrap().get_schema()
            .into_iter()
            .min_by_key(|schema_box| {schema_box.0})
            .unwrap();

        self.block_center_pos.0 - bottom_most_box.0 as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_block_controller() {
        let bc: BlockController = BlockController::new();

        assert_eq!(bc.block_queue.len(), 0);
        assert_eq!(bc.block_center_pos, BLOCK_STARTING_POS);
    }

    #[test]
    fn block_controller_move_down() {
        let mut bc: BlockController = BlockController::new();
        let mut starting_pos = BLOCK_STARTING_POS;

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
        let mut starting_pos = BLOCK_STARTING_POS;

        bc.move_right();

        assert_eq!(bc.block_center_pos, (starting_pos.0 + 1, starting_pos.1));
    }

    #[test]
    fn block_controller_move_left() {
        let mut bc: BlockController = BlockController::new();
        let mut starting_pos = BLOCK_STARTING_POS;

        bc.move_left();

        assert_eq!(bc.block_center_pos, (starting_pos.0 - 1, starting_pos.1));
    }
}