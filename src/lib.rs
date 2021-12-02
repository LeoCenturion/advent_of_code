fn main() {
    println!("Hello, world!");
}

pub mod day_1_sonar_sweep {
    pub type Measurement = u32;
    pub type Report = Vec<Measurement>;
    pub type DepthIncrements = i32;

    pub fn depth_increment_indicator_sw(report: Report) -> DepthIncrements {
        let mut window = [0; 3];
        let mut idx = 0;
        let mut window_occupation = 0;
        let window_size = 3;
        let mut new_report = Report::new();
        for measurement in report {
            window[idx % window_size] = measurement;
            if window_occupation < window_size {
                window_occupation += 1;
            }
            if window_occupation == window_size {
                new_report.push(window.iter().sum());
            }
            idx += 1;
        }
        depth_increment_indicator(new_report)
    }

    pub fn depth_increment_indicator(report: Report) -> DepthIncrements {
        if report.len() < 1 {
            0
        } else {
            let r = &report[..];
            let s = &report[1..];
            r.iter().zip(s.iter()).map(|(x, y)| (x < y) as usize).sum::<usize>() as i32
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod day_1_sonar_seep_tests {
    extern crate test_case;

    use test_case::test_case;

    use crate::day_1_sonar_sweep::{depth_increment_indicator, Measurement, Report, depth_increment_indicator_sw};

    #[test]
    fn sliding_window() {
        let report = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let answer = depth_increment_indicator_sw(report);

        assert_eq!(5, answer);
    }


    #[test]
    fn given_empty_report__then_answer_is_zero() {
        let report = get_empty_report();


        let answer = depth_increment_indicator(report);


        assert_eq!(0, answer);
    }

    #[test]
    fn given_single_measurement__then_answer_is_zero() {
        let report = get_report_with_one_entry();


        let answer = depth_increment_indicator(report);


        assert_eq!(0, answer);
    }

    #[test]
    fn given_report_with_decreasing_depth_only__then_answer_is_zero() {
        let report = get_report_with_decreasing_depths_only();


        let answer = depth_increment_indicator(report);


        assert_eq!(0, answer);
    }

    #[test_case(& [5, 6, 5, 4, 3, 2, 1]; "second measurement increases then all decrease")]
    #[test_case(& [5, 4, 3, 2, 1, 2]; "last measurement increases then all decrease")]
    #[test_case(& [5, 5, 5, 5, 5, 5, 53]; "last measurement increases then all the same")]
    fn given_report_with_one_increasing_depth_change__then_answer_is_one(report_slice: &[Measurement]) {
        let answer = depth_increment_indicator(Report::from(report_slice));


        assert_eq!(1, answer);
    }

    #[test]
    fn given_report_with_all_increasing_measurements__then_answer_is_len_of_the_report_minus_one() {
        let report = get_report_with_all_increasing_entries();
        let length = report.len() as i32;

        let answer = depth_increment_indicator(report);


        assert_eq!(length - 1, answer as i32);
    }

    fn get_report_with_all_increasing_entries() -> Report {
        let mut decreasing_report = get_report_with_decreasing_depths_only();
        decreasing_report.reverse();
        decreasing_report
    }

    fn get_report_with_decreasing_depths_only() -> Report {
        vec![10, 9, 8, 7, 6, 5, 4, 3]
    }

    fn get_report_with_one_entry() -> Report {
        vec![123]
    }

    fn get_empty_report() -> Report {
        vec![]
    }
}