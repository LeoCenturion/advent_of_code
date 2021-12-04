use std::path::Path;
use std::fs::File;
use std::io::Read;
use advent_of_code::day_1_sonar_sweep::*;
use advent_of_code::day_1_sonar_sweep::in_mem_report::InMemoryReport;
use std::borrow::Borrow;

mod test_utils;
use test_utils::read_lines_from;

#[test]
fn test_real_input() {
    let report = read_report();
    let answer = depth_increment_indicator(report);
    assert_eq!(1692, answer)
}
#[test]
fn test_real_input_sliding_window() {
    let report = read_report();
    let answer = depth_increment_indicator_sw(report);
    assert_eq!(1692, answer)
}


const DAY_1_INPUT: &'static str = "./resources/day_1_input";

fn read_report() -> InMemoryReport {
    let s = read_lines_from(DAY_1_INPUT);

    let ms:Vec<Measurement> = s.map(|line|line.parse::<Measurement>())
        .filter(|parsed| parsed.is_ok())
        .map(|parsed| parsed.unwrap())
        .collect();

    InMemoryReport::from(ms.as_ref())
}

