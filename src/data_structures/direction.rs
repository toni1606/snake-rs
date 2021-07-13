#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
	Left,
	Up,
	Right,
	Down
}

impl Direction {
	fn opposite(&self) -> Self {
		match self {
			Direction::Down 	=> Direction::Up,
			Direction::Up		=> Direction::Down,
			Direction::Left 	=> Direction::Right,
			Direction::Right	=> Direction::Left
		}
	}
}