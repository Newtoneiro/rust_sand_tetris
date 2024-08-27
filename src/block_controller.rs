use crate::{
    block::{Block, BlockType},
    constants::{
        block_constants::{BLOCK_CHUNK_SIDE, BLOCK_STARTING_POS},
        colors::{BLUE, GREEN, RED, YELLOW},
    },
    map::{ColisionType, Map},
};
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
        self.block_queue
            .push_front(BlockController::generate_random_block());
        self.color_queue
            .push_front(BlockController::generate_random_color());
        self.block_center_pos = BLOCK_STARTING_POS;
    }

    fn generate_random_block() -> Block {
        let block_type = match rand::thread_rng().gen_range(0..=7) {
            1 => BlockType::LBlock,
            2 => BlockType::RevLBlock,
            3 => BlockType::SquareBlock,
            4 => BlockType::ZBlock,
            5 => BlockType::RevZBlock,
            6 => BlockType::TBlock,
            _ => BlockType::IBlock,
        };

        Block::new(block_type)
    }

    fn generate_random_color() -> Color {
        match rand::thread_rng().gen_range(0..=4) {
            0 => RED,
            1 => BLUE,
            2 => GREEN,
            _ => YELLOW,
        }
    }

    fn check_game_over_and_settle_if(&mut self, map: &mut Map, settle_condition: bool) -> bool {
        let game_over: bool = map.is_game_over(
            &self.get_current_block().get_schema(),
            self.block_center_pos,
        );
        if game_over {
            return true;
        }
        if settle_condition {
            self.settle_block(map);
        }
        false
    }

    pub fn handle_move_down(&mut self, map: &mut Map) -> bool {
        let (can_move, _) = self.move_down(map);
        if !can_move {
            return self.check_game_over_and_settle_if(map, true);
        }
        false
    }

    fn move_down(&mut self, map: &mut Map) -> (bool, ColisionType) {
        let (can_move, colision) = map.can_move_down(
            &self.get_current_block().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.block_center_pos.1 += 1;
        }

        (can_move, colision)
    }

    pub fn handle_move_right(&mut self, map: &mut Map) -> bool {
        let (can_move, colision) = self.move_right(map);
        if !can_move {
            return self.check_game_over_and_settle_if(map, colision == ColisionType::SandColision);
        }
        false
    }

    fn move_right(&mut self, map: &mut Map) -> (bool, ColisionType) {
        let (can_move, colision) = map.can_move_right(
            &self.get_current_block().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.block_center_pos.0 += 1;
        }

        (can_move, colision)
    }

    pub fn handle_move_left(&mut self, map: &mut Map) -> bool {
        let (can_move, colision) = self.move_left(map);
        if !can_move {
            return self.check_game_over_and_settle_if(map, colision == ColisionType::SandColision);
        }
        false
    }

    fn move_left(&mut self, map: &mut Map) -> (bool, ColisionType) {
        let (can_move, colision) = map.can_move_left(
            &self.get_current_block().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.block_center_pos.0 -= 1;
        }

        (can_move, colision)
    }

    pub fn handle_rotate_clockwise(&mut self, map: &mut Map) -> bool {
        let (can_move, colision) = self.rotate_clockwise(map);
        if !can_move {
            return self.check_game_over_and_settle_if(map, colision == ColisionType::SandColision);
        }
        false
    }

    fn rotate_clockwise(&mut self, map: &Map) -> (bool, ColisionType) {
        let (can_move, colision) = map.can_rotate(
            &self.get_current_block_rotated_clockwise().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.get_current_block_mut().rotate_clockwise();
        }

        (can_move, colision)
    }

    pub fn handle_rotate_counter_clockwise(&mut self, map: &mut Map) -> bool {
        let (can_move, colision) = self.rotate_counter_clockwise(map);
        if !can_move {
            return self.check_game_over_and_settle_if(map, colision == ColisionType::SandColision);
        }
        false
    }

    fn rotate_counter_clockwise(&mut self, map: &Map) -> (bool, ColisionType) {
        let (can_move, colision) = map.can_rotate(
            &self.get_current_block_rotated_c_clockwise().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.get_current_block_mut().rotate_counter_clockwise();
        }

        (can_move, colision)
    }

    fn get_current_block(&self) -> &Block {
        self.block_queue.get(0).unwrap()
    }

    fn get_current_color(&self) -> &Color {
        self.color_queue.get(0).unwrap()
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

    pub fn tick_and_check_game_over(&mut self, map: &mut Map) -> bool {
        self.handle_move_down(map)
    }

    pub fn settle_block(&mut self, map: &mut Map) {
        let drawing_schema_color: (Vec<(i32, i32)>, Color) = self.get_block_to_draw();
        map.spawn_block(drawing_schema_color.0, drawing_schema_color.1);
        self.get_new_block();
    }

    pub fn clear(&mut self) {
        self.block_queue.clear();
        self.color_queue.clear();
        self.get_new_block();
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
        let mut map: Map = Map::new(200, 400);
        let starting_pos = BLOCK_STARTING_POS;

        bc.get_new_block();
        bc.move_down(&mut map);

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
        let mut map: Map = Map::new(200, 400);
        let starting_pos = BLOCK_STARTING_POS;

        bc.get_new_block();
        bc.move_right(&mut map);

        assert_eq!(bc.block_center_pos, (starting_pos.0 + 1, starting_pos.1));
    }

    #[test]
    fn block_controller_move_left() {
        let mut bc: BlockController = BlockController::new();
        let mut map: Map = Map::new(200, 400);
        let starting_pos = BLOCK_STARTING_POS;

        bc.get_new_block();
        bc.move_left(&mut map);

        assert_eq!(bc.block_center_pos, (starting_pos.0 - 1, starting_pos.1));
    }
}
