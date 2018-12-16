use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

struct Map {
    carts: Vec<Cart>,
    rails: Vec<Vec<Cell>>,
}

impl Map {
    fn tick(&self) -> Self {
        Map {
            rails: self.rails.clone(),
            carts: self
                .ordered_carts()
                .iter()
                .map(|c| c.tick(&self.rails))
                .collect(),
        }
    }

    fn ordered_carts(&self) -> Vec<Cart> {
        let mut carts_clone = self.carts.clone();
        carts_clone.sort_by_key(|c| c.location.clone());
        carts_clone
    }
}

#[derive(Clone)]
struct Cart {
    controller: Controller,
    location: Location,
    direction: Direction,
}

impl Cart {
    fn tick(&self, rails: &Vec<Vec<Cell>>) -> Self {
        let mut clone = self.clone();
        let new_location = clone.direction.tick(&clone.location);
        let new_direction = rails[clone.location.x][clone.location.y].new_direction(&mut clone);
        clone.location = new_location;
        clone.direction = new_direction;
        clone
    }
}

#[derive(Clone)]
struct Location {
    x: usize,
    y: usize,
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            more_or_less => more_or_less,
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        Some(match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            more_or_less => more_or_less,
        })
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Location {}

#[derive(Clone)]
struct Controller {
    last: Option<Turn>,
}

impl Iterator for Controller {
    type Item = Turn;

    fn next(&mut self) -> Option<Self::Item> {
        let next_turn = match self.last {
            None => Turn::Left(),
            Some(Turn::Left()) => Turn::Straight(),
            Some(Turn::Straight()) => Turn::Right(),
            Some(Turn::Right()) => Turn::Left(),
        };
        self.last = Some(next_turn);
        self.last.clone()
    }
}

#[derive(Clone)]
enum Turn {
    Left(),
    Right(),
    Straight(),
}

impl Turn {
    fn new_direction(&self, dir: &Direction) -> Direction {
        match self {
            Turn::Straight() => dir.clone(),
            Turn::Left() => match dir {
                Direction::Up() => Direction::Left(),
                Direction::Left() => Direction::Down(),
                Direction::Down() => Direction::Right(),
                Direction::Right() => Direction::Up(),
            },
            Turn::Right() => match dir {
                Direction::Left() => Direction::Up(),
                Direction::Down() => Direction::Left(),
                Direction::Right() => Direction::Down(),
                Direction::Up() => Direction::Right(),
            },
        }
    }
}

#[derive(Clone)]
enum Direction {
    Left(),
    Right(),
    Up(),
    Down(),
}

impl Direction {
    fn tick(&self, loc: &Location) -> Location {
        match self {
            Direction::Up() => Location {
                y: loc.y - 1,
                ..*loc
            },
            Direction::Down() => Location {
                y: loc.y + 1,
                ..*loc
            },
            Direction::Left() => Location {
                x: loc.x - 1,
                ..*loc
            },
            Direction::Right() => Location {
                x: loc.x + 1,
                ..*loc
            },
        }
    }
}
#[derive(Clone)]
enum Cell {
    Vertical(),
    Horizontal(),
    Intersection(),
    Turn(),          // "/"
    BackwardsTurn(), // "\"
    NoRails(),
}

impl Cell {
    fn new_direction(&self, cart: &mut Cart) -> Direction {
        match self {
            Cell::Vertical() => cart.direction.clone(),
            Cell::Horizontal() => cart.direction.clone(),
            Cell::Intersection() => cart
                .controller
                .next()
                .unwrap()
                .new_direction(&cart.direction),
            Cell::Turn() => match cart.direction {
                Direction::Right() => Direction::Up(),
                Direction::Up() => Direction::Right(),
                Direction::Left() => Direction::Down(),
                Direction::Down() => Direction::Left(),
            },
            Cell::BackwardsTurn() => match cart.direction {
                Direction::Right() => Direction::Down(),
                Direction::Down() => Direction::Right(),
                Direction::Left() => Direction::Up(),
                Direction::Up() => Direction::Left(),
            },
            Cell::NoRails() => panic!("We should never be here!"),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
