use crate::data_structures::direction::Direction;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
	pub x: u16,
	pub y: u16
}

impl Point {
	pub fn new(x: u16, y: u16) -> Self {
		Self {
			x, 
			y
		}
	}

	pub fn move_point(&mut self, direction: Direction, tiles: u16) {
		let tiles = tiles as i16;

		let move_coordinates = match direction {
			Direction::Left		=> (-tiles, 0),
			Direction::Up		=> (0, -tiles),
			Direction::Right	=> (0, tiles),
			Direction::Down		=> (0, tiles)
		};

		self.x = Point::move_coordinate(self.x, move_coordinates.0);
		self.y = Point::move_coordinate(self.y, move_coordinates.1);
	}
	
	fn move_coordinate(value: u16, by: i16) -> u16 {
		if by.is_negative() && by.abs() > value as i16 {
			panic!("Transforming value {} by {} woud be negative", value, by);
		}
		(value as i16 + by) as u16
	}
}