pub(crate) mod window_constants {
    pub(crate) const WINDOW_TITLE: &str = "Sand Tetris";
    pub(crate) const WINDOW_WIDTH: i32 = 500;
    pub(crate) const WINDOW_HEIGHT: i32 = 800;
}

pub(crate) mod map_constants {
    use super::window_constants;

    pub(crate) const GRAIN_SIDE_SIZE: i32 = 4;
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

    pub(crate) const BACKGROUND_COLOR: Color = Color::new(0.094, 0.074, 0.070, 1.0);
}