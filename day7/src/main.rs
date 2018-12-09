use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug};
use std::io::{self, Read};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
struct Step {
    depends: Vec<Rc<RefCell<Step>>>,
    ready: bool,
    price: u32,
}

#[derive(Debug)]
struct Route {
    steps: HashMap<char, Rc<RefCell<Step>>>,
}

impl Route {
    fn next_step_with_lock(&self, locked: Vec<char>) -> Option<char> {
        self.steps
            .iter()
            .filter(|(name, step)| {
                !locked.contains(name)
                    && !step.borrow().ready
                    && step.borrow().depends.iter().all(|s| s.borrow().ready)
            }).map(|(name, _)| *name)
            .min_by_key(|name| *name)
    }

    fn next_step(&self) -> Option<char> {
        self.next_step_with_lock(vec![])
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

    fn price_from_char(c: char) -> u32 {
        let mut buffer = [0; 1];
        c.encode_utf8(&mut buffer);
        //65 is "A" in ascii
        buffer[0] as u32 - 64 + 60
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
                    price: Route::price_from_char(dependency),
                }))).clone();
            let mut current = steps
                .entry(current)
                .or_insert(Rc::new(RefCell::new(Step {
                    depends: vec![],
                    price: Route::price_from_char(current),
                    ready: false,
                }))).borrow_mut();
            current.depends.push(dependency);
        }
        Ok(Self { steps })
    }
}

struct Worker {
    name: char,
    progress: u32,
    step: Rc<RefCell<Step>>,
}

impl Worker {
    fn new(step: Rc<RefCell<Step>>, ch: char) -> Self {
        Self {
            progress: 0,
            name: ch,
            step,
        }
    }

    fn run(&mut self) {
        self.progress += 1;
    }

    fn ready(&self) -> bool {
        self.progress == self.step.borrow().price
    }
}

impl Debug for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Worker ({}/{} for {})",
            self.progress,
            self.step.borrow().price,
            self.name
        )
    }
}

#[derive(Debug)]
struct WorkerPool {
    max_workers: usize,
    workers: Vec<Worker>,
}

impl WorkerPool {
    fn run(&mut self) {
        self.workers.iter_mut().for_each(|w| w.run());
    }

    fn check(&mut self) -> usize {
        self.run();
        let mut result = 0;
        self.workers.iter().filter(|w| w.ready()).for_each(|w| {
            let mut step = w.step.borrow_mut();
            step.ready = true;
            result += 1
        });
        self.workers.retain(|worker| !worker.ready());
        result
    }

    fn full(&self) -> bool {
        self.workers.len() == self.max_workers
    }

    fn empty(&self) -> bool {
        self.workers.len() == 0
    }

    fn start_task(&mut self, step: Rc<RefCell<Step>>, ch: char) {
        self.workers.push(Worker::new(step, ch));
    }

    fn busy(&self) -> Vec<char> {
        self.workers.iter().map(|w| w.name).collect()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut route = input.parse::<Route>().unwrap();
    println!("Number of Nodes is {}", route.steps.len());
    println!("Route is {}", route.compose_route());

    let mut part2_res = 0;
    let mut completed_steps = 0;
    let route = input.parse::<Route>().unwrap();
    let mut worker_pool = WorkerPool {
        max_workers: 5,
        workers: vec![],
    };
    loop {
        while !worker_pool.full() {
            match route.next_step_with_lock(worker_pool.busy()) {
                None => break,
                Some(step) => worker_pool.start_task(route.steps[&step].clone(), step),
            }
        }
        part2_res += 1;
        println!("[{}] {:?}", part2_res, worker_pool);
        completed_steps += worker_pool.check();
        if completed_steps == route.steps.len() {
            println!("The whole root will take {}", part2_res);
            break;
        }
    }
}
