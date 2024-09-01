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
}

impl MockTetrisRng {
    #[warn(dead_code)]
    pub fn new() -> MockTetrisRng {
        MockTetrisRng { go_right: false }
    }

    #[warn(dead_code)]
    pub fn set_go_right(&mut self, go_right: bool) {
        self.go_right = go_right
    }
}

impl TetrisRng for MockTetrisRng {
    fn gen_do_go_right(&self) -> bool {
        self.go_right
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
