use std::collections::HashMap;
use std::io::{self, Read};

struct Game {
    stones: u32,
    players: u32,
    scores: HashMap<u32, u32>,
    circle: Vec<u32>,
    current: usize,
}

impl Game {
    fn new(stones: u32, players: u32) -> Self {
        //It's a bit too much but it's simple and helps to avoid reallocations
        let mut circle = Vec::with_capacity(stones as usize);
        circle.push(0);
        Self {
            scores: HashMap::new(),
            current: 0,
            circle,
            stones,
            players,
        }
    }

    fn position_at(&self, offset: i32) -> usize {
        //Positive numbers mean clocwise offset. Negative -- counterclockwise
        let mut index = self.current as i32 + offset;
        if index < 0 {
            index += self.circle.len() as i32
        }
        (index as usize) % self.circle.len()
    }

    fn insert_position(&self) -> usize {
        self.position_at(2)
    }

    fn step(&mut self, stone: u32, player: u32) {
        match stone % 23 {
            0 => {
                let extract_index = self.position_at(-7);
                let score_inc = stone + self.circle[extract_index];
                {
                    let score = self.scores.entry(player).or_insert(0);
                    *score += score_inc
                }
                self.current = extract_index;
                self.circle.remove(extract_index);
                //Just in case we removed the last elemen of the circle
                self.current = self.position_at(0);
            }
            _ => {
                let insert_index = self.insert_position();
                self.circle.insert(insert_index, stone);
                self.current = insert_index;
            }
        }
    }

    fn play(&mut self) {
        let mut player = 0;
        for i in 1..=self.stones {
            self.step(i, player);
            if i % 10000 == 0 {
                println!("{}: [{}] {:?}", i, player, self.circle.len());
            }
            player = (player + 1) % self.players;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let chunks = input.split(" ").collect::<Vec<_>>();
    let mut game = Game::new(chunks[6].parse().unwrap(), chunks[0].parse().unwrap());
    game.play();

    println!(
        "Top score is {}",
        game.scores.values().max().ok_or("No max score?").unwrap()
    );

    let mut game = Game::new(
        chunks[6].parse::<u32>().unwrap() * 100,
        chunks[0].parse().unwrap(),
    );

    game.play();
    println!(
        "Top score is {}",
        game.scores.values().max().ok_or("No max score?").unwrap()
    );
}
