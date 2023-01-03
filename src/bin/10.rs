pub fn part_one(input: &str) -> Option<i32> {
    let input = &input[..input.len()-1];
    let mut x = 1;
    let mut cycle = 1;
    let mut answer = 0;

    for line in input.split("\n") {
        //println!("{:<9} cycle {}: x={}", line, cycle, x);

        // start of cycle
        let operation = &line[..4];
        match operation {
            "noop" => cycle += 1,
            "addx" => {
                let operand = line[5..].parse::<i32>().ok()?;
                cycle += 1;
                if (cycle - 20) % 40 == 0 {answer += x * cycle;println!("{:<9} cycle {}: x={}", line, cycle, x);}
                cycle += 1;
                x += operand;
            },
            _ => panic!("Unsupported operation")
        }
        if (cycle - 20) % 40 == 0 {answer += x * cycle;println!("{:<9} cycle {}: x={}", line, cycle, x);}
    }
    Some(answer)
}


pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
