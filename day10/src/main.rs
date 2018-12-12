use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read};

struct Star {
    x: i128,
    y: i128,
    vx: i128,
    vy: i128,
}

impl Star {
    fn parse(s: &str) -> Self {
        //Transform input format to x|y|vx|vy
        let s = s
            .to_owned()
            .replace("position=", "")
            .replace(" velocity=", "|")
            .replace("<", "")
            .replace(">", "")
            .replace(", ", "|")
            .replace(" ", "");
        let chunks = s.split("|").collect::<Vec<_>>();
        Self {
            x: chunks[0].parse().unwrap(),
            y: chunks[1].parse().unwrap(),
            vx: chunks[2].parse().unwrap(),
            vy: chunks[3].parse().unwrap(),
        }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

struct Sky {
    stars: Vec<Star>,
    time: u32,
}

impl Sky {
    fn step(&mut self) {
        self.time += 1;
        self.stars.iter_mut().for_each(|s| s.step());
    }

    //Detect whether or not current state presents a meaningful writing
    //to do so we will try and find vertical line at least 8 stars long
    //(which is lazy, but I assume will work)
    fn meaningful(&self) -> bool {
        let grouped_coordinates = self.stars.iter().fold::<HashMap<i128, Vec<i128>>, _>(
            HashMap::new(),
            |mut map, star| {
                {
                    let entry = map.entry(star.x).or_insert(vec![]);
                    entry.push(star.y);
                    entry.sort_unstable();
                }
                map
            },
        );
        let consecutives = grouped_coordinates
            .values()
            .map(|ys| {
                ys.iter().fold(
                    (None, 1, 0),
                    |(previous, current_max, consecutive), yval| match previous {
                        None => (Some(yval), current_max, 1),
                        Some(previous_y) => {
                            if *previous_y == *yval {
                                (Some(yval), current_max, consecutive)
                            } else if *previous_y == yval - 1 {
                                if consecutive + 1 >= current_max {
                                    (Some(yval), consecutive + 1, consecutive + 1)
                                } else {
                                    (Some(yval), current_max, consecutive + 1)
                                }
                            } else {
                                (Some(yval), current_max, 1)
                            }
                        }
                    },
                )
            }).map(|(_, max, _)| max)
            .max()
            .ok_or("This collection should not be empty!")
            .unwrap();
        consecutives >= 8
    }

    //Tuple of ((minx, maxx, miny, maxy))
    fn range(&self) -> (i128, i128, i128, i128) {
        let mut iter = self.stars.iter();
        let head = iter.next().ok_or("Empty sky?").unwrap();
        let init = (head.x, head.x, head.y, head.y);
        iter.fold(init, |(minx, maxx, miny, maxy), star| {
            (
                cmp::min(minx, star.x),
                cmp::max(maxx, star.x),
                cmp::min(miny, star.y),
                cmp::max(maxy, star.y),
            )
        })
    }
}

impl fmt::Display for Sky {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let range = self.range();
        let mut result =
            String::with_capacity(((range.3 - range.2 + 1) * (range.1 - range.0 + 1)) as usize);
        for y in range.2..=range.3 {
            for x in range.0..=range.1 {
                if self.stars.iter().any(|s| s.x == x && s.y == y) {
                    result.push('#')
                } else {
                    result.push('.')
                }
            }
            result.push('\n')
        }
        return f.write_str(&result);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut sky = Sky {
        time: 0,
        stars: input.split("\n").map(|s| Star::parse(s)).collect(),
    };
    for i in 0..50000 {
        // for i in 0..=10 {
        if sky.meaningful() {
            println!("Meaningful sky at step #{}!", i);
            println!("{}", sky);
            println!("================================================");
        }
        sky.step();
    }
}
