use advent_of_code::day_3_binary_diagnostic::{get_gamma_rate, get_power_consumption};
use test_utils::read_lines_from;

mod test_utils;

const DAY_3_INPUT: &'static str = "./resources/day_3_input";

#[test]
fn test() {
    let lines = read_lines_from(DAY_3_INPUT);
    let commands: Vec<u8> = lines.lines().map(|l| {
        println!("{}", l);
        l.parse::<u8>().unwrap()
    }).collect();

    let consumption = get_power_consumption(&commands);

    assert_eq!(0, consumption)
}