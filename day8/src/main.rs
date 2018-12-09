use std::io::{self, Read};

#[derive(Debug)]
struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<u32>,
}

impl Node {
    fn from_iter<I: Iterator<Item = u32>>(iter: &mut I) -> Self {
        let child_num = iter
            .next()
            .ok_or("Invalid format. Can't read header")
            .unwrap();
        let meta_num = iter
            .next()
            .ok_or("Invalid format. Can't read header")
            .unwrap();
        let children = (0..child_num)
            .map(|_| Box::new(Node::from_iter(iter)))
            .collect();
        let metadata = (0..meta_num)
            .map(|_| {
                iter.next()
                    .ok_or("Invalid format. Can't read meta")
                    .unwrap()
            }).collect();
        return Node { children, metadata };
    }

    fn meta_sum(&self) -> u128 {
        self.metadata.iter().map(|meta| *meta as u128).sum::<u128>()
            + self.children.iter().map(|ch| ch.meta_sum()).sum::<u128>()
    }

    fn node_value(&self) -> u128 {
        if self.children.len() == 0 {
            self.metadata.iter().map(|meta| *meta as u128).sum::<u128>()
        } else {
            self.metadata
                .iter()
                .map(|id| match id {
                    0 => 0,
                    _ => {
                        let i = id - 1;
                        match self.children.get(i as usize) {
                            None => 0,
                            Some(child) => child.node_value(),
                        }
                    }
                }).sum()
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nodes = Node::from_iter(&mut input.split(" ").map(|el| el.parse().unwrap()));
    println!("Metadata sum is {}", nodes.meta_sum());
    println!("Node value is  {}", nodes.node_value());
}
