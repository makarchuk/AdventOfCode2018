use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let frequency: i32 = input.split("\n").fold(0, |current, line| {
        let inc: i32 = line.trim_matches('+').parse().unwrap();
        current + inc
    });
    println!("Current frequency is {}", frequency);
    println!("First repetition is {}", find_repeat(&input));
}

fn find_repeat(input: &str) -> i32 {
    let mut hs: HashSet<i32> = HashSet::new();
    hs.insert(0);
    let mut current = 0;
    loop {
        for line in input.split("\n") {
            let inc: i32 = line.trim_matches('+').parse().unwrap();
            let new_freq = current + inc;
            if hs.contains(&new_freq) {
                return new_freq;
            }
            hs.insert(new_freq);
            current = new_freq;
        }
    }
}
