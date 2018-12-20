use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Game {
    units: Vec<Unit>,
    map: Vec<Vec<Node>>,
    dimensions: (usize, usize),
}

impl Game {
    fn get(&self, x: usize, y: usize) -> &Node {
        &self.map[y][x]
    }

    fn can_visit(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            Node::Wall => false,
            Node::Open => self.units.iter().any(|u| u.x == x && u.y == y),
        }
    }

    fn closest_enemy(&self, x: usize, y: usize, ut: UnitType) -> Option<(usize, usize)> {
        let mut checked_before = HashSet::new();
        checked_before.insert((x, y));
        let mut border = vec![(x, y)];
        loop {
            let new_border = border.iter().fold(vec![], |mut new_locs, loc| {
                if !checked_before.contains(loc) && self.can_visit(loc.0, loc.1) {
                    new_locs.push(*loc)
                }
                new_locs
            });
            if new_border.len() == 0 {
                return None;
            }
            let mut found_enemies = new_border
                .iter()
                .filter(|(cur_x, cur_y)| self.check_enemy(*cur_x, *cur_y, &ut))
                .collect::<Vec<_>>();
            found_enemies.sort_by(|(x, y), (other_x, other_y)| match y.cmp(other_y) {
                Ordering::Equal => x.cmp(other_x),
                more_or_less => more_or_less,
            });
            match found_enemies.iter().next() {
                None => (),
                Some(loc) => return Some(**loc),
            }
            checked_before.extend(new_border.iter());
            border = new_border
        }
    }

    fn check_enemy(&self, x: usize, y: usize, ut: &UnitType) -> bool {
        self.units
            .iter()
            .any(|u| u.x == x && u.y == y && u.unit_type == *ut)
    }

    fn around(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = vec![];
        if x != 0 {
            result.push((x - 1, y))
        }
        if y != 0 {
            result.push((x, y - 1))
        }
        if x != self.dimensions.0 {
            result.push((x + 1, y))
        }
        if y != self.dimensions.1 {
            result.push((x, y + 1))
        }
        result
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

#[derive(Clone, Debug, PartialEq)]
enum UnitType {
    Elf(),
    Goblin(),
}

fn main() {
    println!("Hello, world!");
}
