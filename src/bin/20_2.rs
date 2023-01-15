
#[derive(Debug, Clone)]
enum Tree {
    Node(f64, i32, Box<Tree>, Box<Tree>),
    Nil,
}



impl Tree {
    fn insert(self: &mut Tree, new_key: f64, new_val: i32) {
        let mut key_collision: Option<f64> = None;
        let mut smaller_key: Option<f64> = None;
        let mut curr : &mut Tree = self;
        loop {
            curr = match curr {
                Tree::Node(key, val, ref mut left, ref mut right) => {
                    if new_key < *key {
                        left.as_mut()
                    }
                    else if new_key > *key {
                        smaller_key = Some(*key);
                        right.as_mut()
                    } else {
                        println!("KEY collision {}", val);
                        key_collision = Some(*key);
                        left.as_mut()
                    }
                },
                Tree::Nil => { break; },
            };
        }
        *curr = match key_collision {
            Some(key) => {
                match smaller_key {
                    Some(s_key) => Tree::Node((key + s_key) * 0.5, new_val, Box::new(Tree::Nil), Box::new(Tree::Nil)),
                    None => Tree::Node(key - 1.0, new_val, Box::new(Tree::Nil), Box::new(Tree::Nil))
                }
            },
            None => Tree::Node(new_key, new_val, Box::new(Tree::Nil), Box::new(Tree::Nil))
        }; 
    }

    fn delete(self: &mut Tree, new_key: f64) {
        let mut curr : &mut Tree = self;
        loop {
            curr = match curr {
                Tree::Node(key, _, ref mut left, ref mut right) => {
                    if new_key < *key {
                        left.as_mut()
                    }
                    else if new_key > *key {
                        right.as_mut()
                    } else {
                        left.as_mut()
                    }
                },
                Tree::Nil => { break; },
            };
        }
        *curr = Tree::Nil;
    }

    fn remove(&mut self, value: &f64) {
        let mut current = self;

        while let Some(ref mut node) = current {
            /*
            match node.key.cmp(value) {
                Ordering::Less => current = &mut current.0.as_mut().unwrap().right,
                Ordering::Greater => current = &mut current.0.as_mut().unwrap().left,
                Ordering::Equal => match (node.left.0.as_mut(), node.right.0.as_mut()) {
                    (None, None) => current.0 = None,
                    (Some(_), None) => current.0 = node.left.0.take(),
                    (None, Some(_)) => current.0 = node.right.0.take(),
                    (Some(_), Some(_)) => {
                        current.0.as_mut().unwrap().key = node.right.extract_min().unwrap();
                    }
                }
            }
            */
        }
    }
}



fn modulus(a: i32, b: usize) -> usize {
    (((a % (b as i32)) + (b as i32)) % (b as i32)) as usize
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim();
    let mut encrypted: Vec<i32> = vec![];
    let mut decrypted: Vec<i32> = vec![];
    for line in input.split("\n") {
        let num: i32 = line.parse().unwrap();
        encrypted.push(num);
        decrypted.push(num);
    };
    println!("{:?}", encrypted);

    let mut root = Tree::Node(0.0, 54, Box::new(Tree::Nil), Box::new(Tree::Nil));
    root.insert(0.0, 32);
    root.delete(0.0);

    println!("{:#?}", root);

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
