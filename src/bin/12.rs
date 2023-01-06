use std::collections::{VecDeque, HashMap};

fn get_neighbors(heightmap: &Vec<&[u8]>, row: usize, col: usize) -> Vec<(usize,usize)> {
    let mut neighbors = Vec::<(usize,usize)>::new(); 
    for (x, y) in [(0,-1), (1,0), (0,1), (-1,0)].iter() {
        let new_row = row as i32 + x;
        let new_col = col as i32 + y;
        
        // Check bounds
        if new_row < 0 || new_row >= heightmap.len() as i32 {continue}
        if new_col < 0 || new_col >= heightmap[0].len() as i32 {continue}
   
        // Map Start/End to correct elevation
        let destination = match heightmap[new_row as usize][new_col as usize] as char {
            'S' => 'a' as i32,
            'E' => 'z' as i32,
            other => other as i32
        };
        let current = match heightmap[row][col] as char {
            'S' => 'a' as i32,
            'E' => 'z' as i32,
            other => other as i32
        };

        // Check adjacent
        if (destination - current) <= 1 {neighbors.push((new_row as usize, new_col as usize))}  
    }

   return neighbors;
}

fn bfs(heightmap: &Vec<&[u8]>, starts: Vec<(usize,usize)>, end: (usize,usize)) -> HashMap<(usize,usize), (usize,usize)> {
    // BFS
    let mut q = VecDeque::<(usize, usize)>::from(starts.clone());
    let mut visited = vec![vec![false; heightmap[0].len()]; heightmap.len()];
    for start in starts {visited[start.0][start.1] = true;}
    let mut path_trace = HashMap::new();

    while !q.is_empty() {
        let pos = q.pop_front().unwrap();
        if pos == end {break} 
        for (row, col) in get_neighbors(heightmap, pos.0, pos.1) {
            if !visited[row][col] {
                visited[row][col] = true;
                path_trace.insert((row,col), pos);
                q.push_back((row,col));
            }    
        }
    }
    return path_trace;
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = &input[..input.len()-1];
    let heightmap: Vec<&[u8]> = input.split("\n").map(|line| line.as_bytes()).collect();
    
    // Find starting and ending locations
    let mut start = (0,0);
    let mut end = (0,0);
    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            if heightmap[y][x] == 'S' as u8 {start = (y,x)}
            else if heightmap[y][x] == 'E' as u8 {end = (y,x)}
        }
    }

    let path_trace = bfs(&heightmap, vec![start], end);
        
    // Reconstruct shortest path
    let mut pos = end;
    let mut path_length = 0;
    while pos != start {
        pos = *path_trace.get(&pos)?;
        path_length += 1;
        //println!("{:?}", pos);
    }
    Some(path_length)
}


pub fn part_two(input: &str) -> Option<u32> {
    let input = &input[..input.len()-1];
    let heightmap: Vec<&[u8]> = input.split("\n").map(|line| line.as_bytes()).collect();
    
    // Find starting and ending locations
    let mut starts = Vec::new();
    let mut end = (0,0);
    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            if heightmap[y][x] == 'S' as u8 || heightmap[y][x] == 'a' as u8 {starts.push((y,x))}
            else if heightmap[y][x] == 'E' as u8 {end = (y,x)}
        }
    }

    let path_trace = bfs(&heightmap, starts, end);
    
    // Reconstruct shortest path
    let mut pos = end;
    let mut path_length = 0;
    while heightmap[pos.0][pos.1] != 'a' as u8 {
        pos = *path_trace.get(&pos)?;
        path_length += 1;
        //println!("{:?}", pos);
    }
    Some(path_length)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
