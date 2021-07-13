use crate::data_structures::{point::Point, snake::Snake, direction::Direction, command::Command};

use rand::{Rng, thread_rng};

use std::io::Stdout;
use std::time::{Duration, Instant};

use crossterm::terminal::{Clear, ClearType, size, SetSize, enable_raw_mode, disable_raw_mode};
use crossterm::ExecutableCommand;
use crossterm::cursor::{Show, MoveTo, Hide};
use crossterm::style::{SetForegroundColor, Print, ResetColor, Color};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers, KeyEvent};

// Time Constants
const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;


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

	pub fn run(&mut self) {
		self.place_food();
		self.ui_setup();
		self.render();

		let mut done = false;
		while !done {
			let interval = self.calculate_interval();
			let direction = self.snake.get_direction();
			let now = Instant::now();

			while now.elapsed() < interval {
				if let Some(command) = self.get_command(interval - now.elapsed()) {

				}
			}
		}

		self.ui_restore();
	}

	fn calculate_interval(&self) -> Duration {
		let speed = MAX_SPEED - self.movement_speed;
		Duration::from_millis(
			(MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64
		)
	}

	fn wait_for_key_event(&self, wait_for: Duration) -> Option<KeyEvent> {
		if poll(wait_for).ok()? {
            let event = read().ok()?;
            if let Event::Key(key_event) = event {
                return Some(key_event);
            }
        }

        None
	}

	fn get_command(&self, wait_for: Duration) -> Option<Command> {
		let key_event = self.wait_for_key_event(wait_for)?;
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

	fn ui_restore(&mut self) {
		let (cols, rows) = self.original_terminal_size;
		self.stdout
			.execute(SetSize(self.grid_size.0 + 3, self.grid_size.1 + 3)).unwrap()
			.execute(Clear(ClearType::All)).unwrap()
			.execute(Hide).unwrap();
	}
	
	fn render(&mut self) {
		self.draw_walls();
		self.draw_background();
		self.draw_snake();
		self.draw_food();
	}

	fn draw_walls(&mut self) {
		self.stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();

		// draw lateral walls
		for x in 0..self.grid_size.0 + 2 {
			self.stdout
				.execute(MoveTo(x, 0)).unwrap()
				.execute(Print("#")).unwrap()
				.execute(MoveTo(x, self.grid_size.1 + 1)).unwrap()
				.execute(Print("#")).unwrap();
		}

		// draw vertical walls
		for y in 0..self.grid_size.1 + 2 {
			self.stdout
				.execute(MoveTo(0, y)).unwrap()
				.execute(Print("#")).unwrap()
				.execute(MoveTo(self.grid_size.0 + 1, y)).unwrap()
				.execute(Print("#")).unwrap();
		}

		// draw corners
		self.stdout
            .execute(MoveTo(0, 0)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(self.grid_size.0 + 1, self.grid_size.1 + 1)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(self.grid_size.0 + 1, 0)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(0, self.grid_size.1 + 1)).unwrap()
            .execute(Print("#")).unwrap();
	}

	fn draw_background(&mut self) {
		self.stdout.execute(ResetColor).unwrap();

		// loop through grid and print a space everywhere ecept borders
		for y in 1..self.grid_size.1 + 1 {
			for x in 1..self.grid_size.0 + 1 {
				self.stdout
					.execute(MoveTo(x, y)).unwrap()
					.execute(Print(" ")).unwrap();
			}
		}
	}

	fn draw_snake(&mut self) {
        self.stdout.execute(
			SetForegroundColor(
				match self.movement_speed % 3 {
            		0 => Color::Green,
            		1 => Color::Cyan,
            		_ => Color::Yellow
        		}
			)
		).unwrap();

        let body_points = self.snake.get_body_points();
        for (i, body) in body_points.iter().enumerate() {
            let previous = if i == 0 { None } else { body_points.get(i - 1) };
            let next = body_points.get(i + 1);
            
			let symbol = if let Some(&next) = next {
                if let Some(&previous) = previous {
                    if previous.x == next.x {
                        '║'
                    } else if previous.y == next.y {
                        '═'
                    } else {
                        let mut d = body.clone();
						d.move_point(Direction::Down, 1);

                        let mut r = body.clone();
						r.move_point(Direction::Right, 1);

                        let u = {
							if body.y == 0 { 
								body.clone() 
							} else { 
								let mut temp = body.clone();
								temp.move_point(Direction::Up, 1);
								temp 
							}
						};
                        let l = {
							if body.x == 0 {
								body.clone()
							} else {
								let mut temp = body.clone();
								temp.move_point(Direction::Left, 1);
								temp
							}
						};

                        if (next == d && previous == r) || (previous == d && next == r) {
                            '╔'
                        } else if (next == d && previous == l) || (previous == d && next == l) {
                            '╗'
                        } else if (next == u && previous == r) || (previous == u && next == r) {
                            '╚'
                        } else {
                            '╝'
                        }
                    }
                } else {
                    'O'
                }
            } else if let Some(&previous) = previous {
                if body.y == previous.y {
                    '═'
                } else {
                    '║'
                }
            } else {
                panic!("Invalid snake body point.");
            };

            self.stdout
                .execute(MoveTo(body.x + 1, body.y + 1)).unwrap()
                .execute(Print(symbol)).unwrap();
        }
	}

	fn draw_food(&mut self) {
		self.stdout.execute(SetForegroundColor(Color::White)).unwrap();

		for food in self.food.iter() {
			self.stdout
				.execute(MoveTo(food.x + 1, food.y + 1)).unwrap()
				.execute(Print("•")).unwrap();
		}
	}	
}