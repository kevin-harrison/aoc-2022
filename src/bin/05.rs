use regex::Regex;

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input = &input[..input.len()-1]; // remove trailing newline
    let mut input_split = input.split("\n\n");
    let crates_input: Vec<&str> = input_split
        .next()
        .expect("No crates config")
        .split("\n")
        .collect();
    let movement_input: Vec<&str> = input_split
        .next()
        .expect("No movements found")
        .split("\n")
        .collect();
    (parse_crates(crates_input), parse_movement(movement_input))
}

fn parse_crates(lines: Vec<&str>) -> Vec<Vec<char>> {
    let num_stacks: usize = lines.last().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let mut crates: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    for line in lines.iter().rev() {
        for (i, char) in line.chars().enumerate() {
            if char.is_alphabetic() {
               crates[(i - 1) / 4].push(char); 
            }
        }    
    }
    crates
}

fn parse_movement(lines: Vec<&str>) -> Vec<(usize, usize, usize)> {
    let mut movements = Vec::<(usize, usize, usize)>::new();
    let re = Regex::new(r"\d+").unwrap();

    for line in lines {
        let movement: Vec<usize> = re.find_iter(line)
                                   .map(|n| n.as_str()
                                             .parse::<usize>()
                                             .expect("Should be a number"))
                                   .collect();
        movements.push((movement[0], movement[1]-1, movement[2]-1));
    }
    movements
}


pub fn part_one(input: &str) -> Option<String> {
    // Parse input
    let (mut crates, movements) = parse_input(input);    
    
    // Execute crate movements
    for movement in movements {
        for _ in 0..movement.0 {
            let cargo = crates[movement.1].pop().unwrap();
            crates[movement.2].push(cargo);
        }
    }
    
    // Collect crates at top of stacks into string
    Some(crates.iter()
               .map(|stack: &Vec<char>| stack.last()    
                                             .expect("There was an empty stack!"))
               .collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    // Parse input
    let (mut crates, movements) = parse_input(input);    
    
    // Execute crate movements
    for movement in movements {
        let mut temp = Vec::<char>::new();
        for _ in 0..movement.0 {
            let cargo = crates[movement.1].pop().unwrap();
            temp.push(cargo);
        }
        for _ in 0..movement.0 {
            let cargo = temp.pop().unwrap();
            crates[movement.2].push(cargo);
        }
    }
    // Collect crates at top of stacks into string
    Some(crates.iter()
               .map(|stack: &Vec<char>| stack.last()    
                                             .expect("There was an empty stack!"))
               .collect::<String>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
