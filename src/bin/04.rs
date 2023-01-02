use std::cmp::Ordering;



pub fn part_one(input: &str) -> Option<u32> {
    let input = &input[..input.len()-1]; // Get red of trailing newline
    let mut subsets = 0;

    for line in input.split("\n") {
        let ranges = line.split(",").map(|s| s.split("-").collect());
        let ranges: Vec<(u32, u32)> = ranges.map(|vec: Vec<&str>| match vec[..] {
            [low, high] => (low.parse().unwrap(), high.parse().unwrap()),
            _ => (0, 0) // should handle as parse error
        }).collect();
        
        let (left_low, left_high) = ranges[0];
        let (right_low, right_high) = ranges[1];

        let subset_found = match left_low.cmp(&right_low) {
            Ordering::Equal => true,
            Ordering::Less => left_high >= right_high,
            Ordering::Greater => left_high <= right_high
        };
        if subset_found {subsets += 1}
    }
    Some(subsets)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = &input[..input.len()-1]; // Get red of trailing newline
    let mut subsets = 0;

    for line in input.split("\n") {
        let ranges = line.split(",").map(|s| s.split("-").collect());
        let ranges: Vec<(u32, u32)> = ranges.map(|vec: Vec<&str>| match vec[..] {
            [low, high] => (low.parse().unwrap(), high.parse().unwrap()),
            _ => (0, 0) // should handle as parse error
        }).collect();
        
        let (left_low, left_high) = ranges[0];
        let (right_low, right_high) = ranges[1];

        let subset_found = match left_low <= right_high {
            true  => left_high >= right_low,
            false => false
        };
        if subset_found {subsets += 1}
    }
    Some(subsets)

}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
