use std::{cmp::Ordering, f64};

#[derive(Debug)]
struct Node {
    key: f64,
    value: (usize, i32),
    left_size: usize,
    right_size: usize,
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
            left_size: 0,
            right_size: 0,
        }
    }
}

impl Tree {
    fn add(&mut self, key: f64, value: (usize, i32)) -> f64 {        
        let mut key_collision: Option<f64> = None;
        let mut larger_key: Option<f64> = None;
        let mut current = self;

        while let Some(ref mut node) = current.0 {
            match node.key.partial_cmp(&key) {
                Some(Ordering::Greater) => {
                    larger_key = Some(node.key);
                    node.left_size += 1;
                    current = &mut node.left},
                Some(Ordering::Less) => {
                    node.right_size += 1;
                    current = &mut node.right
                },
                Some(Ordering::Equal) => {
                    key_collision = Some(node.key);
                    node.right_size += 1;
                    current = &mut node.right
                },
                None => panic!("uhoh")
            }
        }

        match key_collision {
            Some(collision_key) => {
                match larger_key {
                    Some(large_key) => {
                        let new_key = (collision_key + large_key) * 0.5;
                        current.0 = Some(Box::new(Node::new(new_key, value)));
                        return new_key;
                    },
                    None => {
                        current.0 = Some(Box::new(Node::new(key + 1.0, value)));
                        return key + 1.0;
                    }
                }
            },
            None => {
                current.0 = Some(Box::new(Node::new(key, value)));
                return key;
            }
        }
    }

    fn add_rank(&mut self, rank: usize, value: (usize, i32)) -> f64 {
        let mut key_collision: Option<f64> = None;
        let mut larger_key: Option<f64> = None;        
        let mut curr_rank = self.0.as_ref().unwrap().left_size;
        let mut current = self;

        // If rank = 0 add as first node
        if rank == 0 {
            let mut smallest_key = 0.0;
            while let Some(ref mut node) = current.0 {
                smallest_key = node.key;
                current = &mut node.left;
            }
            current.0 = Some(Box::new(Node::new(smallest_key - 1.0, value)));
            return smallest_key - 1.0;
        }

        // Otherwise add directly after node with rank-1
        let rank = rank - 1;
        while let Some(ref mut node) = current.0 {
            println!("current rank:{} Destination:{}", curr_rank, rank);
            match curr_rank.cmp(&rank) {
                Ordering::Greater => {
                    larger_key = Some(node.key);
                    node.left_size += 1;
                    current = &mut node.left;
                    match current.0.as_ref() {
                        Some(n) => curr_rank -= n.right_size + 1,
                        None => ()
                    }
                    println!("left {}", curr_rank);
                },
                Ordering::Less => {
                    node.right_size += 1;
                    current = &mut node.right;
                    curr_rank += current.0.as_ref().unwrap().left_size + 1;
                    println!("right {}", curr_rank);
                },
                Ordering::Equal => {
                    key_collision = Some(node.key);
                    node.right_size += 1;
                    current = &mut node.right;
                    match current.0.as_ref() {
                        Some(n) => curr_rank += n.left_size + 1,
                        None => ()
                    }
                    println!("Foundit");
                    println!("right {}", curr_rank);
                }
            }
        } 
        
        match key_collision {
            Some(collision_key) => {
                match larger_key {
                    Some(large_key) => {
                        let new_key = (collision_key + large_key) * 0.5;
                        current.0 = Some(Box::new(Node::new(new_key, value)));
                        return new_key;
                    },
                    None => {
                        current.0 = Some(Box::new(Node::new(collision_key + 1.0, value)));
                        return collision_key + 1.0;
                    }
                }
            },
            None => {
                panic!("Element with rank {} doesn't exist to add node after!", rank);
            }
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

    fn extract_min(&mut self) -> Option<(f64, (usize, i32))> {
        let mut node = None;

        if self.0.is_some() {
            let mut current = self;

            while current.0.as_ref().unwrap().left.0.is_some() {
                current.0.as_mut().unwrap().left_size -= 1;
                current = &mut current.0.as_mut().unwrap().left;
            }

            let temp = current.0.take().unwrap();
            node = Some((temp.key, temp.value));
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

    fn remove(&mut self, key: &f64) -> usize {
        let mut current = self;
        let mut nth: usize = 0;

        while let Some(ref mut node) = current.0 {
            match node.key.partial_cmp(key) {
                Some(Ordering::Less) => {
                    nth += node.left_size + 1;
                    node.right_size -= 1;
                    current = &mut current.0.as_mut().unwrap().right;
                },
                Some(Ordering::Greater) => {
                    node.left_size -= 1;
                    current = &mut current.0.as_mut().unwrap().left
                },
                Some(Ordering::Equal) => {
                    nth += node.left_size;
                    match (node.left.0.as_mut(), node.right.0.as_mut()) {
                        (None, None) => current.0 = None,
                        (Some(_), None) => {
                            node.left_size -= 1;
                            current.0 = node.left.0.take();
                        },
                        (None, Some(_)) => {
                            node.right_size -= 1;
                            current.0 = node.right.0.take();
                        },
                        (Some(_), Some(_)) => {
                            node.right_size -= 1;
                            let replacement_node = node.right.extract_min().unwrap();
                            current.0.as_mut().unwrap().key = replacement_node.0;
                            current.0.as_mut().unwrap().value = replacement_node.1;

                        }
                    };
                    //break;
                    return nth;
                },
                None => panic!("uhoh")
            }
        }
        panic!("Tried to remove node with key {} which doesn't exist", key);
    }

}

fn print(tree: &Tree) {
    if let Some(ref node) = tree.0 {
        print(&node.left);
        print!("{:?} ", node.value.1);
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
                right:Tree(None),
                left_size: 0,
                right_size: 0,
            }));
        }
        let mid = (low + high) / 2;
        let left = Tree(BinarySearchTree::build(list, low, mid-1));
        let right = Tree(BinarySearchTree::build(list, mid+1, high));
        Some(Box::new(Node {
            key: list[mid].0 as f64, 
            value: list[mid],
            left,
            right,
            left_size: mid - low,
            right_size: high - mid,
        }))
    }

