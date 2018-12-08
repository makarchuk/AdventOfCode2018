use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
struct Step {
    depends: Vec<Rc<RefCell<Step>>>,
    ready: bool,
}

#[derive(Debug)]
struct Route {
    steps: HashMap<char, Rc<RefCell<Step>>>,
}

impl Route {
    fn next_step(&self) -> Option<char> {
        self.steps
            .iter()
            .filter(|(_, step)| {
                !step.borrow().ready && step.borrow().depends.iter().all(|s| s.borrow().ready)
            }).map(|(name, _)| *name)
            .min_by_key(|name| *name)
    }

    fn compose_route(&mut self) -> String {
        let mut result = String::with_capacity(self.steps.len());
        loop {
            match self.next_step() {
                None => return result,
                Some(ch) => {
                    self.steps.entry(ch).and_modify(|rc| {
                        let mut step = rc.borrow_mut();
                        step.ready = true
                    });
                    result.push(ch)
                }
            }
        }
    }
}

impl FromStr for Route {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut steps = HashMap::new();
        for line in s.split("\n") {
            let mut chars = line.chars();
            let dependency = chars.nth(5).ok_or("Invalid input")?;
            let current = chars.nth(30).ok_or("Invalid input")?;
            let dependency = steps
                .entry(dependency)
                .or_insert(Rc::new(RefCell::new(Step {
                    depends: vec![],
                    ready: false,
                }))).clone();
            let mut current = steps
                .entry(current)
                .or_insert(Rc::new(RefCell::new(Step {
                    depends: vec![],
                    ready: false,
                }))).borrow_mut();
            current.depends.push(dependency);
        }
        Ok(Self { steps })
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut route = input.parse::<Route>().unwrap();
    println!("Number of Nodes is {}", route.steps.len());
    println!("Route is {}", route.compose_route());
}
