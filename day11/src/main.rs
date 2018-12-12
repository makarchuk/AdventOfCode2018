use std::cmp;
use std::io::{self, Read};

struct Grid {
    cells: Vec<Vec<Option<i128>>>,
    serial: i128,
}

impl Grid {
    fn new(serial: i128, size: usize) -> Self {
        Self {
            cells: vec![vec![None; size]; size],
            serial,
        }
    }

    fn score(&mut self, x: usize, y: usize) -> i128 {
        if let Some(v) = self.cells[x][y] {
            return v;
        }
        let rack_id = x as i128 + 10;
        let power_level = rack_id * y as i128;
        let power_level = power_level + self.serial;
        let power_level = power_level * rack_id;
        let power_level = (power_level / 100) % 10;
        let power_level = power_level - 5;
        self.cells[x][y] = Some(power_level);
        power_level
    }

    fn square_score(&mut self, minx: usize, miny: usize, size: usize) -> i128 {
        // println!("Square: {},{}", minx, miny);
        let mut total = 0;
        for x in minx..minx + size {
            for y in miny..miny + size {
                // println!("{},{}->{}", x, y, self.score(x, y));
                total += self.score(x, y)
            }
        }
        total
    }

    fn squares(&self, min_size: usize) -> Squares {
        Squares::new(
            0,
            self.cells.len() - min_size,
            0,
            self.cells.len() - min_size,
        )
    }
}

struct Squares {
    minx: usize,
    maxx: usize,
    miny: usize,
    maxy: usize,
    x: usize,
    y: Option<usize>,
}

impl Iterator for Squares {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.y {
            Some(y) => {
                if y < self.maxy {
                    self.y = Some(y + 1)
                } else {
                    if self.x < self.maxx {
                        self.x += 1;
                        self.y = Some(self.miny)
                    } else {
                        return None;
                    }
                }
                Some((self.x, self.y.expect("There really should be an y here")))
            }
            None => {
                self.y = Some(self.miny);
                Some((self.minx, self.miny))
            }
        }
    }
}

impl Squares {
    fn new(minx: usize, maxx: usize, miny: usize, maxy: usize) -> Self {
        Self {
            minx,
            maxx,
            miny,
            maxy,
            x: minx,
            y: None,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let serial: i128 = input.parse().unwrap();
    let mut grid = Grid::new(serial, 300);
    let (x, y, score) = grid
        .squares(3)
        .map(|(x, y)| (x, y, grid.square_score(x, y, 3)))
        .max_by_key(|(_x, _y, score)| *score)
        .ok_or("Can't find max")
        .unwrap();
    println!(
        "Corner of the best 3x3 square is {},{}. Score={}",
        x, y, score
    );
    let (x, y, size, score) = grid
        .squares(1)
        .map(|(x, y)| {
            if x % 10 == 0 && y % 100 == 0 {
                println!("{};{}", x, y);
            }
            let xleft = grid.cells.len() - x;
            let yleft = grid.cells.len() - y;
            let (size, score) = (0..cmp::min(xleft, yleft))
                .map(|size| (size, grid.square_score(x, y, size)))
                .max_by_key(|(_, score)| *score)
                .ok_or("Unable to find optimal size")
                .unwrap();
            (x, y, size, score)
        })
        .max_by_key(|(_x, _y, _size, score)| *score)
        .ok_or("Can't find max")
        .unwrap();
    println!("The best square is {},{},{}. Score={}", x, y, size, score);
}
