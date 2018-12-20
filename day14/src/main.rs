use std::io::{self, Read};

struct Book {
    recipies: Vec<u8>,
    elfs: Vec<usize>,
}

impl Book {
    //Construct recipies with required capacity to avoid reallocations
    fn new(size: usize) -> Self {
        let mut recipies = Vec::with_capacity(size);
        recipies.push(3);
        recipies.push(7);
        Self {
            recipies,
            elfs: vec![0, 1],
        }
    }

    fn tick(&mut self) {
        let sum: u8 = self.elfs.iter().map(|e| self.recipies[*e]).sum();
        let new_chars = sum
            .to_string()
            .chars()
            .map(|ch| ch.to_string().parse().unwrap())
            .collect::<Vec<_>>();
        self.recipies.extend(new_chars.iter());

        self.elfs = self
            .elfs
            .iter()
            .map(|e| (*e + self.recipies[*e] as usize + 1) % self.recipies.len())
            .collect();
    }

    pub fn wait_for_len(&mut self, len: usize) -> Vec<u8> {
        loop {
            if self.recipies.len() >= len + 10 {
                return (0..10).map(|i| self.recipies[i + len]).collect();
            }
            self.tick();
        }
    }

    pub fn wait_for_sequence(&mut self, seq: &str) -> usize {
        let seq_as_vec: Vec<u8> = seq
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        loop {
            if self.recipies.len() < seq.len() + 1 {
                self.tick();
                continue;
            }
            for pos in (self.recipies.len() - seq.len() - 1)..=(self.recipies.len() - seq.len()) {
                if seq_as_vec
                    .iter()
                    .enumerate()
                    .all(|(i, num)| self.recipies[i + pos] == *num)
                {
                    return pos;
                }
            }
            self.tick();
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let len: usize = input.parse().unwrap();
    let mut book = Book::new(len + 11);
    println!(
        "10 scores are {}",
        book.wait_for_len(len)
            .iter()
            .map(|r| r.to_string())
            .fold(String::new(), |acc, token| acc + &token)
    );
    let mut book = Book::new(100000000);
    println!(
        "Position before {} is {}",
        input,
        book.wait_for_sequence(&input)
    )
}
