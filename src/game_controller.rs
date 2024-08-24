use macroquad::input::KeyCode;

use crate::{
    block_controller::BlockController, constants::map_constants::{MAP_HEIGHT, MAP_WIDTH}, graphic_controller::GraphicController, map::Map
};

pub struct GameController {
    score: i32,
    block_controller: BlockController,
    map: Map,
}

impl GameController {
    pub fn new() -> GameController {
        let block_controller: BlockController = BlockController::new();
        let map: Map = Map::new(MAP_WIDTH, MAP_HEIGHT);

        GameController { score: 0, block_controller, map }
    }

    pub fn init_game(&mut self) {
        self.block_controller.get_new_block();
    }

    pub async fn tick(&mut self) {
        GraphicController::draw_background();

        GraphicController::draw_block(self.block_controller.get_block_to_draw()).await;
        GraphicController::draw_fields(self.map.get_fields_to_draw()).await;
        
        GraphicController::flush().await;
        
        self.map.tick();
        self.block_controller.tick();

    }

    pub fn do_move(&mut self, key: KeyCode) {
        match key {
            KeyCode::D => self.block_controller.move_right(),
            KeyCode::A => self.block_controller.move_left(),
            KeyCode::S => self.block_controller.move_down(),
            _ => ()
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn create_game_controller() {
        let gc: GameController = GameController::new();

        assert_eq!(gc.score, 0);
    }
}