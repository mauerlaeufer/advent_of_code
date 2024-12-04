use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let mut arr = content.split_ascii_whitespace();
    let mut par = false;
    let mut arr_l: Vec<i32> = vec![];
    let mut arr_r: Vec<i32> = vec![];
    while let Some(num) = arr.next() {
        if par {
            arr_l.push(num.parse().unwrap());
        } else {
            arr_r.push(num.parse().unwrap());
        }
        par = !par;
    }
    arr_l.sort();
    arr_r.sort();
    let mut diff = 0;
    for i in 0..arr_l.len() {
        diff += i32::abs(arr_r[i] - arr_l[i]);
    }
    println!("Part 1: {}", diff);
    let mut map = HashMap::new();
    for element in arr_r {
        *map.entry(element).or_insert(0) += 1;
    }
    let mut diff_2 = 0;
    for element in arr_l {
        diff_2 += element * map.get(&element).unwrap_or(&0);
    }
    println!("Part 2: {}", diff_2);
}
