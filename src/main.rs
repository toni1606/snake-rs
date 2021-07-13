use snake::data_structures::game::Game;
use std::io::stdout;

fn main() {
    println!("Hello, world!");
	Game::new(stdout(), (10, 10)).run();
}
