use crate::data_structures::direction::Direction;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Command {
	Quit,
	Move(Direction)
}