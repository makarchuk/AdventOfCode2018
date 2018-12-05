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
    let chars: HashSet<char> = compressed
        .chars()
        .map(|ch| ch.to_ascii_uppercase())
        .collect();
    println!(
        "Minimal possible len is {}",
        chars
            .iter()
            .map(|excluded| compress(
                &compressed
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
    let (mut result, tail) = input.chars().fold(
        (String::new(), None),
        |(mut new_str, previous), ch| match previous {
            None => (new_str, Some(ch)),
            Some(last_ch) => {
                if last_ch.to_ascii_uppercase() == ch.to_ascii_uppercase() && last_ch != ch {
                    (new_str, None)
                } else {
                    new_str.push(last_ch);
                    (new_str, Some(ch))
                }
            }
        },
    );
    if let Some(last_ch) = tail {
        result.push(last_ch)
    }
    result
}

// a b a A B
// a b B
// a
