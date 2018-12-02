use std::collections::HashMap;
use std::io::{self, Read};
use std::str::Chars;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let part1_res =
        input
            .split("\n")
            .map(|line| counts(line.chars()))
            .fold((0, 0), |acc, counter| {
                let counts = counter.iter().fold(HashMap::new(), |mut acc, (_, count)| {
                    acc.entry(count).or_insert(1);
                    acc
                });
                (
                    acc.0 + match counts.get(&2) {
                        None => 0,
                        Some(_) => 1,
                    },
                    acc.1 + match counts.get(&3) {
                        None => 0,
                        Some(_) => 1,
                    },
                )
            });
    println!("Checksum is {}", part1_res.0 * part1_res.1);
    println!(
        "Similar part is {}",
        find_similar(
            input
                .split("\n")
                .map(|line| line.chars().collect())
                .collect()
        )
    );
}

fn counts(i: Chars) -> HashMap<char, i32> {
    i.fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    })
}

fn find_similar(input: Vec<Vec<char>>) -> String {
    for line in &input {
        for similar in &input {
            let matching: Vec<_> = line
                .iter()
                .enumerate()
                .filter(|(i, ch)| **ch == similar[*i])
                .map(|(_, ch)| *ch)
                .collect();
            if matching.len() == line.len() - 1 {
                return matching.iter().collect();
            }
        }
    }
    panic!("Unable to find a match!")
}
