use std::fs;

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let mut counter = 0;
    let mut con_counter = 0;
    for line in input.lines() {
        let mut split = line.split(":");
        let res = split.next().unwrap().parse::<usize>().unwrap();
        let vals: Vec<usize> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        if solve_rec(&vals, 0, res) {
            counter += res;
        }
        if solve_rec_concat(&vals, 0, res) {
            con_counter += res;
        }
    }
    println!("Part 1: {counter}");
    println!("Part 1: {con_counter}");
}

fn solve_rec(vals: &[usize], current: usize, target: usize) -> bool {
    if vals.is_empty() {
        return current == target;
    } else if current > target {
        return false;
    } else {
        let add = vals[0] + current;
        let mul = vals[0] * current;
        return solve_rec(&vals[1..], add, target) || solve_rec(&vals[1..], mul, target);
    }
}

fn solve_rec_concat(vals: &[usize], current: usize, target: usize) -> bool {
    if vals.is_empty() {
        return current == target;
    } else if current > target {
        return false;
    } else {
        let add = vals[0] + current;
        let concat = format!("{:?}{:?}", current, vals[0])
            .parse::<usize>()
            .unwrap();
        let mul = vals[0] * current;
        return solve_rec_concat(&vals[1..], add, target)
            || solve_rec_concat(&vals[1..], mul, target)
            || solve_rec_concat(&vals[1..], concat, target);
    }
}
