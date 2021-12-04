mod test_utils;
use test_utils::read_lines_from;
use advent_of_code::day_2_dive::{MovementVector, move_submarine};

const DAY_2_INPUT: &'static str = "./resources/day_2_input";
#[test]
fn test() {
    let lines = read_lines_from(DAY_2_INPUT);
    let commands = lines.lines().collect();

    let  (horizontal_pos, depth) = move_submarine(commands).into();

    assert_eq!(-1, horizontal_pos * depth)

}