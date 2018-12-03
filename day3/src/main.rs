use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

struct Claim {
    id: usize,
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let chunks: Vec<_> = line.split(" ").collect();
        let id = chunks[0].trim_matches('#').parse().unwrap();
        let start: Vec<_> = chunks[2]
            .trim_matches(':')
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let (x0, y0) = (start[0], start[1]);
        let dimensions: Vec<usize> = chunks[3].split("x").map(|x| x.parse().unwrap()).collect();
        let (x1, y1) = (x0 + dimensions[0], y0 + dimensions[1]);
        Ok(Self { id, x0, y0, x1, y1 })
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let claims: Vec<Claim> = input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();
    println!("Overlaps count is {}", find_overlaps(&claims));
    println!("Good claim is {}", find_non_overlaping(&claims));
}

fn fabric<'a>(claims: &'a Vec<Claim>) -> Vec<Vec<Vec<&'a Claim>>> {
    let mut fabric: Vec<Vec<Vec<&Claim>>> = (0..1000)
        .map(|_| (0..1000).map(|_| vec![]).collect())
        .collect();
    claims.iter().for_each(|claim| {
        (claim.x0..claim.x1)
            .for_each(|x| (claim.y0..claim.y1).for_each(|y| fabric[x][y].push(&claim)))
    });
    fabric
}

fn find_overlaps(claims: &Vec<Claim>) -> usize {
    fabric(claims)
        .iter()
        .map(|row| row.iter().filter(|claims| claims.len() > 1).count())
        .sum()
}

fn find_non_overlaping(claims: &Vec<Claim>) -> usize {
    let mut bad_claims = HashSet::new();
    fabric(claims).iter().for_each(|row| {
        row.iter()
            .filter(|claims| claims.len() > 1)
            .for_each(|claims| {
                claims.iter().for_each(|claim| {
                    bad_claims.insert(claim.id);
                })
            })
    });
    claims
        .iter()
        .filter(|c| !bad_claims.contains(&c.id))
        .next()
        .ok_or("No good claims found")
        .unwrap()
        .id
}
