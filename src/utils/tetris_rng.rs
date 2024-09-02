use macroquad::color::Color;
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    constants::colors::{BLUE, GREEN, RED, YELLOW},
    objects::{block::BlockType, field::Field},
};

pub trait TetrisRng {
    fn gen_do_go_right(&self) -> bool;

    fn get_random_row_order(&self, width: i32) -> Vec<i32>;

    fn shuffle_fields(&self, fields: &mut Vec<&Field>);

    fn generate_block_type(&self) -> BlockType;

    fn generate_block_color(&self) -> Color;
}

pub struct ThreadTetrisRng;

impl ThreadTetrisRng {
    pub fn new() -> ThreadTetrisRng {
        ThreadTetrisRng {}
    }
}

impl TetrisRng for ThreadTetrisRng {
    fn gen_do_go_right(&self) -> bool {
        thread_rng().gen_bool(0.5)
    }

    fn get_random_row_order(&self, width: i32) -> Vec<i32> {
        let mut row_order: Vec<i32> = (0..width).collect();
        row_order.shuffle(&mut thread_rng());

        row_order
    }

    fn shuffle_fields(&self, fields: &mut Vec<&Field>) {
        fields.shuffle(&mut thread_rng());
    }

    fn generate_block_type(&self) -> BlockType {
        let block_type = match thread_rng().gen_range(0..=7) {
            1 => BlockType::LBlock,
            2 => BlockType::RevLBlock,
            3 => BlockType::SquareBlock,
            4 => BlockType::ZBlock,
            5 => BlockType::RevZBlock,
            6 => BlockType::TBlock,
            _ => BlockType::IBlock,
        };

        block_type
    }

    fn generate_block_color(&self) -> Color {
        match thread_rng().gen_range(0..=4) {
            0 => RED,
            1 => BLUE,
            2 => GREEN,
            _ => YELLOW,
        }
    }
}

pub struct MockTetrisRng {
    go_right: bool,
    row_order: Vec<i32>,
    block_type: BlockType,
    block_color: Color,
}

#[warn(dead_code)]
impl MockTetrisRng {
    pub fn new() -> MockTetrisRng {
        MockTetrisRng {
            go_right: false,
            row_order: Vec::new(),
            block_type: BlockType::LBlock,
            block_color: RED,
        }
    }

    pub fn set_go_right(&mut self, go_right: bool) {
        self.go_right = go_right
    }

    pub fn set_random_row_order(&mut self, row_order: Vec<i32>) {
        self.row_order = row_order
    }

    pub fn set_block_type(&mut self, block_type: BlockType) {
        self.block_type = block_type
    }

    pub fn set_block_color(&mut self, block_color: Color) {
        self.block_color = block_color
    }
}

impl TetrisRng for MockTetrisRng {
    fn gen_do_go_right(&self) -> bool {
        self.go_right
    }

    fn get_random_row_order(&self, width: i32) -> Vec<i32> {
        match self.row_order.len() {
            0 => {
                let mut output = Vec::new();
                for i in 0..width {
                    output.push(i);
                }
                output
            }
            _ => self.row_order.clone(),
        }
    }

    fn shuffle_fields(&self, _fields: &mut Vec<&Field>) {}

    fn generate_block_type(&self) -> BlockType {
        self.block_type.clone()
    }

    fn generate_block_color(&self) -> Color {
        self.block_color.clone()
    }
}

#[cfg(test)]
mod test_mock_tetris_rng {
    use super::*;

    #[test]
    fn create() {
        let rng = MockTetrisRng::new();

        assert_eq!(rng.go_right, false);
        assert_eq!(rng.row_order.len(), 0);
        assert_eq!(rng.block_type, BlockType::LBlock);
        assert_eq!(rng.block_color, RED);
    }

    #[test]
    fn setters() {
        let mut rng = MockTetrisRng::new();

        rng.set_go_right(true);
        rng.set_random_row_order(Vec::from([1, 2, 3]));
        rng.set_block_type(BlockType::SquareBlock);
        rng.set_block_color(BLUE);

        assert_eq!(rng.go_right, true);
        assert_eq!(rng.row_order, Vec::from([1, 2, 3]));
        assert_eq!(rng.block_type, BlockType::SquareBlock);
        assert_eq!(rng.block_color, BLUE);
    }

    #[test]
    fn gen_do_go_right() {
        let mut rng = MockTetrisRng::new();

        assert_eq!(rng.gen_do_go_right(), false);

        rng.set_go_right(true);

        assert_eq!(rng.gen_do_go_right(), true);
    }

    #[test]
    fn get_random_row_order() {
        let mut rng: MockTetrisRng = MockTetrisRng::new();

        assert_eq!(rng.get_random_row_order(3), Vec::from([0, 1, 2]));

        rng.set_random_row_order(Vec::from([3, 2, 1]));

        assert_eq!(rng.get_random_row_order(2), Vec::from([3, 2, 1]));
        assert_eq!(rng.get_random_row_order(3), Vec::from([3, 2, 1]));
        assert_eq!(rng.get_random_row_order(4), Vec::from([3, 2, 1]));
    }

    #[test]
    fn shuffle_fields() {
        let rng: MockTetrisRng = MockTetrisRng::new();
        let field1 = Field::new(0, 0, RED, 0);
        let field2 = Field::new(0, 1, YELLOW, 1);
        let field3 = Field::new(1, 1, BLUE, 2);
        let fields_orig: Vec<&Field> = Vec::from([&field1, &field2, &field3]);
        let mut fields_to_shuffle = fields_orig.clone();

        rng.shuffle_fields(&mut fields_to_shuffle);

        assert_eq!(fields_to_shuffle, fields_orig);
    }

    #[test]
    fn generate_block_type() {
        let mut rng: MockTetrisRng = MockTetrisRng::new();

        assert_eq!(rng.generate_block_type(), BlockType::LBlock);

        rng.set_block_type(BlockType::SquareBlock);

        assert_eq!(rng.generate_block_type(), BlockType::SquareBlock);
    }

    #[test]
    fn generate_block_color() {
        let mut rng: MockTetrisRng = MockTetrisRng::new();

        assert_eq!(rng.generate_block_color(), RED);

        rng.set_block_color(BLUE);

        assert_eq!(rng.generate_block_color(), BLUE);
    }
}
