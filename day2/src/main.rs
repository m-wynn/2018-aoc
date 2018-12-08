use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let mut f = File::open(&args[1]).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("Checksum: {}", part_1(&contents));
    println!("Common Letters: {}", part_2(&contents));
}

fn part_1(input: &str) -> u16 {
    let how_it_counts = |s: &str| {
        s.chars()
            .sorted()
            .into_iter()
            .group_by(|c| *c)
            .into_iter()
            .map(|(_, grp)| grp.count())
            .fold((false, false), |(has_two, has_three), count| {
                (has_two || count == 2, has_three || count == 3)
            })
    };

    let (twos, threes) = input.lines().map(how_it_counts).fold(
        (0, 0),
        |(twos, threes), (has_two, has_three): (bool, bool)| {
            (twos + has_two as u8, threes + has_three as u8)
        },
    );

    u16::from(twos) * u16::from(threes)
}

fn part_2(input: &str) -> String {
    let diff_strings = |x: &str, y: &str| {
        let mut diff_count = 0;
        let mut same = String::with_capacity(x.len());
        for (a, b) in x.chars().zip(y.chars()) {
            if a == b {
                same.push(a);
            } else {
                diff_count += 1;
            }
            if diff_count > 1 {
                return None;
            }
        }
        Some(same)
    };

    input
        .lines()
        .tuple_combinations()
        .filter_map(|(x, y)| diff_strings(x, y))
        .next()
        .unwrap()
}