    pub fn add(&mut self, key: f64, value: (usize, i32)) -> f64 {
        self.root.add(key, value)
    }   
    pub fn add_rank(&mut self, rank: usize, value: (usize, i32)) -> f64 {
        self.root.add_rank(rank, value)
    }
    pub fn remove(&mut self, key: &f64) -> usize {
        self.root.remove(key)
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

    let mut keys: Vec<f64> = encrypted.iter().map(|(index ,_)| *index as f64).collect();
    let mut tree = BinarySearchTree::from_ordered(&encrypted);

    //println!("Added new index {}", tree.add(1.0, (55,55)));
    println!("{:#?}", tree);

    //println!("Removed the {}-th element", tree.remove(&5.0));
    //println!("{:#?}", tree);

    
    for &(original_index, val) in encrypted.iter() {
        //if original_index == 4 {break}
        let shift_by = modulus(val, encrypted.len());
        if shift_by == 0 {continue;}

        let key = keys[original_index];
        let rank = tree.remove(&key);
        //println!("{:#?}", tree);
        let new_rank = modulus(rank as i32 + shift_by as i32, encrypted.len());
        println!("{:?}: {} -> {}", (original_index, val), rank, new_rank);
        if rank == new_rank {   
            panic!("Shouldn't be shfiting by 0");
        }
        let new_key = tree.add_rank(new_rank, (original_index, val));
        keys[original_index] = new_key;
        print(&tree.root);
        println!();
        //println!("{:#?}", tree);
        println!();
        println!();
        println!();
    }
    

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
