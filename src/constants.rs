pub(crate) mod window_constants {
    pub(crate) const WINDOW_TITLE: &str = "Sand Tetris";
    pub(crate) const WINDOW_WIDTH: i32 = 500;
    pub(crate) const WINDOW_HEIGHT: i32 = 800;
}

pub(crate) mod map_constants {
    use super::window_constants;

    pub(crate) const GRAIN_SIDE_SIZE: i32 = 2;
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

    pub(crate) const COLOR_VAR_WEIGHTS: [f32; 3] = [0.5, 0.25, 0.25];
    
    pub(crate) const RED: Color = Color::new(0.6392, 0.0862, 0.1294, 1.0);
    pub(crate) const RED_LIGHT: Color = Color::new(0.6509, 0.2431, 0.2784, 1.0);
    pub(crate) const RED_DARK: Color = Color::new(0.3960, 0.0588, 0.0901, 1.0);
    pub(crate) const RED_VAR: [Color; 3] = [RED, RED_LIGHT, RED_DARK];
    
    pub(crate) const BLUE: Color = Color::new(0.0039, 0.4352, 0.7254, 1.0);
    
    pub(crate) const GREEN: Color = Color::new(0.4980, 0.6980, 0.5215, 1.0);
    
    pub(crate) const YELLOW: Color = Color::new(0.8588, 0.6, 0.3529, 1.0);
}

pub(crate) mod block_constants {
    use super::map_constants::MAP_WIDTH;

    pub(crate) const BLOCK_CHUNK_SIDE: i32 = 10;
    pub(crate) const BLOCK_STARTING_POS: (i32, i32) = (
        MAP_WIDTH / 2 - (BLOCK_CHUNK_SIDE / 2),
        BLOCK_CHUNK_SIDE * 4,
    );
}

pub(crate) mod block_schemas {
    pub(crate) const L_BLOCK: [(i8, i8); 4] = [(0, 0), (0, -1), (0, 1), (1, 1)];
    pub(crate) const REV_L_BLOCK: [(i8, i8); 4] = [(0, 0), (0, -1), (0, 1), (-1, 1)];
    pub(crate) const SQUARE_BLOCK: [(i8, i8); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
    pub(crate) const Z_BLOCK: [(i8, i8); 4] = [(0, 0), (0, -1), (1, 0), (1, 1)];
    pub(crate) const REV_Z_BLOCK: [(i8, i8); 4] = [(0, 0), (0, 1), (1, 0), (1, -1)];
    pub(crate) const I_BLOCK: [(i8, i8); 4] = [(0, 0), (0, -1), (0, 1), (0, 2)];
}

pub(crate) mod game_config {
    pub(crate) const RANDOMIZE_BLOCK_COLOUR: bool = true;
}