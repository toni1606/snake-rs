use crate::data_structures::point::Point;
use crate::data_structures::direction::Direction;

pub struct Snake {
	body: Vec<Point>,
	direction: Direction,
	digesting: bool
}

impl Snake {
	fn new(start: Point, length: u8, direction: Direction) -> Self {
		
		for body_segment in 0..length {
			
		}

		Self {
			body: ,
			direction: direction,
			digesting: false
		}
	}
}