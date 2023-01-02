use std::result;

fn scan_treeline(trees: Vec<&[u8]>, result: &[&[u8]]) {
}


pub fn part_one(input: &str) -> Option<u32> {
    let input = &input[..input.len()-1]; // Remove trailing newline
    let trees: Vec<&[u8]> = input.split("\n")
                                 .map(|line| line.as_bytes())
                                 .collect();
    let len = trees.len(); // grid is square
    const SIZE: usize = 99;
    let mut treeline_vertical = [0; SIZE];
    let mut treeline_horizontal = [0; SIZE];
    let mut result = [[0; SIZE]; SIZE];

    // top-left pass
    for i in 0..len {
        for j in 0..len {
            if trees[i][j] > treeline_vertical[j] {
                treeline_vertical[j] = trees[i][j];
                result[i][j] += 1;
            }
            if trees[i][j] > treeline_horizontal[i] {
               treeline_horizontal[i] = trees[i][j];
               result[i][j] += 1;
            }
        }
    }

    // bottom-right pass
    treeline_vertical = [0; SIZE];
    treeline_horizontal = [0; SIZE];

    for i in (0..len).rev() {
        for j in (0..len).rev() {
            if trees[i][j] > treeline_vertical[j] {
                treeline_vertical[j] = trees[i][j];
                result[i][j] += 1;
            }
            if trees[i][j] > treeline_horizontal[i] {
               treeline_horizontal[i] = trees[i][j];
               result[i][j] += 1;
            }
        }
    }


    let answer: u32 = result.iter()
                       .map(|line| line.iter()
                                       .filter(|&&x| x > 0)
                                       .count()
                                       as u32)
                       .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
