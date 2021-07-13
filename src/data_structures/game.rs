use crate::data_structures::{point::Point, snake::Snake, direction::Direction};
use rand::{Rng, thread_rng};
use std::io::Stdout;
use crossterm::{terminal::{size, enable_raw_mode, SetSize, Clear, ClearType}, ExecutableCommand, cursor::Hide};

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
					match thread_rng().gen_range(0..4) {
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

	fn place_food(&mut self) {
		loop {
			let point = Point::new(thread_rng().gen_range(0..self.grid_size.0) , thread_rng().gen_range(0..self.grid_size.1));

			if !self.snake.contains_point(&point) {
				self.food = Some(point);
				break;
			}
		}
	}

	fn ui_setup(&mut self) {
		enable_raw_mode().unwrap();

		self.stdout
			.execute(SetSize(self.grid_size.0 + 3, self.grid_size.1 + 3)).unwrap()
			.execute(Clear(ClearType::All)).unwrap()
			.execute(Hide).unwrap();
	}
	
	fn render(&mut self) {}

	pub fn run(&mut self) {
		self.place_food();
		self.ui_setup();
		self.render();


	}
}