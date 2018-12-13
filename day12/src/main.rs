use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Write};
use std::io::{self, Read};

#[derive(Clone)]
struct Rule {
    result: bool,
    mask: Vec<bool>,
}

impl Rule {
    fn check(&self, state: &State, index: i32) -> Option<bool> {
        if self.mask.iter().enumerate().all(|(i, val)| {
            let inc = i as i32 - 2;
            state.get(index + inc) == *val
        }) {
            Some(self.result)
        } else {
            None
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in self.mask.iter() {
            f.write_char(match v {
                true => '#',
                false => '.',
            })?
        }
        f.write_str(" => ")?;
        f.write_char(match self.result {
            true => '#',
            false => '.',
        })
    }
}

#[derive(Clone)]
struct State {
    plants: HashSet<i32>,
    rules: Vec<Rule>,
}

impl State {
    fn create(input: &str) -> Self {
        let mut iter = input.split("\n");
        let plants = iter
            .next()
            .unwrap()
            .replace("initial state: ", "")
            .char_indices()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i as i32)
            .collect();
        iter.next().unwrap();
        Self {
            plants,
            rules: iter
                .map(|line| {
                    let chunks = line.split(" => ").collect::<Vec<_>>();
                    let result = chunks[1] == "#";
                    let mask = chunks[0].chars().map(|c| c == '#').collect::<Vec<_>>();
                    if mask.len() != 5 {
                        panic!("Invalid mask in line: {}", line)
                    }
                    Rule { result, mask }
                })
                .collect(),
        }
    }

    //return maximum possible for the next generation
    //it's current generation range +- 2 plants
    fn range(&self) -> (i32, i32) {
        let current_range = self.plants.iter().fold((0, 0), |(min, max), current| {
            (cmp::min(min, *current), cmp::max(max, *current))
        });
        return (current_range.0 - 2, current_range.1 + 2);
    }

    fn next(&self) -> Self {
        let possible_range = self.range();
        Self {
            rules: self.rules.clone(),
            plants: (possible_range.0..=possible_range.1)
                .filter(|ind| {
                    self.rules
                        .iter()
                        .map(|r| r.check(self, *ind))
                        .filter(|check| check.is_some())
                        .next()
                        .unwrap() //unwrap "next"
                        .unwrap() //unwrap "check"
                })
                .collect(),
        }
    }

    fn get(&self, index: i32) -> bool {
        return self.plants.contains(&index);
    }

    //Move state to 0 point to compare moved states
    fn base_0(&self) -> HashSet<i32> {
        let min = self.plants.iter().min().unwrap();
        self.plants.iter().map(|p| p - min).collect()
    }

    fn cache(&self) -> CachedValue {
        CachedValue {
            state: self.clone(),
            base_0: self.base_0(),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let range = self.range();
        for i in range.0..=range.1 {
            f.write_char(match self.get(i) {
                true => '#',
                false => '.',
            })?
        }
        Ok(())
    }
}

struct CachedValue {
    state: State,
    base_0: HashSet<i32>,
}

struct CacheStore {
    states: Vec<CachedValue>,
}

impl CacheStore {
    fn push(&mut self, state: &State) {
        self.states.push(state.cache());
    }

    fn find(&self, state: &State) -> Option<i128> {
        let cached = state.cache();
        self.states
            .iter()
            .enumerate()
            .filter(|(_, cached_value)| cached_value.base_0 == cached.base_0)
            .map(|(i, _)| i as i128)
            .next()
    }

    fn get_by_index(&self, i: usize) -> State {
        self.states[i].state.clone()
    }
}

fn find_at_5bil(state: State) -> i128 {
    let FIVE_BIL = 50000000000_i128;
    let mut state = state;
    let mut store = CacheStore { states: vec![] };
    store.push(&state);
    for i in 1..=160 {
        state = state.next();
        match store.find(&state) {
            None => store.push(&state),
            Some(min) => {
                println!("Found loop {} - {}", min, i);
                if (FIVE_BIL - min) % (i - min) == 0 {
                    let sum_at_min: i128 = store
                        .get_by_index(min as usize)
                        .plants
                        .iter()
                        .map(|i| *i as i128)
                        .sum();
                    let current_sum: i128 = state.plants.iter().map(|i| *i as i128).sum();
                    return sum_at_min + (current_sum - sum_at_min) * (FIVE_BIL - min) / (i - min);
                }
                store.push(&state)
            }
        }
    }
    state.plants.iter().map(|i| *i as i128).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let original_state = State::create(&input);
    let mut state = original_state.clone();
    for _ in 1..=20 {
        state = state.next();
    }
    let sum_of_pots: i32 = state.plants.iter().sum();
    println!("Sum of pots in step #20 is {}", sum_of_pots);
    println!(
        "Sum of pots in 5 bil is  {}",
        find_at_5bil(original_state.clone())
    );
}
