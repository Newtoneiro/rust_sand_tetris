use crate::constants::colors::BACKGROUND_COLOR;
use crate::field::Field;

pub struct Map {
    width: usize,
    height: usize,
    grid: Vec<Vec<Field>>
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = Map::create_grid(width, height);

        Map { width, height, grid }
    }

    fn create_grid(width: usize, height: usize) -> Vec<Vec<Field>> {
        let mut grid: Vec<Vec<Field>> = Vec::new();
        for y in 0..height {
            grid.push(Vec::new());
            for x in 0..width {
                grid[y].push(
                    Field::new(x, y, BACKGROUND_COLOR)
                );
            }
        };
        grid
    }

    pub fn get_field(&self, x: usize, y: usize) -> Option<&Field> {
        if !self.check_coords_in_bounds(x, y) {
            return None
        }
        Some(&self.grid[y][x])
    }

    fn check_coords_in_bounds(&self, x: usize, y: usize) -> bool {
        if (x < self.width) && (y < self.height) {
            return true;
        }
        false
    }

    pub fn get_grid(&self) -> &Vec<Vec<Field>> {
        &self.grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_map() {
        let map: Map = Map::new(200, 400);

        assert_eq!(map.width, 200);
        assert_eq!(map.height, 400);
        assert_eq!(map.grid.len(), 400);
        assert_eq!(map.grid.get(0).unwrap().len(), 200);
    }

    #[test]
    fn get_field() {
        let map: Map = Map::new(200, 400);

        let field_middle: &Field = map.get_field(20, 40).unwrap();
        let field_min: &Field = map.get_field(0, 0).unwrap();
        let field_max: &Field = map.get_field(199, 399).unwrap();

        assert_eq!(field_middle.get_x(), 20);
        assert_eq!(field_middle.get_y(), 40);
        assert_eq!(field_min.get_x(), 0);
        assert_eq!(field_min.get_y(), 0);
        assert_eq!(field_max.get_x(), 199);
        assert_eq!(field_max.get_y(), 399);
    }

    #[test]
    fn get_field_out_of_bounds() {
        let map: Map = Map::new(200, 400);

        for (x, y) in [(200, 0), (201, 40), (0, 400), (100, 401)] {
            let field_middle = map.get_field(x, y);

            assert!(field_middle.is_none());
        }
    }
}