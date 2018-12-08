use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let mut f = File::open(&args[1]).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("Frequency: {}", part_1(&contents));
    println!("First reached twice: {}", part_2(&contents).unwrap());

}

fn part_1(input: &str) -> i32 {
    input.lines().map(|s| s.parse::<i32>().unwrap()).sum()
}


fn part_2(input: &str) -> Option<i32> {
    let num_iterator = input.lines().map(|s| s.parse::<i32>().unwrap());
    let mut seen = HashSet::new();
    let mut acc = 0;
    for i in num_iterator.cycle() {
        acc += i;
        if !seen.insert(acc) {
            return Some(acc)
        }
    }
    None
}
