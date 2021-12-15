use crate::{common::Commandable, Command};

#[derive(Copy, Clone)]
pub struct Submarine {
    position_horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    pub(crate) fn default() -> Submarine {
        Submarine {
            position_horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Commandable for Submarine {
    fn navigate(&self, cmd: Command) -> Self {
        match cmd {
            Command::FORWARD(unit) => Self {
                position_horizontal: self.position_horizontal + unit,
                depth: self.depth + (self.aim * unit),
                ..*self
            },
            Command::DOWN(unit) => Self {
                aim: self.aim + unit,
                ..*self
            },
            Command::UP(unit) => Self {
                aim: self.aim - unit,
                ..*self
            },
            _ => *self,
        }
    }

    fn get_position(&self) -> i32 {
        self.position_horizontal * self.depth
    }
}
