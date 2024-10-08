pub(crate) mod window_constants {
    pub(crate) const WINDOW_TITLE: &str = "Sand Tetris";
    pub(crate) const WINDOW_WIDTH: i32 = 500;
    pub(crate) const WINDOW_HEIGHT: i32 = 800;
}

pub(crate) mod map_constants {
    use super::window_constants;

    pub(crate) const GRAIN_SIDE_SIZE: i32 = 5;
    pub(crate) const MAP_WIDTH: i32 = {
        assert!(
            window_constants::WINDOW_WIDTH % GRAIN_SIDE_SIZE == 0,
            "WINDOW_WIDTH is not divisible by GRAIN_SIDE_SIZE"
        );
        window_constants::WINDOW_WIDTH / GRAIN_SIDE_SIZE
    };
    pub(crate) const MAP_HEIGHT: i32 = {
        assert!(
            window_constants::WINDOW_HEIGHT % GRAIN_SIDE_SIZE == 0,
            "WINDOW_HEIGHT is not divisible by GRAIN_SIDE_SIZE"
        );
        window_constants::WINDOW_HEIGHT / GRAIN_SIDE_SIZE
    };
}

pub(crate) mod colors {
    use macroquad::color::Color;

    pub(crate) const BACKGROUND_COLOR: Color = Color::new(0.10196, 0.0941, 0.1058, 1.0);
    pub(crate) const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);

    pub(crate) const WHITE: Color = Color::new(0.9196, 0.8941, 0.9058, 1.0);

    pub(crate) const RED: Color = Color::new(0.6392, 0.0862, 0.1294, 1.0);
    pub(crate) const RED_LIGHT: Color = Color::new(0.6509, 0.2431, 0.2784, 1.0);
    pub(crate) const RED_DARK: Color = Color::new(0.3960, 0.0588, 0.0901, 1.0);
    pub(crate) const RED_VAR: [Color; 3] = [RED, RED_LIGHT, RED_DARK];

    pub(crate) const BLUE: Color = Color::new(0.0039, 0.4352, 0.7254, 1.0);
    pub(crate) const BLUE_LIGHT: Color = Color::new(0.2549, 0.6078, 0.8588, 1.0);
    pub(crate) const BLUE_DARK: Color = Color::new(0.0000, 0.2588, 0.4862, 1.0);
    pub(crate) const BLUE_VAR: [Color; 3] = [BLUE, BLUE_LIGHT, BLUE_DARK];

    pub(crate) const GREEN: Color = Color::new(0.2980, 0.6862, 0.3137, 1.0);
    pub(crate) const GREEN_LIGHT: Color = Color::new(0.4549, 0.7843, 0.4666, 1.0);
    pub(crate) const GREEN_DARK: Color = Color::new(0.2078, 0.4862, 0.2196, 1.0);
    pub(crate) const GREEN_VAR: [Color; 3] = [GREEN, GREEN_LIGHT, GREEN_DARK];

    pub(crate) const YELLOW: Color = Color::new(0.8588, 0.6, 0.3529, 1.0);
    pub(crate) const YELLOW_LIGHT: Color = Color::new(0.9607, 0.7490, 0.5294, 1.0);
    pub(crate) const YELLOW_DARK: Color = Color::new(0.7372, 0.4862, 0.2588, 1.0);
    pub(crate) const YELLOW_VAR: [Color; 3] = [YELLOW, YELLOW_LIGHT, YELLOW_DARK];
}

pub(crate) mod block_constants {
    use super::map_constants::MAP_WIDTH;

    pub(crate) const BLOCK_CHUNK_SIDE: i32 = MAP_WIDTH / 12;
    pub(crate) const PREVIEW_BLOCK_CHUNK_SIDE: i32 = 3;
    pub(crate) const BLOCK_STARTING_POS: (i32, i32) = (
        MAP_WIDTH / 2 - (BLOCK_CHUNK_SIDE / 2),
        -BLOCK_CHUNK_SIDE * 2,
    );
}

pub(crate) mod block_schemas {
    pub(crate) const L_BLOCK: [(i8, i8); 4] = [(0, 0), (0, -1), (0, 1), (1, 1)];
    pub(crate) const REV_L_BLOCK: [(i8, i8); 4] = [(0, 0), (0, -1), (0, 1), (-1, 1)];
    pub(crate) const SQUARE_BLOCK: [(i8, i8); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
    pub(crate) const Z_BLOCK: [(i8, i8); 4] = [(0, 0), (0, -1), (1, 0), (1, 1)];
    pub(crate) const REV_Z_BLOCK: [(i8, i8); 4] = [(0, 0), (0, 1), (1, 0), (1, -1)];
    pub(crate) const I_BLOCK: [(i8, i8); 4] = [(0, 0), (0, -1), (0, 1), (0, 2)];
    pub(crate) const T_BLOCK: [(i8, i8); 4] = [(0, 0), (-1, 0), (1, 0), (0, -1)];
}

pub(crate) mod block_skins {
    pub(crate) const SKIN_SIDE: usize = 6;
    // 0 - normal, 1 - light, 2 - dark
    pub(crate) const NATURAL: [[usize; SKIN_SIDE]; SKIN_SIDE] = [
        [2, 1, 0, 0, 0, 2],
        [2, 0, 0, 1, 1, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 2, 1],
        [1, 1, 0, 0, 0, 1],
        [0, 0, 2, 2, 0, 0],
    ];
}

pub(crate) mod interface_constants {
    pub(crate) const GAME_OVER_TEXT: &str = "Game Over";
    pub(crate) const GAME_OVER_FONT_SIZE: u16 = 60;
    pub(crate) const GAME_OVER_BOTTOM_TEXT: &str = "press (R) to restart...";
    pub(crate) const GAME_OVER_BOTTOM_FONT_SIZE: u16 = 40;
    pub(crate) const GAME_OVER_OUTLINE_WIDTH: u16 = 3;

    pub(crate) const SCORE_TEXT: &str = "Score";
    pub(crate) const SCORE_FONT_SIZE: u16 = 45;
    pub(crate) const SCORE_OUTLINE_WIDTH: u16 = 3;

    pub(crate) const H_BORDER_OFFSET: f32 = 20.0;
    pub(crate) const V_BORDER_OFFSET: f32 = 20.0;
}

pub(crate) mod animation_constants {
    pub(crate) const DEMOLISHION_CHUNK_SIZE: usize = 20;
}

#[derive(Debug, PartialEq)]
pub struct TetrisConstants {
    pub map_width: i32,
    pub map_height: i32,
    pub block_chunk_side: i32,
    pub grain_side_size: i32,
    pub preview_block_chunk_side: i32,
    pub block_starting_pos: (i32, i32),
}
