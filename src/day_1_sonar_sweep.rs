pub type Measurement = u32;
pub type DepthIncrements = usize;

pub trait Report: Iterator<Item=Measurement> {}

impl Report for dyn Iterator<Item=Measurement> {}

pub fn depth_increment_indicator_sw<R>(report: R) -> DepthIncrements
    where R: Report {
    let window_size = 3;

    let new_report = report.into_iter().scan(([0; 3], 0, 0), |(window, idx, window_occupation), m| {
        window[*idx % window_size] = m;
        *idx += 1;
        let sum = window.clone().iter().sum();
        Some(sum)
    }).skip(window_size-1);
    depth_increment_indicator(new_report)
}

pub fn depth_increment_indicator<I>(report: I) -> DepthIncrements
    where I: Iterator<Item=Measurement> {
    report.scan::<Measurement, usize, fn(&mut Measurement, Measurement) -> Option<usize>>(Measurement::default(), |st, m| {
        if *st <= 0 {
            *st = m;
            Some(false as usize)
        } else {
            let prev = *st;
            *st = m;
            Some((prev < m) as usize)
        }
    }).sum()
}

pub mod in_mem_report {
    use crate::day_1_sonar_sweep::{Measurement, Report};

    pub struct InMemoryReport {
        measurements: Vec<Measurement>,
        idx: usize,
    }

    impl InMemoryReport {
        pub(crate) fn reverse(&mut self) {
            self.measurements.reverse()
        }
        pub fn len(&self) -> usize {
            self.measurements.len()
        }
    }

    impl From<&[Measurement]> for InMemoryReport {
        fn from(m: &[Measurement]) -> Self {
            Self {
                measurements: Vec::from(m),

                idx: 0,
            }
        }
    }


    impl Iterator for InMemoryReport {
        type Item = Measurement;

        fn next(&mut self) -> Option<Measurement> {
            let m = self.measurements.get(self.idx);
            self.idx += 1;
            m.map(|m| *m)
        }
    }

    impl Report for InMemoryReport {}

}
#[cfg(test)]
#[allow(non_snake_case)]
mod day_1_sonar_seep_tests {
    extern crate test_case;
    use std::task::RawWakerVTable;
    use test_case::test_case;

    use crate::day_1_sonar_sweep::in_mem_report::*;
    use crate::day_1_sonar_sweep::{depth_increment_indicator, depth_increment_indicator_sw, Measurement, Report};


    #[test]
    fn sliding_window() {
        let ms = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let report: InMemoryReport = ms.as_slice().into();
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
    //
    // #[test_case(& [5, 6, 5, 4, 3, 2, 1]; "second measurement increases then all decrease")]
    // #[test_case(& [5, 4, 3, 2, 1, 2]; "last measurement increases then all decrease")]
    // #[test_case(& [5, 5, 5, 5, 5, 5, 53]; "last measurement increases then all the same")]
    // fn given_report_with_one_increasing_depth_change__then_answer_is_one(report_slice: &[Measurement]) {
    #[test]
    fn given_report_with_one_increasing_depth_change__then_answer_is_one() {
        let report_slice:&[Measurement] = &[5, 5, 5, 5, 5, 5, 53];
        let answer = depth_increment_indicator(InMemoryReport::from(report_slice));


        assert_eq!(1, answer);
    }

    #[test]
    fn given_report_with_all_increasing_measurements__then_answer_is_len_of_the_report_minus_one() {
        let report = get_report_with_all_increasing_entries();
        let length = report.len() as i32;

        let answer = depth_increment_indicator(report);


        assert_eq!(length - 1, answer as i32);
    }

    fn get_report_with_all_increasing_entries() -> InMemoryReport {
        let mut decreasing_report = get_report_with_decreasing_depths_only();
        decreasing_report.reverse();
        decreasing_report
    }

    fn get_report_with_decreasing_depths_only() -> InMemoryReport {
        vec![10, 9, 8, 7, 6, 5, 4, 3].as_slice().into()
    }

    fn get_report_with_one_entry() -> InMemoryReport {
        vec![123].as_slice().into()
    }

    fn get_empty_report() -> InMemoryReport {
        vec![].as_slice().into()
    }
}