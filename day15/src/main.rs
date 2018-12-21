use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Game {
    units: Vec<RefCell<Unit>>,
    map: Vec<Vec<Node>>,
    dimensions: (usize, usize),
}

impl Game {
    fn from_string(input: &str) -> Self {
        let mut units = vec![];

        let map = input
            .split("\n")
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, ch)| match ch {
                        '#' => Node::Wall,
                        '.' => Node::Open,
                        'G' => {
                            units.push(Unit::rc(x, y, UnitType::Goblin));
                            Node::Open
                        }
                        'E' => {
                            units.push(Unit::rc(x, y, UnitType::Elf));
                            Node::Open
                        }
                        _ => panic!("Unknwn char! {}", ch),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let dimensions = (map[0].len(), map.len());
        Self {
            map,
            units,
            dimensions,
        }
    }

    fn get(&self, x: usize, y: usize) -> &Node {
        &self.map[y][x]
    }

    fn open(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            Node::Wall => false,
            Node::Open => true,
        }
    }

    fn get_unit(&self, x: usize, y: usize) -> Option<RefCell<Unit>> {
        match self
            .units
            .iter()
            .filter(|u| u.borrow().x == x && u.borrow().y == y)
            .next()
        {
            None => None,
            Some(u) => {
                //Ignore the dead!
                match u.borrow().is_dead() {
                    true => None,
                    false => Some(u.clone()),
                }
            }
        }
    }

    fn can_visit(&self, x: usize, y: usize) -> bool {
        self.open(x, y) && self.get_unit(x, y).is_none()
    }

    fn route_to_enemy(&self, x: usize, y: usize, ut: &UnitType) -> Option<Vec<(usize, usize)>> {
        let mut routes = vec![vec![(x, y)]];
        let mut visited_before: HashSet<(usize, usize)> = HashSet::new();
        visited_before.insert((x, y));
        loop {
            let mut new_routes = routes
                .iter()
                .map(|route| {
                    self.continue_route(&route)
                        .iter()
                        .filter(|new_route| {
                            let head = new_route[new_route.len()];
                            //Discard visited before and walls
                            visited_before.contains(&head) && self.open(head.0, head.1)
                        })
                        .map(|v| v.clone())
                        .collect::<Vec<_>>()
                })
                .fold(vec![], |mut all_routes, some_routes| {
                    all_routes.extend(some_routes.iter().map(|v| v.clone()));
                    all_routes
                });
            let enemy_route = new_routes
                .iter()
                .filter(|route| {
                    let head = route[route.len()];
                    match self.get_unit(head.0, head.1) {
                        None => false,
                        Some(u) => u.borrow().unit_type == *ut,
                    }
                })
                .next();
            if let Some(r) = enemy_route {
                return Some(r.clone());
            }
            visited_before.extend(new_routes.iter().map(|route| route[route.len()]));
            new_routes.retain(|route| {
                let head = route[route.len()];
                self.get_unit(head.0, head.1).is_none()
            });
            if new_routes.len() == 0 {
                return None;
            }
            routes = new_routes
        }
    }

    fn continue_route(&self, route: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
        let top = route[route.len()];
        self.around(top.0, top.1)
            .iter()
            .filter(|(x, y)| {
                if route.len() > 1 {
                    let prev = route[route.len() - 1];
                    prev.0 != *x && prev.1 != *y
                } else {
                    true
                }
            })
            .map(|(x, y)| {
                let mut clone = route.clone();
                clone.push((*x, *y));
                clone
            })
            .collect()
    }

    fn around(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = vec![];
        //Order matters here. It's Reading order
        if y != 0 {
            result.push((x, y - 1))
        }
        if x != 0 {
            result.push((x - 1, y))
        }
        if x != self.dimensions.0 {
            result.push((x + 1, y))
        }
        if y != self.dimensions.1 {
            result.push((x, y + 1))
        }
        result
    }

    fn sort_units(&mut self) {
        self.units
            .sort_by(|u, other| match u.borrow().y.cmp(&other.borrow().y) {
                Ordering::Equal => u.borrow().x.cmp(&other.borrow().x),
                more_or_less => more_or_less,
            });
    }

    fn tick(&mut self) {
        self.sort_units();
        self.units.retain(|u| !u.borrow().is_dead());

        for unit in self.units.iter() {
            let optional_route = self.route_to_enemy(
                unit.borrow().x,
                unit.borrow().y,
                &unit.borrow().enemy_type(),
            );
            //immediate proximity!
            match optional_route {
                None => {
                    println!("Got nothing to do!");
                }
                Some(route) => {
                    let head = route[route.len()];
                    let tail = route[0];
                    if route.len() == 2 {
                        self.attack(unit, &self.get_unit(head.0, head.1).unwrap())
                    } else {
                        self.step(unit, tail.0, tail.1)
                    }
                }
            }
        }
    }

    fn step(&self, unit: &RefCell<Unit>, x: usize, y: usize) {
        let mut borrowed = unit.borrow_mut();
        borrowed.x = x;
        borrowed.y = y;
    }

    fn attack(&self, attacker: &RefCell<Unit>, target: &RefCell<Unit>) {
        target.borrow_mut().hp = attacker.borrow().attack
    }
}

#[derive(Clone, Debug)]
enum Node {
    Wall,
    Open,
}

#[derive(Clone, Debug)]
struct Unit {
    x: usize,
    y: usize,
    hp: u8,
    attack: u8,
    unit_type: UnitType,
}

impl Unit {
    fn rc(x: usize, y: usize, ut: UnitType) -> RefCell<Self> {
        RefCell::new(Self {
            x,
            y,
            unit_type: ut,
            hp: 200,
            attack: 3,
        })
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn enemy_type(&self) -> UnitType {
        match self.unit_type {
            UnitType::Elf => UnitType::Goblin,
            UnitType::Goblin => UnitType::Elf,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum UnitType {
    Elf,
    Goblin,
}

fn main() {
    println!("Hello, world!");
}
