use bounded_vec_deque::BoundedVecDeque;
use macroquad::color::Color;

use crate::{
    constants::TetrisConstants,
    controllers::map_controller::{ColisionType, MapController},
    objects::block::Block,
    utils::tetris_rng::TetrisRng,
};

pub struct BlockController<'a> {
    block_center_pos: (i32, i32),
    block_queue: BoundedVecDeque<Block>,
    color_queue: BoundedVecDeque<Color>,
    constants: &'a TetrisConstants,
}

impl<'a> BlockController<'a> {
    pub fn new(constants: &'a TetrisConstants) -> Self {
        BlockController {
            block_center_pos: constants.block_starting_pos,
            block_queue: BoundedVecDeque::new(2),
            color_queue: BoundedVecDeque::new(2),
            constants,
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

        self.block_center_pos = self.constants.block_starting_pos;
    }

    fn generate_random_block(rng: &mut impl TetrisRng) -> Block {
        let block_type = rng.generate_block_type();

        Block::new(block_type)
    }

    fn generate_random_color(rng: &mut impl TetrisRng) -> Color {
        let color = rng.generate_block_color();

        color
    }

    fn check_game_over(&mut self, mc: &mut MapController) -> bool {
        mc.is_game_over(
            &self.get_current_block().get_schema(),
            self.block_center_pos,
        )
    }

    fn settle_and_get_new_block(&mut self, mc: &mut MapController, rng: &mut impl TetrisRng) {
        self.settle_block(mc);
        self.get_new_block(rng)
    }

    pub fn handle_move_down(&mut self, mc: &mut MapController, rng: &mut impl TetrisRng) -> bool {
        let (can_move, _) = self.move_down(mc);
        if !can_move {
            match self.check_game_over(mc) {
                true => return true,
                false => self.settle_and_get_new_block(mc, rng),
            }
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
            match self.check_game_over(mc) {
                true => return true,
                false => {
                    if colision == ColisionType::SandColision {
                        self.settle_and_get_new_block(mc, rng)
                    }
                }
            }
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
            match self.check_game_over(mc) {
                true => return true,
                false => {
                    if colision == ColisionType::SandColision {
                        self.settle_and_get_new_block(mc, rng)
                    }
                }
            }
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
            match self.check_game_over(mc) {
                true => return true,
                false => {
                    if colision == ColisionType::SandColision {
                        self.settle_and_get_new_block(mc, rng)
                    }
                }
            }
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
            match self.check_game_over(mc) {
                true => return true,
                false => {
                    if colision == ColisionType::SandColision {
                        self.settle_and_get_new_block(mc, rng)
                    }
                }
            }
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
                self.block_center_pos.0 + block_box.0 as i32 * self.constants.block_chunk_side,
                self.block_center_pos.1 + block_box.1 as i32 * self.constants.block_chunk_side,
            ))
        }
        (output, color)
    }

    pub fn get_next_block_miniature(&self) -> (Vec<(i32, i32)>, Color) {
        let mut output: Vec<(i32, i32)> = Vec::new();
        let color: Color = self.get_next_color().clone();

        for block_box in self.get_next_block().get_schema() {
            output.push((
                block_box.0 as i32 * self.constants.preview_block_chunk_side,
                block_box.1 as i32 * self.constants.preview_block_chunk_side,
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
    use crate::{
        constants::{
            block_schemas::SQUARE_BLOCK,
            colors::{RED, WHITE, YELLOW},
        },
        objects::{block::BlockType, field::Field},
        utils::tetris_rng::{MockTetrisRng, ThreadTetrisRng},
    };

    use super::*;

    const TEST_CONSTANTS: TetrisConstants = TetrisConstants {
        map_width: 10,
        map_height: 10,
        block_chunk_side: 1,
        grain_side_size: 1,
        preview_block_chunk_side: 1,
        block_starting_pos: (5, 0),
    };

    #[test]
    fn create_block_controller() {
        let bc: BlockController = BlockController::new(&TEST_CONSTANTS);

        assert_eq!(bc.block_center_pos, TEST_CONSTANTS.block_starting_pos);
        assert_eq!(bc.block_queue.len(), 0);
        assert_eq!(bc.color_queue.len(), 0);
    }

    #[test]
    fn init_block_queue() {
        let mut bc: BlockController = BlockController::new(&TEST_CONSTANTS);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();

        bc.init_block_queue(&mut rng);

        assert_eq!(bc.block_center_pos, TEST_CONSTANTS.block_starting_pos);
        assert_eq!(bc.block_queue.len(), 2);
        assert_eq!(bc.color_queue.len(), 2);
    }

    #[test]
    fn get_new_block() {
        let mut bc: BlockController = BlockController::new(&TEST_CONSTANTS);
        let mut rng: MockTetrisRng = MockTetrisRng::new();
        rng.set_block_type(BlockType::SquareBlock);
        rng.set_block_color(RED);

        bc.get_new_block(&mut rng);
        assert_eq!(bc.block_queue.len(), 1);
        assert_eq!(bc.block_queue.get(0).unwrap().get_schema(), SQUARE_BLOCK);
        assert_eq!(bc.color_queue.len(), 1);
        assert_eq!(*bc.color_queue.get(0).unwrap(), RED);
        assert_eq!(bc.block_center_pos, TEST_CONSTANTS.block_starting_pos);

        bc.get_new_block(&mut rng);
        assert_eq!(bc.block_queue.len(), 2);
        assert_eq!(bc.color_queue.len(), 2);
        assert_eq!(bc.block_center_pos, TEST_CONSTANTS.block_starting_pos);

        bc.get_new_block(&mut rng);
        assert_eq!(bc.block_queue.len(), 2);
        assert_eq!(bc.color_queue.len(), 2);
        assert_eq!(bc.block_center_pos, TEST_CONSTANTS.block_starting_pos);
    }

    #[test]
    fn generate_random_block() {
        let mut rng: MockTetrisRng = MockTetrisRng::new();
        rng.set_block_type(BlockType::SquareBlock);

        let new_block: Block = BlockController::generate_random_block(&mut rng);

        assert_eq!(new_block.get_schema(), SQUARE_BLOCK);
    }

    #[test]
    fn generate_random_color() {
        let mut rng: MockTetrisRng = MockTetrisRng::new();
        rng.set_block_color(YELLOW);

        let color: Color = BlockController::generate_random_color(&mut rng);

        assert_eq!(color, YELLOW);
    }

    #[test]
    fn check_game_over_true() {
        let mut bc: BlockController = BlockController::new(&TEST_CONSTANTS);
        let mut mc: MapController = MapController::new(&TEST_CONSTANTS);
        let mut rng: MockTetrisRng = MockTetrisRng::new();

        rng.set_block_type(BlockType::SquareBlock);
        bc.init_block_queue(&mut rng);

        bc.block_center_pos = (0, -1);

        assert!(bc.check_game_over(&mut mc));
    }

    #[test]
    fn check_game_over_false() {
        let mut bc: BlockController = BlockController::new(&TEST_CONSTANTS);
        let mut mc: MapController = MapController::new(&TEST_CONSTANTS);
        let mut rng: MockTetrisRng = MockTetrisRng::new();

        rng.set_block_type(BlockType::SquareBlock);
        bc.init_block_queue(&mut rng);

        bc.block_center_pos = (1, 1);

        assert!(!bc.check_game_over(&mut mc));
    }

    #[test]
    fn settle_and_get_new_block() {
        let mut bc: BlockController = BlockController::new(&TEST_CONSTANTS);
        let mut mc: MapController = MapController::new(&TEST_CONSTANTS);
        let mut rng: MockTetrisRng = MockTetrisRng::new();

        rng.set_block_type(BlockType::SquareBlock);
        rng.set_block_color(WHITE); // for Graphics Controller
        bc.init_block_queue(&mut rng);

        bc.settle_and_get_new_block(&mut mc, &mut rng);

        assert_eq!(bc.block_queue.len(), 2);
        assert_eq!(bc.color_queue.len(), 2);

        let settled_fields: Vec<Field> = mc
            .get_fields_to_draw()
            .iter()
            .map(|f| (*f).clone())
            .collect();

        for (x, y) in [(5, 0), (6, 0), (5, 1), (6, 1)] {
            assert!(settled_fields.contains(&Field::new(x, y, WHITE, 1)))
        }
    }

    #[test]
    fn block_controller_move_down() {
        let mut bc: BlockController = BlockController::new(&TEST_CONSTANTS);
        let mut mc: MapController = MapController::new(&TEST_CONSTANTS);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        let starting_pos = TEST_CONSTANTS.block_starting_pos;

        bc.init_block_queue(&mut rng);
        bc.move_down(&mut mc);

        assert_eq!(bc.block_center_pos, (starting_pos.0, starting_pos.1 + 1));
    }

    #[test]
    fn block_controller_move_right() {
        let mut bc: BlockController = BlockController::new(&TEST_CONSTANTS);
        let mut mc: MapController = MapController::new(&TEST_CONSTANTS);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        let starting_pos = TEST_CONSTANTS.block_starting_pos;

        bc.init_block_queue(&mut rng);
        bc.move_right(&mut mc);

        assert_eq!(bc.block_center_pos, (starting_pos.0 + 1, starting_pos.1));
    }

    #[test]
    fn block_controller_move_left() {
        let mut bc: BlockController = BlockController::new(&TEST_CONSTANTS);
        let mut mc: MapController = MapController::new(&TEST_CONSTANTS);
        let mut rng: ThreadTetrisRng = ThreadTetrisRng::new();
        let starting_pos = TEST_CONSTANTS.block_starting_pos;

        bc.init_block_queue(&mut rng);
        bc.move_left(&mut mc);

        assert_eq!(bc.block_center_pos, (starting_pos.0 - 1, starting_pos.1));
    }
}
