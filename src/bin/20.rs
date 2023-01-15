use std::{cmp::Ordering, f64};

#[derive(Debug)]
struct Node {
    key: f64,
    value: (usize, i32),
    left: Tree,
    right: Tree,
}

#[derive(Debug)]
struct Tree(Option<Box<Node>>);

#[derive(Debug)]
pub struct BinarySearchTree {
    root: Tree,
}

impl Node {
    fn new(key: f64, value: (usize, i32)) -> Self {
        Node {
            key,
            value,
            left: Tree(None),
            right: Tree(None),
        }
    }
}

impl Tree {
    fn add(&mut self, key: f64, value: (usize, i32)) {        
        let mut key_collision: Option<f64> = None;
        let mut larger_key: Option<f64> = None;
        let mut current = self;

        while let Some(ref mut node) = current.0 {
            match node.key.partial_cmp(&key) {
                Some(Ordering::Greater) => {
                    larger_key = Some(node.key);
                    current = &mut node.left},
                Some(Ordering::Less) =>
                    current = &mut node.right,
                Some(Ordering::Equal) => {
                    key_collision = Some(node.key);
                    current = &mut node.right
                },
                None => panic!("uhoh")
            }
        }

        match key_collision {
            Some(collision_key) => {
                match larger_key {
                    Some(large_key) => current.0 = Some(Box::new(Node::new((collision_key + large_key) * 0.5, value))),
                    None => current.0 = Some(Box::new(Node::new(key + 1.0, value)))
                }
            },
            None => current.0 = Some(Box::new(Node::new(key, value)))
        }
    }

    fn successor(&self, key: &f64) -> Option<&f64> {
        let mut current = self.0.as_ref();
        let mut successor = None;
        while current.is_some() {
            let node = current.unwrap();
            if *key < node.key {
                successor = current;
                current = node.left.0.as_ref();
            } else {
                current = node.right.0.as_ref();
            }
        }

        successor.map(|node| &node.key)
    }

    fn extract_min(&mut self) -> Option<f64> {
        let mut node = None;

        if self.0.is_some() {
            let mut current = self;

            while current.0.as_ref().unwrap().left.0.is_some() {
                current = &mut current.0.as_mut().unwrap().left;
            }

            let temp = current.0.take().unwrap();
            node = Some(temp.key);
            current.0 = temp.right.0;
        }

        node
    }

    fn extract_max(&mut self) -> Option<f64> {
        let mut node = None;

        if self.0.is_some() {
            let mut current = self;

            while current.0.as_ref().unwrap().right.0.is_some() {
                current = &mut current.0.as_mut().unwrap().right;
            }

            let temp = current.0.take().unwrap();
            node = Some(temp.key);
            current.0 = temp.left.0;
        }

        node
    }

    fn remove(&mut self, key: &f64) {
        let mut current = self;

        while let Some(ref mut node) = current.0 {
            match node.key.partial_cmp(key) {
                Some(Ordering::Less) => current = &mut current.0.as_mut().unwrap().right,
                Some(Ordering::Greater) => current = &mut current.0.as_mut().unwrap().left,
                Some(Ordering::Equal) => match (node.left.0.as_mut(), node.right.0.as_mut()) {
                    (None, None) => current.0 = None,
                    (Some(_), None) => current.0 = node.left.0.take(),
                    (None, Some(_)) => current.0 = node.right.0.take(),
                    (Some(_), Some(_)) => {
                        current.0.as_mut().unwrap().key = node.right.extract_min().unwrap();
                    }
                },
                None => panic!("uhoh")
            }
        }
    }

}

fn print(tree: &Tree) {
    if let Some(ref node) = tree.0 {
        print(&node.left);
        print!("{:?} ", (node.key, node.value.1));
        print(&node.right);
    }
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: Tree(None) }
    }
    pub fn from_ordered(list : &Vec<(usize, i32)>) -> Self {
        let root: Option<Box<Node>> = BinarySearchTree::build(list, 0, list.len() - 1);
        BinarySearchTree { root: Tree(root) }
    }
    fn build(list: &Vec<(usize, i32)>, low: usize, high: usize) -> Option<Box<Node>> {
        if low > high {
            return None;
        }
        if low == high {
            return  Some(Box::new(Node {
                key: list[low].0 as f64, 
                value: list[low],
                left: Tree(None),
                right:Tree(None)
            }));
        }
        let mid = (low + high) / 2;
        let left = Tree(BinarySearchTree::build(list, low, mid-1));
        let right = Tree(BinarySearchTree::build(list, mid+1, high));
        Some(Box::new(Node {
            key: list[mid].0 as f64, 
            value: list[mid],
            left,
            right
        }))
    }

    pub fn add(&mut self, key: f64, value: (usize, i32)) {
        self.root.add(key, value);
    }
    pub fn remove(&mut self, key: &f64) {
        self.root.remove(key);
    }
    pub fn successor(&self, key: &f64) -> Option<&f64> {
        self.root.successor(key)
    }
}



fn modulus(a: i32, b: usize) -> usize {
    (((a % (b as i32)) + (b as i32)) % (b as i32)) as usize
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim();
    let encrypted: Vec<(usize, i32)>  = input
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .enumerate()
        .collect();
    println!("{:?}", encrypted);

    let mut tree = BinarySearchTree::from_ordered(&encrypted);

    println!("{:#?}", tree);
    let (index, val) = (0.0, 1);
    print(&tree.root);
    println!();
    tree.remove(&index);
    print(&tree.root);
    println!();
    tree.add(index + val as f64, (0, val));
    print(&tree.root);
    println!();
    println!();

    let (index, val) = (1.0, 2);
     print(&tree.root);
    println!();
    tree.remove(&index);
    print(&tree.root);
    println!();
    tree.add(index + val as f64, (0, val));
    print(&tree.root);
    println!();



    Some(1)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
