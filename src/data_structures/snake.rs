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

	pub fn get_head_point(&self) -> Point {
		self.body.first().unwrap().clone()
	}

	pub fn get_body_points(&self) -> Vec<Point> {
		self.body.clone()
	}

	pub fn get_direction(&self) -> Direction {
		self.direction
	}

	pub fn contains_point(&self, point: &Point) -> bool {
		self.body.contains(point)
	}

	pub fn set_direction(&mut self, new_direction: Direction) {
		self.direction = new_direction;
	}

	pub fn grow(&mut self) {
		self.digesting = true;
	}

	pub fn slither(&mut self) {
		self.body.get_mut(0).unwrap().move_point(self.direction, 1);

		if !self.digesting {
			self.body.remove(self.body.len() - 1);
		} else {	
			self.digesting = false
		}
	}

}