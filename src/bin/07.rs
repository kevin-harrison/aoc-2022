pub fn part_one(input: &str) -> Option<u32> {
    let input = &input[..input.len()-1]; // Remove trailing newline
    let mut folder_size_sum = 0;
    let mut folder_stack = Vec::<u32>::new();
    
    for line in input.split("\n") {
        if line.starts_with("dir ") {continue}
        else if line.starts_with("$ ls") {continue}
        
        // Go up a folder == done calculating its size
        else if line.starts_with("$ cd ..") {
            let folder_size = folder_stack.pop().unwrap();
            let file_idx = folder_stack.len()-1;
            folder_stack[file_idx] += folder_size; 
            if folder_size <= 100000 {folder_size_sum += folder_size;}
        }
        // Go down a folder == new folder on stack
        else if line.starts_with("$ cd") {folder_stack.push(0)}
        // Add file size to top of stack
        else {
            let file_size: u32 = line.split(" ").next().unwrap().parse().unwrap();
            let file_idx = folder_stack.len()-1;
            folder_stack[file_idx] += file_size;
        }
    }
    
    // Processes final folder chain
    for i in (0..folder_stack.len()).rev() {
        let folder_size = folder_stack.pop().unwrap();
        if folder_size <= 100000 {folder_size_sum += folder_size;}
        if i != 0 {folder_stack[i-1] += folder_size}
    }
    Some(folder_size_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = &input[..input.len()-1]; // Remove trailing newline
    let mut folder_stack = Vec::<u32>::new();
    let mut smallest_deleteable_file = u32::MAX;
    
    for line in input.split("\n") {
        if line.starts_with("dir ") {continue}
        else if line.starts_with("$ ls") {continue}
        
        // Go up a folder == done calculating its size
        else if line.starts_with("$ cd ..") {
            let folder_size = folder_stack.pop().unwrap();
            let file_idx = folder_stack.len()-1;
            folder_stack[file_idx] += folder_size; 
            if folder_size >= 8381165 && folder_size < smallest_deleteable_file {smallest_deleteable_file = folder_size;}
        }
        // Go down a folder == new folder on stack
        else if line.starts_with("$ cd") {folder_stack.push(0)}
        // Add file size to top of stack
        else {
            let file_size: u32 = line.split(" ").next().unwrap().parse().unwrap();
            let file_idx = folder_stack.len()-1;
            folder_stack[file_idx] += file_size;
        }
    }

    // Processes final folder chain
    for i in (0..folder_stack.len()).rev() {
        let folder_size = folder_stack.pop().unwrap();
        if folder_size >= 8381165 && folder_size < smallest_deleteable_file {smallest_deleteable_file = folder_size;}
        if i != 0 {folder_stack[i-1] += folder_size}
    }

    Some(smallest_deleteable_file)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
