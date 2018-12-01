use std::collections::HashSet;
use std::io::{self, Read};
use std::str::Split;

struct Transfromer<'a> {
    previous: i32,
    input: &'a str,
    input_iter: Split<'a, char>,
    repeat: bool,
}

impl<'a> Iterator for Transfromer<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.get_line_from_input() {
            None => None,
            Some(line) => Some(self.get_next_value(line)),
        }
    }
}

impl<'a> Transfromer<'a> {
    fn get_next_value(&mut self, line: &str) -> i32 {
        let inc: i32 = line.trim_matches('+').parse().unwrap();
        self.previous = self.previous + inc;
        self.previous
    }

    fn get_line_from_input(&mut self) -> Option<&'a str> {
        match self.input_iter.next() {
            None => {
                if self.repeat {
                    self.input_iter = self.input.split('\n');
                    return self.input_iter.next();
                }
                None
            }
            Some(line) => Some(line),
        }
    }

    fn new(input: &'a str, repeat: bool) -> Transfromer<'a> {
        Transfromer {
            previous: 0,
            input,
            repeat,
            input_iter: input.split('\n'),
        }
    }

    fn exhaust(&mut self) -> i32 {
        self.last().ok_or("Empty iterator?!").unwrap()
    }

    fn find_repeat(&mut self) -> i32 {
        let mut hs = HashSet::new();
        self.filter(|el| {
            if hs.contains(el) {
                true
            } else {
                hs.insert(*el);
                false
            }
        }).next()
        .ok_or("Empty iterator?!")
        .unwrap()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!(
        "Current frequency is {}",
        Transfromer::new(&input, false).exhaust()
    );
    println!(
        "First repetition is {}",
        Transfromer::new(&input, true).find_repeat()
    );
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
