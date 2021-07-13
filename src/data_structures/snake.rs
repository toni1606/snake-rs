use crate::data_structures::point::Point;
use crate::data_structures::direction::Direction;

#[derive(Debug)]
pub struct Snake {
	body: Vec<Point>,
	direction: Direction,
	digesting: bool
}

impl Snake {
	pub fn new(start: Point, length: u16, direction: Direction) -> Self {
		let mut body: Vec<Point> = vec![]; 
		
		for body_segment in 0..length {
			let mut seg = start;
			seg.move_point(direction.opposite(), body_segment);
			body.push(seg);
		}

		Self {
			body: body,
			direction: direction,
			digesting: false
		}
	}
}