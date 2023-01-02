fn ascii_to_priority(ascii_code: usize) -> usize {
    match 65 <= ascii_code && ascii_code <= 90 {
        true => ascii_code - 38,
        false => ascii_code - 96
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rucksack = [false; 53];
    let mut priority_sum = 0;    

    for line in input.split("\n") {
        if line.len() == 0 {break}
        let (fst_compartment, snd_compartment) = line.split_at(line.len() / 2);        
        
        for char in fst_compartment.chars() {
            rucksack[ascii_to_priority(char as usize)] = true;
        }
        for char in snd_compartment.chars() {
            let priority = ascii_to_priority(char as usize);
            if rucksack[priority] {
                priority_sum += priority;
                break;
            }
        }
        rucksack = [false;53];
    }
    Some(priority_sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut group_item_types = [0; 53];
    let mut already_changed = [false; 53];
    let mut priority_sum = 0;

    for (i, line) in input.split("\n").enumerate() {
        for char in line.chars() {
            let priority = ascii_to_priority(char as usize);
            
            if !already_changed[priority] {
                group_item_types[priority] += 1;
                already_changed[priority] = true;
            }
            if group_item_types[priority] == 3 {
                priority_sum += priority;
                break;
            }
        }
        already_changed = [false; 53];
        if i % 3 == 2 {group_item_types = [0;53]}
    }
    Some(priority_sum as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
