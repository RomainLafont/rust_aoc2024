use std::env;
use std::fs;
use std::str::FromStr;

fn is_report_safe(report: Vec<i32>) -> i32 {
    let mut current_elem = report[0]; 
    let mut safe = 1; 
    if !report.windows(2).all(|w| w[0] < w[1]) && !report.windows(2).all(|w| w[0] > w[1]) {
        // println!("report {:?} not strict", report);
        safe = 0;
    } 
    for elem in &report[1..] {
        if (current_elem - elem).abs() > 3 || (current_elem - elem).abs() < 1 {
            // println!("report {:?} not safe", report);
            safe = 0;
        }
        current_elem = *elem;
    }
    safe
}

fn is_report_safe2(report: Vec<i32>) -> i32 {
    let mut safe = 0;
    let length = report.len();
    if is_report_safe((&report).to_vec()) == 1 {
        safe = 1
    } else {
        for i in 0 .. length {
            if i == 0 {
                if is_report_safe((&report[1..length]).to_vec()) == 1 {    
                    safe = 1
                }
            } else if i == length - 1 {
                if is_report_safe((&report[0..length-1]).to_vec()) == 1 {    
                    safe = 1
                }
            } else {
                if is_report_safe([&report[0..i],&report[i+1..length]].concat()) == 1 {    
                    safe = 1
                }
            }
        }
    }
    safe
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut reports : Vec<Vec<i32>> = Vec::new();
    for line in fs::read_to_string(&filename).unwrap().lines() {
        let report = line.split(&[' ']).filter(|k| !k.is_empty()).collect::<Vec<&str>>().iter().map(|&e| i32::from_str(e).unwrap()).collect();
        reports.push(report);
    }
    let mut nb_safe_reports = 0;
    for report in &reports {
        nb_safe_reports = is_report_safe(report.to_vec()) + nb_safe_reports;
    }
    println!("Part 1 : {} reports safe", nb_safe_reports);

    let mut nb_safe_reports2 = 0;
    for report in &reports {
        nb_safe_reports2 = is_report_safe2(report.to_vec()) + nb_safe_reports2;
    }
    println!("Part 2 : {} reports safe", nb_safe_reports2);
}
