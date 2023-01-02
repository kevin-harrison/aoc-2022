pub fn part_one(input: &str) -> Option<u32> {
    let mut calories: u32 = 0;
    let mut elf_calories: Vec<u32> = Vec::new();

    for line in input.split("\n") {
        if line == "" {
            elf_calories.push(calories);
            calories = 0;
        }
        else {
            calories += line.parse::<u32>().unwrap();
        }
    }
    elf_calories.push(calories);
    elf_calories.sort_by(|a, b| b.cmp(a));
    return Some(elf_calories[0]);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories: u32 = 0;
    let mut elf_calories: Vec<u32> = Vec::new();

    for line in input.split("\n") {
        if line == "" {
            elf_calories.push(calories);
            calories = 0;
        }
        else {
            calories += line.parse::<u32>().unwrap();
        }
    }
    elf_calories.push(calories);
    elf_calories.sort_by(|a, b| b.cmp(a));
    return Some(elf_calories[..3].iter().sum::<u32>());
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
