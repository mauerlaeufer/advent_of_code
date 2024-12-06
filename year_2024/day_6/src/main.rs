use std::{collections::HashSet, fs};

fn main() {
    //let input = fs::read_to_string("./input");
    let input = fs::read_to_string("./input").unwrap();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut guard_pos: (i32, i32) = (0, 0);
    let mut dir: (i32, i32) = (-1, 0);
    let mut counter = 1;
    let x_size = input.lines().next().unwrap().len() - 1;
    let y_size = input.lines().count();
    let mut field: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.to_string().chars().collect())
        .collect();
    for x in 0..x_size {
        for y in 0..y_size {
            if field[x][y] == '^' {
                guard_pos = (x as i32, y as i32);
                field[x][y] = 'X';
                break;
            }
        }
    }

    loop {
        let next_x = dir.0 + guard_pos.0;
        let next_y = dir.1 + guard_pos.1;
        if next_x < 0 || next_x > x_size as i32 || next_y < 0 || next_y > y_size as i32 {
            break;
        }
        let next_element = &mut field[(next_x) as usize][(next_y) as usize];
        match next_element {
            '.' => {
                counter += 1;
                guard_pos = (next_x, next_y);
                *next_element = 'X';
            }
            'X' => {
                guard_pos = (next_x, next_y);
            }
            '#' => match dir {
                (1, 0) => dir = (0, -1),
                (0, 1) => dir = (1, 0),
                (-1, 0) => dir = (0, 1),
                (0, -1) => dir = (-1, 0),
                _ => unreachable!(":))"),
            },
            _ => unreachable!(":)"),
        }
    }
    println!("Part 1: {counter}");
}

fn part_two(input: &str) {
    let mut guard_pos: (i32, i32) = (0, 0);
    let mut dir: (i32, i32);
    let mut counter = 0;
    let x_size = input.lines().next().unwrap().len() - 1;
    let y_size = input.lines().count() - 1;
    let mut field: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.to_string().chars().collect())
        .collect();
    for x in 0..x_size {
        for y in 0..y_size {
            if field[x][y] == '^' {
                guard_pos = (x as i32, y as i32);
                field[x][y] = '.';
                break;
            }
        }
    }

    for x in 0..x_size {
        for y in 0..y_size {
            let mut guard_pos_clone = guard_pos.clone();
            dir = (-1, 0);
            let mut h_set: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
            loop {
                c += 1;
                let next_x = dir.0 + guard_pos_clone.0;
                let next_y = dir.1 + guard_pos_clone.1;
                if next_x < 0
                    || next_x > x_size as i32
                    || next_y < 0
                    || next_y > y_size as i32
                    || (x as i32, y as i32) == guard_pos
                {
                    break;
                }
                let next_element;
                if x as i32 == next_x && y as i32 == next_y {
                    next_element = '#';
                } else {
                    next_element = field[(next_x) as usize][(next_y) as usize];
                }
                match next_element {
                    '.' => {
                        guard_pos_clone = (next_x, next_y);
                    }
                    '#' => match dir {
                        (1, 0) => dir = (0, -1),
                        (0, 1) => dir = (1, 0),
                        (-1, 0) => dir = (0, 1),
                        (0, -1) => dir = (-1, 0),
                        _ => unreachable!(":))"),
                    },
                    _ => unreachable!(":)"),
                }
                if h_set.contains(&((next_x, next_y), (dir.0, dir.1))) {
                    counter += 1;
                    break;
                }
                h_set.insert(((next_x, next_y), (dir.0, dir.1)));
            }
        }
    }
    println!("Part 2: {counter}");
}
