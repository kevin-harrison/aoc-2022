use std::{collections::HashSet, iter::TakeWhile};

pub fn part_one(input: &str) -> Option<u32> {
    let input = &input[..input.len()-1];
    let mut tail_positions = HashSet::<(i32,i32)>::new();
    let mut head: (i32,i32) = (0,0);
    let mut tail: (i32,i32) = (0,0);
    tail_positions.insert(tail);


    for line in input.split("\n") {
        let steps = line[2..].parse::<u32>().ok()?;
        
        for _ in 0..steps {
            match line.chars().nth(0)? {
                'R' => head.0 += 1,
                'L' => head.0 -= 1,
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                _ => panic!("Invalid direction")
            }
            let diff_x = head.0 - tail.0;
            let diff_y = head.1 - tail.1;
            if diff_x.abs() > 1 || diff_y.abs() > 1{
                tail.0 += diff_x.clamp(-1, 1);
                tail.1 += diff_y.clamp(-1, 1);
                tail_positions.insert(tail);
            }
        }
    }
    Some(tail_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = &input[..input.len()-1];
    let mut tail_positions = HashSet::<(i32,i32)>::new();
    let mut rope: [(i32,i32); 10] = [(0,0); 10];
    tail_positions.insert((0,0));


    for line in input.split("\n") {
        let steps = line[2..].parse::<u32>().ok()?;
        
        for _ in 0..steps {
            match line.chars().nth(0)? {
                'R' => rope[0].0 += 1,
                'L' => rope[0].0 -= 1,
                'U' => rope[0].1 += 1,
                'D' => rope[0].1 -= 1,
                _ => panic!("Invalid direction")
            }

            
            for i in 1..rope.len() {
                let diff_x = rope[i-1].0 - rope[i].0;
                let diff_y = rope[i-1].1 - rope[i].1;
                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    rope[i].0 += diff_x.clamp(-1, 1);
                    rope[i].1 += diff_y.clamp(-1, 1);
                }
            }
            tail_positions.insert(rope[9]);
        }
    }
    Some(tail_positions.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(0));
    }
}
