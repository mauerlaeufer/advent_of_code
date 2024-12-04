use std::fs::{self};
fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let mut safe_reports_part_1 = 0;
    for line in input.lines().to_owned() {
        let v: Vec<i8> = line
            .to_string()
            .split_whitespace()
            .filter_map(|s| s.parse::<i8>().ok())
            .collect();
        let mut v_rev = v.clone();
        v_rev.reverse();
        if v.is_sorted() || v_rev.is_sorted() {
            if !v
                .windows(2)
                .any(|pair| (pair[0] - pair[1]).abs() > 3 || (pair[0] - pair[1]).abs() == 0)
            {
                safe_reports_part_1 += 1;
            } else {
            }
        };
    }

    let mut safe_reports = 0;
    let mut original_cases = 0;
    // 717
    for line in input.lines().to_owned() {
        let v: Vec<i8> = line
            .to_string()
            .split_whitespace()
            .filter_map(|s| s.parse::<i8>().ok())
            .collect();
        let mut v_rev = v.clone();
        v_rev.reverse();

        if v.is_sorted() || v_rev.is_sorted() {
            if !v
                .windows(2)
                .any(|pair| (pair[0] - pair[1]).abs() > 3 || (pair[0] - pair[1]).abs() == 0)
            {
                safe_reports += 1;
                original_cases += 1;
                continue;
            }
        };

        for i in 0..(v.len()) {
            let mut copy_normal = v.clone();
            let mut copy_rev = v.clone();
            copy_rev.remove(i);
            copy_rev.reverse();
            copy_normal.remove(i);

            if copy_normal.is_sorted() || copy_rev.is_sorted() {
                if !copy_normal
                    .windows(2)
                    .any(|pair| (pair[0] - pair[1]).abs() > 3 || (pair[0] - pair[1]).abs() == 0)
                {
                    safe_reports += 1;
                    break;
                }
            };
        }
    }
    println!("Safe Reports 1: {safe_reports_part_1}");
    println!("Safe Reports 2: {safe_reports} - original: {original_cases}");
}
