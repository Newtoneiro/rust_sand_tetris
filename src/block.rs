use crate::constants::block_schemas;

#[derive(Clone)]
pub struct Block {
    schema: Vec<(i8, i8)>,
    rotation: u8,
    block_type: BlockType,
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
        Block { schema: Vec::from(schema), rotation: 0, block_type }
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
        if !self.can_rotate() { return () }

        if self.rotation == 3 {
            self.rotation = 0;
        } else {
            self.rotation += 1;
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        if !self.can_rotate() { return () }

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
    fn create_lblock() {
        let lb: Block = Block::new(BlockType::LBlock);

        assert_eq!(lb.get_schema(), block_schemas::L_BLOCK);
    }

    #[test]
    fn rotate_90_lblock() {
        let mut lb: Block = Block::new(BlockType::LBlock);

        lb.rotate_clockwise();

        let rotated_schema = lb.get_schema();
        let expected_rotated_schema = Vec::from(
            [
                (0, 0),
                (1, 0),
                (-1, 0),
                (-1, 1),
            ]
        );

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
        let expected_rotated_schema = Vec::from(
            [
                (0, 1),
                (0, 0),
                (0, -1),
                (-1, -1),
            ]
        );

        for expected_square in expected_rotated_schema {
            assert!(rotated_schema.contains(&expected_square));
        }
    }

    #[test]
    fn rotate_270_lblock() {
        let mut lb: Block = Block::new(BlockType::LBlock);

        lb.rotate_counter_clockwise();

        let rotated_schema = lb.get_schema();
        let expected_rotated_schema = Vec::from(
            [
                (-1, 0),
                (0, 0),
                (1, 0),
                (1, -1),
            ]
        );

        for expected_square in expected_rotated_schema {
            assert!(rotated_schema.contains(&expected_square));
        }
    }
}