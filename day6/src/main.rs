use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn distance(&self, other: &Point) -> i16 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn closest(&self, others: &Vec<Point>) -> Option<usize> {
        let distances = others
            .iter()
            .enumerate()
            .map(|(i, point)| (i, self.distance(point)))
            .collect::<Vec<_>>();
        let min_distance = distances
            .iter()
            .min_by_key(|(_, d)| d)
            .ok_or("Can't get max value!")
            .unwrap()
            .1;
        let closest_points = distances
            .iter()
            .filter(|(_, d)| *d == min_distance)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        match closest_points.len() {
            1 => Some(*closest_points[0]),
            _ => None,
        }
    }
}

impl FromStr for Point {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates = s.split(", ").collect::<Vec<_>>();
        if coordinates.len() != 2 {
            return Err(Box::new(UserError::new(format!(
                "Invalid string provided: {:?}",
                coordinates
            ))));
        }
        return Ok(Self {
            x: coordinates[0].parse()?,
            y: coordinates[1].parse()?,
        });
    }
}

#[derive(Debug)]
enum Area {
    Infinity(),
    Some(u32),
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let points = get_points_from_str(&input);
    let corner = points.iter().fold(Point { x: 0, y: 0 }, |corner, p| Point {
        x: max(corner.x, p.x),
        y: max(corner.y, p.y),
    });
    part1(&corner, &points);
    part2(&corner, &points);
}

fn part1(corner: &Point, points: &Vec<Point>) {
    let mut counts: HashMap<usize, Area> = HashMap::new();
    for x in 0..=corner.x {
        for y in 0..=corner.y {
            match (Point { x, y }.closest(&points)) {
                None => (),
                Some(ind) => {
                    let entry = counts.entry(ind).or_insert(Area::Some(0));
                    if x == 0 || y == 0 || x == corner.x || y == corner.y {
                        *entry = Area::Infinity()
                    } else {
                        match entry {
                            Area::Infinity() => (),
                            Area::Some(x) => *x += 1,
                        }
                    }
                }
            }
        }
    }
    let biggest_area = counts
        .iter()
        .map(|(_, count)| match count {
            Area::Infinity() => 0,
            Area::Some(x) => *x,
        }).max()
        .ok_or("Can't get biggest area")
        .unwrap();
    println!("Biggest safe area is: {:?}", biggest_area);
}

fn part2(corner: &Point, points: &Vec<Point>) {
    let mut area = 0;
    for x in -1000..=corner.x + 1000 {
        for y in -1000..=corner.y + 1000 {
            let current_point = Point { x, y };
            if points
                .iter()
                .map(|p| current_point.distance(p) as u128)
                .sum::<u128>()
                < 10000
            {
                area += 1
            }
        }
    }
    println!("Area amongst the points is {}", area);
}

fn get_points_from_str(input: &str) -> Vec<Point> {
    input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct UserError {
    reason: String,
}

impl UserError {
    fn new(reason: String) -> Self {
        return Self { reason };
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserError({})", self.reason)
    }
}

impl Error for UserError {}
