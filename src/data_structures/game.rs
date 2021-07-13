use crate::data_structures::point::Point;
use crate::data_structures::snake::Snake;
use crate::data_structures::direction::Direction;

use crossterm::terminal::size;

use rand::Rng;

use std::io::Stdout;

#[derive(Debug)]
pub struct Game {
	stdout: Stdout,
	original_terminal_size: (u16, u16),
	grid_size: (u16, u16),
	food: Option<Point>,
	snake: Snake,
	movement_speed: u16,
	score: u16
}

impl Game {
	pub fn new(stdout: Stdout, grid_size: (u16, u16)) -> Self {
		Self {
			stdout: stdout,
			original_terminal_size: size().unwrap(),
			grid_size: grid_size,
			food: None,
			snake: Snake::new(
				Point::new(
					grid_size.0 / 2, grid_size.1 / 2),
					3,
					match rand::thread_rng().gen_range(0..4) {
						0 => Direction::Up,
						1 => Direction::Down,
						2 => Direction::Left,
						_ => Direction::Right
					}
				),
			movement_speed: 0,
			score: 0
		}
	}

	
}