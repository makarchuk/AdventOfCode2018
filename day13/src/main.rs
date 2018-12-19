use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashSet;
use std::fmt;
use std::io::{self, Read};

#[derive(Clone)]
struct Map {
    carts: Vec<Cart>,
    rails: Vec<Vec<Cell>>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.rails.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match self
                    .carts
                    .iter()
                    .filter(|c| c.location == Location { x, y })
                    .next()
                {
                    Some(c) => f.write_str(match c.direction {
                        Direction::Right() => ">",
                        Direction::Left() => "<",
                        Direction::Up() => "^",
                        Direction::Down() => "v",
                    })?,
                    None => f.write_str(match cell {
                        Cell::NoRails() => " ",
                        Cell::BackwardsTurn() => "\\",
                        Cell::Turn() => "/",
                        Cell::Vertical() => "|",
                        Cell::Horizontal() => "-",
                        Cell::Intersection() => "+",
                    })?,
                }
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut carts = vec![];
        let max_len = input
            .split("\n")
            .map(|l| l.len())
            .max()
            .expect("No lines in input");
        let rails = input
            .split("\n")
            .enumerate()
            .map(|(y, s)| {
                let mut row = s
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '-' => Cell::Horizontal(),
                        '|' => Cell::Vertical(),
                        '/' => Cell::Turn(),
                        '\\' => Cell::BackwardsTurn(),
                        '+' => Cell::Intersection(),
                        'v' => {
                            carts.push(Cart {
                                crashed: false,
                                id: carts.len(),
                                direction: Direction::Down(),
                                location: Location { x, y },
                                controller: Controller::new(),
                            });
                            Cell::Vertical()
                        }
                        '^' => {
                            carts.push(Cart {
                                crashed: false,
                                id: carts.len(),
                                direction: Direction::Up(),
                                location: Location { x, y },
                                controller: Controller::new(),
                            });
                            Cell::Vertical()
                        }
                        '>' => {
                            carts.push(Cart {
                                crashed: false,
                                id: carts.len(),
                                direction: Direction::Right(),
                                location: Location { x, y },
                                controller: Controller::new(),
                            });
                            Cell::Horizontal()
                        }
                        '<' => {
                            carts.push(Cart {
                                crashed: false,
                                id: carts.len(),
                                direction: Direction::Left(),
                                location: Location { x, y },
                                controller: Controller::new(),
                            });
                            Cell::Horizontal()
                        }
                        ' ' => Cell::NoRails(),
                        _ => panic!("Unknown char"),
                    })
                    .collect::<Vec<_>>();
                row.resize(max_len, Cell::NoRails());
                row
            })
            .collect();

        Self { rails, carts }
    }

    fn get(&self, loc: &Location) -> Cell {
        self.rails[loc.y][loc.x].clone()
    }

    fn tick(&self) -> (Self, Option<Vec<Location>>) {
        let mut carts = self.ordered_carts().clone();
        let mut collisions = None;
        let mut targets = vec![];
        for (i, cart) in carts.clone().iter().enumerate() {
            let mut new_state_cart = cart.tick(self);
            match carts
                .iter()
                .enumerate()
                .filter(|(_, c)| c.location == new_state_cart.location)
                .next()
            {
                None => (),
                Some((j, c)) => {
                    targets.push(j);
                    new_state_cart.crashed = true;
                    collisions = {
                        match collisions {
                            None => Some(vec![c.location.clone()]),
                            Some(mut locs) => {
                                locs.push(c.location.clone());
                                Some(locs)
                            }
                        }
                    }
                }
            }
            carts[i] = new_state_cart
        }
        for i in targets {
            carts[i].crashed = true
        }
        (
            Map {
                rails: self.rails.clone(),
                carts: carts,
            },
            collisions,
        )
    }

    fn check_collisions(&self) -> Option<Location> {
        let mut hs = HashSet::new();
        for cart in self.ordered_carts().iter() {
            if hs.contains(&(cart.location.x, cart.location.y)) {
                return Some(cart.location.clone());
            } else {
                hs.insert((cart.location.x, cart.location.y).clone());
            }
        }
        None
    }

    fn ordered_carts(&self) -> Vec<Cart> {
        let mut carts_clone = self.carts.clone();
        carts_clone.sort_by_key(|c| c.location.clone());
        carts_clone
    }
}

#[derive(Clone, Debug)]
struct Cart {
    id: usize,
    crashed: bool,
    controller: Controller,
    location: Location,
    direction: Direction,
}

impl Cart {
    fn tick(&self, map: &Map) -> Self {
        let mut clone = self.clone();
        let new_location = clone.direction.tick(&clone.location);
        if new_location == clone.location {
            panic!("We should move!");
        }
        let new_direction = map.get(&new_location).new_direction(&mut clone);
        clone.location = new_location;
        clone.direction = new_direction;
        clone
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
struct Controller {
    last: Option<Turn>,
}

impl Controller {
    fn new() -> Self {
        Self { last: None }
    }
}

#[test]
fn test_controller() {
    let mut c = Controller { last: None };
    let mut results = vec![];
    for _ in 0..10 {
        results.push(c.next().unwrap())
    }
    assert_eq!(
        results,
        vec![
            Turn::Left(),
            Turn::Straight(),
            Turn::Right(),
            Turn::Left(),
            Turn::Straight(),
            Turn::Right(),
            Turn::Left(),
            Turn::Straight(),
            Turn::Right(),
            Turn::Left()
        ]
    )
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

#[derive(Clone, Debug, PartialEq)]
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
                Direction::Up() => Direction::Right(),
                Direction::Right() => Direction::Down(),
                Direction::Down() => Direction::Left(),
            },
        }
    }
}

#[derive(Clone, Debug)]
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
                x: loc.x,
            },
            Direction::Down() => Location {
                y: loc.y + 1,
                x: loc.x,
            },
            Direction::Left() => Location {
                x: loc.x - 1,
                y: loc.y,
            },
            Direction::Right() => Location {
                x: loc.x + 1,
                y: loc.y,
            },
        }
    }
}
#[derive(Clone, Debug)]
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut map = Map::from_str(&input);
    let mut step = 0;
    loop {
        let (new_map, collisions) = map.tick();
        match collisions {
            None => map = new_map,
            Some(locs) => {
                println!("Collision at {},{} at step #{}", locs[0].x, locs[0].y, step);
                let new_carts = new_map
                    .carts
                    .iter()
                    .filter(|c| !c.crashed)
                    .map(|c| c.clone())
                    .collect::<Vec<_>>();
                if new_carts.len() == 1 {
                    let cart_id = new_carts[0].id;
                    let last_cart_location = new_carts
                        .iter()
                        .filter(|c| c.id == cart_id)
                        .next()
                        .unwrap()
                        .location
                        .clone();
                    println!(
                        "Last cart is: {},{}",
                        last_cart_location.x, last_cart_location.y
                    );
                    break;
                }
                map = new_map;
                map.carts = new_carts;
                println!("Carts: {}", map.carts.len());
            }
        }

        step += 1;
    }
}
