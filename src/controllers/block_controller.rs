use bounded_vec_deque::BoundedVecDeque;
use macroquad::color::Color;

use crate::{
    constants::block_constants::{BLOCK_CHUNK_SIDE, BLOCK_STARTING_POS, PREVIEW_BLOCK_CHUNK_SIDE},
    controllers::map_controller::{ColisionType, MapController},
    objects::block::Block,
    utils::tetris_rng::TetrisRng,
};

pub struct BlockController {
    block_center_pos: (i32, i32),
    block_queue: BoundedVecDeque<Block>,
    color_queue: BoundedVecDeque<Color>,
}

impl BlockController {
    pub fn new() -> Self {
        BlockController {
            block_center_pos: BLOCK_STARTING_POS,
            block_queue: BoundedVecDeque::new(2),
            color_queue: BoundedVecDeque::new(2),
        }
    }

    pub fn init_block_queue(&mut self, rng: &mut impl TetrisRng) {
        self.block_queue.clear();
        self.color_queue.clear();
        self.get_new_block(rng); // Current
        self.get_new_block(rng); // Previous
    }

    fn get_new_block(&mut self, rng: &mut impl TetrisRng) {
        self.block_queue
            .push_front(BlockController::generate_random_block(rng));
        self.color_queue
            .push_front(BlockController::generate_random_color(rng));

        self.block_center_pos = BLOCK_STARTING_POS;
    }

    fn generate_random_block(rng: &mut impl TetrisRng) -> Block {
        let block_type = rng.generate_block_type();

        Block::new(block_type)
    }

    fn generate_random_color(rng: &mut impl TetrisRng) -> Color {
        let color = rng.generate_block_color();

        color
    }

    fn check_game_over_and_settle_if(
        &mut self,
        mc: &mut MapController,
        settle_condition: bool,
        rng: &mut impl TetrisRng,
    ) -> bool {
        let game_over: bool = mc.is_game_over(
            &self.get_current_block().get_schema(),
            self.block_center_pos,
        );
        if game_over {
            return true;
        }
        if settle_condition {
            self.settle_block(mc);
            self.get_new_block(rng)
        }
        false
    }

    pub fn handle_move_down(&mut self, mc: &mut MapController, rng: &mut impl TetrisRng) -> bool {
        let (can_move, _) = self.move_down(mc);
        if !can_move {
            return self.check_game_over_and_settle_if(mc, true, rng);
        }
        false
    }

    fn move_down(&mut self, mc: &mut MapController) -> (bool, ColisionType) {
        let (can_move, colision) = mc.can_block_move_down(
            &self.get_current_block().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.block_center_pos.1 += 1;
        }

        (can_move, colision)
    }

    pub fn handle_move_right(&mut self, mc: &mut MapController, rng: &mut impl TetrisRng) -> bool {
        let (can_move, colision) = self.move_right(mc);
        if !can_move {
            return self.check_game_over_and_settle_if(
                mc,
                colision == ColisionType::SandColision,
                rng,
            );
        }
        false
    }

    fn move_right(&mut self, mc: &mut MapController) -> (bool, ColisionType) {
        let (can_move, colision) = mc.can_block_move_right(
            &self.get_current_block().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.block_center_pos.0 += 1;
        }

        (can_move, colision)
    }

    pub fn handle_move_left(&mut self, mc: &mut MapController, rng: &mut impl TetrisRng) -> bool {
        let (can_move, colision) = self.move_left(mc);
        if !can_move {
            return self.check_game_over_and_settle_if(
                mc,
                colision == ColisionType::SandColision,
                rng,
            );
        }
        false
    }

    fn move_left(&mut self, mc: &mut MapController) -> (bool, ColisionType) {
        let (can_move, colision) = mc.can_block_move_left(
            &self.get_current_block().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.block_center_pos.0 -= 1;
        }

        (can_move, colision)
    }

    pub fn handle_rotate_clockwise(
        &mut self,
        mc: &mut MapController,
        rng: &mut impl TetrisRng,
    ) -> bool {
        let (can_move, colision) = self.rotate_clockwise(mc);
        if !can_move {
            return self.check_game_over_and_settle_if(
                mc,
                colision == ColisionType::SandColision,
                rng,
            );
        }
        false
    }

    fn rotate_clockwise(&mut self, mc: &MapController) -> (bool, ColisionType) {
        let (can_move, colision) = mc.can_block_rotate(
            &self.get_current_block_rotated_clockwise().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.get_current_block_mut().rotate_clockwise();
        }

        (can_move, colision)
    }

    pub fn handle_rotate_counter_clockwise(
        &mut self,
        mc: &mut MapController,
        rng: &mut impl TetrisRng,
    ) -> bool {
        let (can_move, colision) = self.rotate_counter_clockwise(mc);
        if !can_move {
            return self.check_game_over_and_settle_if(
                mc,
                colision == ColisionType::SandColision,
                rng,
            );
        }
        false
    }

    fn rotate_counter_clockwise(&mut self, mc: &MapController) -> (bool, ColisionType) {
        let (can_move, colision) = mc.can_block_rotate(
            &self.get_current_block_rotated_c_clockwise().get_schema(),
            self.block_center_pos,
        );
        if can_move {
            self.get_current_block_mut().rotate_counter_clockwise();
        }

        (can_move, colision)
    }

