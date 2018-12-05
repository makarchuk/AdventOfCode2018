use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let compressed = compress(&input);
    println!(
        "Fully compressed is: {}. It's len is: {}",
        compressed,
        compressed.len()
    );
    let chars: HashSet<char> = input.chars().map(|ch| ch.to_ascii_uppercase()).collect();
    println!(
        "Minimal possible len is {}",
        chars
            .iter()
            .map(|excluded| compress(
                &input
                    .chars()
                    .filter(|c| c.to_ascii_uppercase() != *excluded)
                    .collect::<String>(),
            ).len()).min()
            .ok_or("Can't get minimal value")
            .unwrap()
    )
}

fn compress(input: &str) -> String {
    let mut result = input.to_owned();
    loop {
        let compressed = wrap(&result);
        if compressed == result {
            return compressed;
        }
        result = compressed
    }
}

fn wrap(input: &str) -> String {
    let (mut result, last_ch) =
        input
            .chars()
            .fold((String::new(), ' '), |(mut new_str, last_ch), ch| {
                if ch != last_ch && ch.to_ascii_uppercase() == last_ch.to_ascii_uppercase() {
                    (new_str, ' ')
                } else {
                    if last_ch != ' ' {
                        new_str.push(last_ch);
                    }
                    (new_str, ch)
                }
            });
    if last_ch != ' ' {
        result.push(last_ch)
    }
    result
}

// a b a A B
// a b B
// a
