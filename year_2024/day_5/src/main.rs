use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Page {
    index: usize,
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = ITEM_ORDER.lock().unwrap();
        if let Some(values) = ordering.get(&self.index) {
            if values.contains(&other.index) {
                return Ordering::Less;
            }
        }
        if let Some(values) = ordering.get(&other.index) {
            // println!("----- Inside{self:?} --- {other:?}");
            if values.contains(&self.index) {
                return Ordering::Greater;
            }
        }
        // println!("SERVERE ERROR: Is {self:?} --- {other:?}");
        self.index.cmp(&other.index) // Default. Not sure if this is smart.
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // Delegate to Ord's cmp
    }
}

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ITEM_ORDER: Mutex<HashMap<usize, Vec<usize>>> = Mutex::new(HashMap::new());
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let mut sections = input.split("\n\n");
    let rules = sections.next().unwrap();
    println!("Hello, world! {rules}");
    {
        let mut ordering = ITEM_ORDER.lock().unwrap();
        for line in rules.lines() {
            if let Some((key, value)) = line.split_once('|') {
                let key: usize = key.parse().expect("Invalid key");
                let value: usize = value.parse().expect("Invalid value");
                ordering.entry(key).or_insert_with(Vec::new).push(value);
            }
        }
        println!("Ordering: {ordering:?}");
    }
    let print_orders = sections.next().unwrap();
    let mut count_already_sorted = 0;
    let mut count_freshly_sorted = 0;
    for line in print_orders.lines() {
        let mut v: Vec<Page> = line
            .split(',') // Split the string by commas
            .map(|s| s.trim()) // Trim any whitespace
            .map(|s| s.parse().expect("Invalid number")) // Parse into usize
            //.collect::<Vec<usize>>()
            .map(|num| Page { index: num })
            .collect(); // Collect into a Vec
        let sorted = v.is_sorted();
        if sorted {
            count_already_sorted += v.get(v.len() / 2).unwrap().index;
        } else {
            v.sort();

            count_freshly_sorted += v.get(v.len() / 2).unwrap().index;
        }
    }

    println!("Part 1: {count_already_sorted:?}");
    println!("Part 2: {count_freshly_sorted:?}");
}