    fn get_current_block(&self) -> &Block {
        self.block_queue.get(1).unwrap()
    }

    fn get_current_color(&self) -> &Color {
        self.color_queue.get(1).unwrap()
    }

    fn get_next_block(&self) -> &Block {
        self.block_queue.get(0).unwrap()
    }

    fn get_next_color(&self) -> &Color {
        self.color_queue.get(0).unwrap()
    }

    fn get_current_block_mut(&mut self) -> &mut Block {
        self.block_queue.get_mut(1).unwrap()
    }

    fn get_current_block_rotated_clockwise(&self) -> Block {
        let mut rotated_block = (*self.block_queue.get(1).unwrap()).clone();
        rotated_block.rotate_clockwise();

        return rotated_block;
    }

    fn get_current_block_rotated_c_clockwise(&self) -> Block {
        let mut rotated_block = (*self.block_queue.get(1).unwrap()).clone();
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

    pub fn get_next_block_miniature(&self) -> (Vec<(i32, i32)>, Color) {
        let mut output: Vec<(i32, i32)> = Vec::new();
        let color: Color = self.get_next_color().clone();

        for block_box in self.get_next_block().get_schema() {
            output.push((
                block_box.0 as i32 * PREVIEW_BLOCK_CHUNK_SIDE,
                block_box.1 as i32 * PREVIEW_BLOCK_CHUNK_SIDE,
            ))
        }
        (output, color)
    }

    pub fn tick_and_check_game_over(
        &mut self,
        mc: &mut MapController,
        rng: &mut impl TetrisRng,
    ) -> bool {
        self.handle_move_down(mc, rng)
    }

    pub fn settle_block(&mut self, mc: &mut MapController) {
        let drawing_schema_color: (Vec<(i32, i32)>, Color) = self.get_block_to_draw();
        mc.spawn_block(drawing_schema_color.0, drawing_schema_color.1);
    }

    pub fn clear(&mut self, rng: &mut impl TetrisRng) {
        self.init_block_queue(rng);
    }
}

#[cfg(test)]
mod test {
    use crate::utils::tetris_rng::ThreadTetrisRng;

    use super::*;

    #[test]
    fn create_block_controller() {
        let bc: BlockController = BlockController::new();

        assert_eq!(bc.block_queue.len(), 0);
        assert_eq!(bc.color_queue.len(), 0);
        assert_eq!(bc.block_center_pos, BLOCK_STARTING_POS);
    }

    #[test]
    fn init_block_queue() {
        let mut bc: BlockController = BlockController::new();
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();

        bc.block_center_pos = (0, 0);
        assert_eq!(bc.block_center_pos, (0, 0));

        bc.init_block_queue(&mut rng);
        assert_eq!(bc.block_queue.len(), 2);
        assert_eq!(bc.color_queue.len(), 2);
        assert_eq!(bc.block_center_pos, BLOCK_STARTING_POS);
    }

    #[test]
    fn get_new_block() {
        let mut bc: BlockController = BlockController::new();
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();

        bc.block_center_pos = (0, 0);
        assert_eq!(bc.block_center_pos, (0, 0));

        bc.get_new_block(&mut rng);
        assert_eq!(bc.block_queue.len(), 1);
        assert_eq!(bc.color_queue.len(), 1);
        assert_eq!(bc.block_center_pos, BLOCK_STARTING_POS);
        bc.get_new_block(&mut rng);
        assert_eq!(bc.block_queue.len(), 2);
        assert_eq!(bc.color_queue.len(), 2);
        assert_eq!(bc.block_center_pos, BLOCK_STARTING_POS);
        bc.get_new_block(&mut rng);
        assert_eq!(bc.block_queue.len(), 2);
        assert_eq!(bc.color_queue.len(), 2);
        assert_eq!(bc.block_center_pos, BLOCK_STARTING_POS);
    }

    #[test]
    fn block_controller_move_down() {
        let mut bc: BlockController = BlockController::new();
        let mut mc: MapController = MapController::new(200, 400, 10);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        let starting_pos = BLOCK_STARTING_POS;

        bc.init_block_queue(&mut rng);
        bc.move_down(&mut mc);

        assert_eq!(bc.block_center_pos, (starting_pos.0, starting_pos.1 + 1));
    }

    #[test]
    fn block_controller_move_right() {
        let mut bc: BlockController = BlockController::new();
        let mut mc: MapController = MapController::new(200, 400, 10);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        let starting_pos = BLOCK_STARTING_POS;

        bc.init_block_queue(&mut rng);
        bc.move_right(&mut mc);

        assert_eq!(bc.block_center_pos, (starting_pos.0 + 1, starting_pos.1));
    }

    #[test]
    fn block_controller_move_left() {
        let mut bc: BlockController = BlockController::new();
        let mut mc: MapController = MapController::new(200, 400, 10);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        let starting_pos = BLOCK_STARTING_POS;

        bc.init_block_queue(&mut rng);
        bc.move_left(&mut mc);

        assert_eq!(bc.block_center_pos, (starting_pos.0 - 1, starting_pos.1));
    }
}
