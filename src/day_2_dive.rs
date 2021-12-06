use std::iter::Sum;

use crate::day_2_dive::Command::{Down, Forward, Up};

#[derive(Debug, Default)]
pub struct Position(i32, i32);

impl Sum for Position {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut result = Position(0, 0);
        for m in iter {
            result.0 += m.0;
            result.1 += m.1;
        }
        result
    }
}

impl Into<(i32, i32)> for Position {
    fn into(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

impl From<(&str, i32)> for Position {
    fn from((direction, magnitude): (&str, i32)) -> Self {
        match direction {
            "forward" => Position(magnitude, 0),
            "down" => Position(0, magnitude),
            "up" => Position(0, -magnitude),
            _ => Position(0, 0),
        }
    }
}

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<(&str, i32)> for Command {
    fn from((direction, magnitude): (&str, i32)) -> Self {
        match direction {
            "forward" => Forward(magnitude),
            "down" => Down(magnitude),
            "up" => Up(magnitude),
            _ => Forward(0),
        }
    }
}
#[derive(Debug, Default)]
pub struct PositionWitAim(i32, i32, i32);
impl Into<Position> for PositionWitAim {
    fn into(self) -> Position {
        Position(self.0, self.2)
    }
}

pub fn move_submarine(commands: Vec<&str>) -> Position {
    commands.into_iter().map(|command| {
        let mut splited = command.split(" ");
        let direction: &str = splited.next().unwrap();
        let magnitude: i32 = splited.next().unwrap().parse::<i32>().unwrap();
        Position::from((direction, magnitude))
    }).sum()
}

pub fn move_submarine_2(commands: Vec<&str>) -> Position {
    commands.into_iter().map(|command| {
        let mut splited = command.split(" ");
        let direction: &str = splited.next().unwrap();
        let magnitude: i32 = splited.next().unwrap().parse::<i32>().unwrap();
        Command::from((direction, magnitude))
    }).fold(PositionWitAim::default(), |pos, comm| {
        match comm {
            Forward(m) => PositionWitAim(pos.0 + m, pos.1,pos.2 + pos.1 * m, ),
            Down(m) => PositionWitAim(pos.0, pos.1 + m, pos.2),
            Up(m) => PositionWitAim(pos.0, pos.1 - m, pos.2),
        }
    }).into()
}

#[cfg(test)]
mod tests_part_two {
    use crate::day_2_dive::{move_submarine_2, Position};

    #[test]
    fn test() {
        let commands = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];

        let Position(horizontal_pos, depth) = move_submarine_2(commands);

        assert_eq!(900, horizontal_pos * depth)
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2_dive::*;

    #[test]
    fn test() {
        let commands = vec!["forward 1", "down 2", "up 3"];

        let Position(horizontal_pos, depth) = move_submarine(commands);

        assert_eq!(-1, horizontal_pos * depth)
    }

    #[test]
    fn test2() {
        let commands = vec!["forward 5",
                            "down 5",
                            "forward 8",
                            "up 3",
                            "down 8",
                            "forward 2"];

        let Position(horizontal_pos, depth) = move_submarine(commands);

        assert_eq!(150, horizontal_pos * depth)
    }
}