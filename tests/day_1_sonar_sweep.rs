use std::path::Path;
use std::fs::File;
use std::io::Read;
use advent_of_code::day_1_sonar_sweep::*;

#[test]
fn test_real_input() {
    let report = read_report();
    let answer = depth_increment_indicator(report);
    assert_eq!(1692, answer)
}

const DAY_1_INPUT: &'static str = "./resources/day_1_input";

fn read_report() -> Report {
    let path = Path::new(DAY_1_INPUT);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {},
    }

    s.lines().map(|line|line.parse::<Measurement>())
        .filter(|parsed| parsed.is_ok())
        .map(|parsed| parsed.unwrap())
        .collect()
}