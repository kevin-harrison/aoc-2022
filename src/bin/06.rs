pub fn part_one(input: &str) -> Option<usize> {
    const PACKET_MARKER_LEN: usize = 4;
    let mut occurances = [0; 256];
    let mut unique_symbols = 0;
    let input: Vec<char> = input.chars().collect();

    for (i, char) in input.iter().enumerate() {
        let char_idx = *char as usize;
        
        // Remove occurance
        if i >= PACKET_MARKER_LEN {
            occurances[input[i-PACKET_MARKER_LEN] as usize] -= 1;
            if occurances[input[i-PACKET_MARKER_LEN] as usize] == 0 {unique_symbols -= 1}
        }

        // Add occurance
        if occurances[char_idx] == 0 {unique_symbols += 1}          
        occurances[char_idx] += 1;

        // Check for packet
        if unique_symbols == PACKET_MARKER_LEN {
            return Some(i+1); 
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    const PACKET_MARKER_LEN: usize = 14;
    let mut occurances = [0; 256];
    let mut unique_symbols = 0;
    let input: Vec<char> = input.chars().collect();

    for (i, char) in input.iter().enumerate() {
        let char_idx = *char as usize;
        
        // Remove occurance
        if i >= PACKET_MARKER_LEN {
            occurances[input[i-PACKET_MARKER_LEN] as usize] -= 1;
            if occurances[input[i-PACKET_MARKER_LEN] as usize] == 0 {unique_symbols -= 1}
        }

        // Add occurance
        if occurances[char_idx] == 0 {unique_symbols += 1}          
        occurances[char_idx] += 1;

        // Check for packet
        if unique_symbols == PACKET_MARKER_LEN {
            return Some(i+1); 
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
