extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::str::FromStr;

struct Schedule {
    naps: Vec<Nap>,
}

impl FromStr for Schedule {
    type Err = Box<Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut naps = vec![];
        let mut input_vector: Vec<_> = input.split("\n").collect();
        input_vector.sort();
        let mut current_guard: u16 = 0;
        let mut nap_start = Utc::now().naive_local();
        input_vector.iter().for_each(|line| {
            let chunks: Vec<_> = line.trim_matches('[').split("] ").collect();
            let ch = chunks[0].replace("1518", "2018");
            let ts = NaiveDateTime::parse_from_str(&ch, "%Y-%m-%d %H:%M").unwrap();
            match chunks[1] {
                "falls asleep" => nap_start = ts,
                "wakes up" => naps.push(Nap {
                    id: current_guard,
                    start: nap_start,
                    end: ts,
                }),
                st => {
                    let chunks: Vec<_> = st.split(" ").collect();
                    current_guard = chunks[1].trim_matches('#').parse().unwrap();
                }
            }
        });
        return Ok(Self { naps });
    }
}

impl Schedule {
    fn most_consisten_sleeper(&self) -> (u16, u8) {
        let mut sleep_minutes: HashMap<(u16, u8), u8> = HashMap::new();
        for nap in &self.naps {
            for minute in nap.minutes_of_sleep() {
                let entry = sleep_minutes.entry((nap.id, minute)).or_insert(0);
                *entry += 1;
            }
        }
        *sleep_minutes
            .iter()
            .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
            .ok_or("Can't find max")
            .unwrap()
            .0
    }

    fn best_sleeper(&self) -> (u16, u8) {
        let mut sleep_minutes: HashMap<u16, Vec<u8>> = HashMap::new();
        for nap in &self.naps {
            for minute in nap.minutes_of_sleep() {
                let entry = sleep_minutes.entry(nap.id).or_insert(vec![]);
                (*entry).push(minute);
            }
        }
        let res = sleep_minutes
            .iter()
            .max_by(|(_, xminutes), (_, yminutes)| xminutes.len().cmp(&yminutes.len()))
            .ok_or("Can't get max")
            .unwrap()
            .clone();
        let best_sleepers_minutes =
            res.1
                .iter()
                .fold(HashMap::new(), |mut acc: HashMap<u8, u16>, minute| {
                    {
                        let entry = acc.entry(*minute).or_insert(0);
                        *entry += 1;
                    }
                    acc
                });
        let best_minute = best_sleepers_minutes
            .iter()
            .max_by(|x, y| x.1.cmp(&y.1))
            .ok_or("Can't get max")
            .unwrap()
            .0;
        return (*res.0, *best_minute);
    }
}

#[derive(Debug)]
struct Nap {
    id: u16,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl Nap {
    fn minutes_of_sleep(&self) -> Vec<u8> {
        let mut current = self.start.clone();
        let mut result = vec![];
        while current < self.end {
            result.push(current.time().format("%M").to_string().parse().unwrap());
            current = current + Duration::minutes(1)
        }
        result
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let sched: Schedule = input.parse().unwrap();
    let part1_res = sched.best_sleeper();
    println!(
        "Part1 {:?} =>  {}",
        part1_res,
        part1_res.0 as u32 * part1_res.1 as u32
    );
    let part2_res = sched.most_consisten_sleeper();
    println!(
        "Part2 {:?} =>  {}",
        part2_res,
        part2_res.0 as u32 * part2_res.1 as u32
    );
}
