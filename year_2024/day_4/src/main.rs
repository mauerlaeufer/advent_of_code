use std::fs;

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    part_one(&input);
    part_two(&input);
    println!("Hello, world!");
}

fn part_one(input: &str) {
    let mut counter = 0;
    let forward = "XMAS";
    let backwards = "SAMX";
    let forward_tup = ('X', 'M', 'A', 'S');
    let backwards_tup = ('S', 'A', 'M', 'X');

    let arr: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Grid is square for some reason.
    let size = arr.len() - 1;

    // top-left to bottom-right
    for col in 0..(size - 2) {
        let mut row = 0;
        while row + 2 < size {
            let tup = (
                arr[row][col],
                arr[row + 1][col + 1],
                arr[row + 2][col + 2],
                arr[row + 3][col + 3],
            );
            if tup == forward_tup || tup == backwards_tup {
                counter += 1;
            }
            row += 1;
        }
    }

    // top-right to bottom-left
    for col in (3..(size + 1)).rev() {
        let mut row = 0;
        while row + 2 < size {
            let tup = (
                arr[row][col],
                arr[row + 1][col - 1],
                arr[row + 2][col - 2],
                arr[row + 3][col - 3],
            );
            if tup == forward_tup || tup == backwards_tup {
                counter += 1;
            }
            row += 1;
        }
    }

    // Vertical
    for col in 0..(size + 1) {
        let mut row = 0;
        while row + 2 < size {
            let tup = (
                arr[row][col],
                arr[row + 1][col],
                arr[row + 2][col],
                arr[row + 3][col],
            );
            if tup == forward_tup || tup == backwards_tup {
                counter += 1;
            }
            row += 1;
        }
    }

    // Horizontal
    for line in input.lines() {
        for i in 0..(line.len() - 3) {
            let win = &line[i..(i + 4)];
            if win == forward || win == backwards {
                counter += 1;
            }
        }
    }
    println!("Part One: {}", counter);
}

fn part_two(input: &str) {
    let arr: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Grid is square for some reason.
    let size = arr.len();
    let mut counter = 0;

    let forward_tup = ('M', 'A', 'S');
    let backward_tup = ('S', 'A', 'M');

    for col in 0..(size - 2) {
        for row in 0..(size - 2) {
            let a = (arr[col][row], arr[col + 1][row + 1], arr[col + 2][row + 2]);
            let b = (arr[col + 2][row], arr[col + 1][row + 1], arr[col][row + 2]);
            if (a == forward_tup || a == backward_tup) && (b == forward_tup || b == backward_tup) {
                counter += 1;
            }
        }
    }
    println!("Part 2: {counter}");
}
