use std::iter::Sum;

pub struct MovementVector(i32, i32);

impl Sum for MovementVector {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut result = MovementVector(0, 0);
        for m in iter {
            result.0 += m.0;
            result.1 += m.1;
        }
        result
    }
}

impl Into<(i32,i32)> for MovementVector {
    fn into(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

pub fn move_submarine(commands: Vec<&str>) -> MovementVector {
    commands.into_iter().map(|command| {
        let mut splited = command.split(" ");
        let direction: &str = splited.next().unwrap();
        let magnitude: i32 = splited.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => MovementVector(magnitude, 0),
            "down" => MovementVector(0, magnitude),
            "up" => MovementVector(0, -magnitude),
            _ => MovementVector(0, 0),
        }
    }).sum()
}


#[cfg(test)]
mod tests {
    use crate::day_2_dive::*;

    #[test]
    fn test() {
        let commands = vec!["forward 1", "down 2", "up 3"];

        let MovementVector(horizontal_pos, depth) = move_submarine(commands);

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

        let MovementVector(horizontal_pos, depth) = move_submarine(commands);

        assert_eq!(150, horizontal_pos * depth)
    }
}