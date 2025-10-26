use std::fs;

fn is_valid_line(report: &str) -> i32{
    let mut count = 0;
    for line in report.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|a| a.parse::<i32>().ok())
            .collect();
        // println!("{:?}", report);
        // need to set true or false
        if is_true(&report) {
            count += 1;
        } else {
            if check_false_again(report) {
                count += 1;
            }
        }
    }
    count
}

fn is_true(report: &Vec<i32>) -> bool {
    // set direction
    let first_diff = report[1] - report[0];
    if first_diff == 0 || first_diff.abs() > 3 {
        return false;
    }

    let increasing = first_diff > 0;
    
    for window in report.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 || diff.abs() > 3 {
            return false
        }
        if (diff > 0) != increasing {
            return false;
        }
    }
    // println!("each report{:?}", report);
    true
}

fn check_false_again (report: Vec<i32>) -> bool {
    report.iter().enumerate().any(|(i, _)| {
        let adjusted_report: Vec<_> = report.iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, &x)| x)
            .collect();
        
        is_true(&adjusted_report)
    })
}

fn main() {
    let reports = fs::read_to_string("input.txt").expect("can't read");
    // println!("{}", reports);
    
    println!("valid reports: {}", is_valid_line(&reports));

}
