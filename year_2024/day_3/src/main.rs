use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    part_one(&input);
    part_two(&input)
}

fn regex_wrapper(string: &str, regex_pattern: &str) -> Vec<String> {
    let pattern = Regex::new(regex_pattern).unwrap();
    pattern
        .find_iter(string)
        .map(|v| v.as_str().to_owned())
        .collect::<Vec<String>>()
}

fn part_one(input: &str) {
    let res = regex_wrapper(input, r"mul\(\d{1,4}\,\d{1,4}\)")
        .into_iter()
        .map(|r| regex_wrapper(&r, r"\d{1,4}"))
        .into_iter()
        .map(|v| v[0].parse::<i32>().unwrap() * v[1].parse::<i32>().unwrap())
        .sum::<i32>();
    println!("Part 1: {res:?}");
}

fn part_two(input: &str) {
    let res = regex_wrapper(input, r"mul\(\d{1,4}\,\d{1,4}\)|don?\'?t?\(\)");
    let mut sum = 0;
    let mut switch = true;
    for val in res {
        match val.as_str() {
            "do()" => switch = true,
            "don't()" => switch = false,
            _ => {
                if switch {
                    sum += regex_wrapper(&val, r"\d{1,4}")
                        .into_iter()
                        .map(|v| v.parse::<i32>().unwrap())
                        .product::<i32>()
                } else {
                }
            }
        }
    }
    println!("Part 2: {sum:?}")
}
