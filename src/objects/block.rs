use crate::constants::block_schemas;

#[derive(Clone)]
pub struct Block {
    schema: Vec<(i8, i8)>,
    block_type: BlockType,
    rotation: u8,
}

#[derive(Clone, PartialEq)]
pub enum BlockType {
    LBlock,
    RevLBlock,
    SquareBlock,
    ZBlock,
    RevZBlock,
    IBlock,
    TBlock,
}

const NOT_ROTABLE_TYPES: [BlockType; 1] = [BlockType::SquareBlock];

impl Block {
    pub fn new(block_type: BlockType) -> Block {
        let schema = match block_type {
            BlockType::LBlock => block_schemas::L_BLOCK,
            BlockType::RevLBlock => block_schemas::REV_L_BLOCK,
            BlockType::SquareBlock => block_schemas::SQUARE_BLOCK,
            BlockType::ZBlock => block_schemas::Z_BLOCK,
            BlockType::RevZBlock => block_schemas::REV_Z_BLOCK,
            BlockType::IBlock => block_schemas::I_BLOCK,
            BlockType::TBlock => block_schemas::T_BLOCK,
        };
        Block {
            schema: Vec::from(schema),
            rotation: 0,
            block_type,
        }
    }

    pub fn get_schema(&self) -> Vec<(i8, i8)> {
        match self.rotation {
            0 => self.schema.clone(),
            1 => {
                let mut output_schema = Vec::new();
                for (x, y) in self.schema.clone() {
                    output_schema.push((-y, x));
                }
                output_schema
            }
            2 => {
                let mut output_schema = Vec::new();
                for (x, y) in self.schema.clone() {
                    output_schema.push((-x, -y));
                }
                output_schema
            }
            3 => {
                let mut output_schema = Vec::new();
                for (x, y) in self.schema.clone() {
                    output_schema.push((y, -x));
                }
                output_schema
            }
            _ => Vec::new(),
        }
    }

    fn can_rotate(&self) -> bool {
        !NOT_ROTABLE_TYPES.contains(&self.block_type)
    }

    pub fn rotate_clockwise(&mut self) {
        if !self.can_rotate() {
            return ();
        }

        if self.rotation == 3 {
            self.rotation = 0;
        } else {
            self.rotation += 1;
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        if !self.can_rotate() {
            return ();
        }

        if self.rotation == 0 {
            self.rotation = 3;
        } else {
            self.rotation -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_lblock_simple() {
        let lb: Block = Block::new(BlockType::LBlock);

        assert_eq!(lb.get_schema(), block_schemas::L_BLOCK);
    }

    #[test]
    fn can_not_rotate() {
        for non_rotable_type in [BlockType::SquareBlock] {
            let b: Block = Block::new(non_rotable_type);

            assert!(!b.can_rotate());
        }
    }

    #[test]
    fn can_rotate() {
        for non_rotable_type in [
            BlockType::LBlock,
            BlockType::RevLBlock,
            BlockType::ZBlock,
            BlockType::RevZBlock,
            BlockType::IBlock,
            BlockType::TBlock,
        ] {
            let b: Block = Block::new(non_rotable_type);

            assert!(b.can_rotate());
        }
    }

    #[test]
    fn get_schema_invalid_value() {
        let mut lb: Block = Block::new(BlockType::LBlock);

        lb.rotation = 5;
        let rotated_schema = lb.get_schema();

        assert_eq!(rotated_schema.len(), 0);
    }

    #[test]
    fn rotate_clockwise() {
        let mut lb: Block = Block::new(BlockType::LBlock);

        assert_eq!(lb.rotation, 0);

        lb.rotate_clockwise();

        assert_eq!(lb.rotation, 1);
    }

    #[test]
    fn rotate_counter_clockwise() {
        let mut lb: Block = Block::new(BlockType::LBlock);
        lb.rotate_clockwise();

        assert_eq!(lb.rotation, 1);

        lb.rotate_counter_clockwise();

        assert_eq!(lb.rotation, 0);
    }

    #[test]
    fn cant_rotate_unrotable_block() {
        let mut lb: Block = Block::new(BlockType::SquareBlock);
        assert_eq!(lb.rotation, 0);

        lb.rotate_clockwise();
        assert_eq!(lb.rotation, 0);

        lb.rotate_counter_clockwise();
        assert_eq!(lb.rotation, 0);
    }

    #[test]
    fn rotate_pass_12_o_clock() {
        let mut lb: Block = Block::new(BlockType::LBlock);

        assert_eq!(lb.rotation, 0);

        lb.rotate_counter_clockwise();

        assert_eq!(lb.rotation, 3);

        lb.rotate_clockwise();

        assert_eq!(lb.rotation, 0);
    }

    #[test]
    fn rotate_90_lblock() {
        let mut lb: Block = Block::new(BlockType::LBlock);

        lb.rotate_clockwise();

        let rotated_schema = lb.get_schema();
        let expected_rotated_schema = Vec::from([(0, 0), (1, 0), (-1, 0), (-1, 1)]);

        for expected_square in expected_rotated_schema {
            assert!(rotated_schema.contains(&expected_square));
        }
    }

    #[test]
    fn rotate_180_lblock() {
        let mut lb: Block = Block::new(BlockType::LBlock);

        lb.rotate_clockwise();
        lb.rotate_clockwise();

        let rotated_schema = lb.get_schema();
        let expected_rotated_schema = Vec::from([(0, 1), (0, 0), (0, -1), (-1, -1)]);

        for expected_square in expected_rotated_schema {
            assert!(rotated_schema.contains(&expected_square));
        }
    }

    #[test]
    fn rotate_270_lblock() {
        let mut lb: Block = Block::new(BlockType::LBlock);

        lb.rotate_counter_clockwise();

        let rotated_schema = lb.get_schema();
        let expected_rotated_schema = Vec::from([(-1, 0), (0, 0), (1, 0), (1, -1)]);

        for expected_square in expected_rotated_schema {
            assert!(rotated_schema.contains(&expected_square));
        }
    }
}
