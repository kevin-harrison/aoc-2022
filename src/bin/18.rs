use std::collections::VecDeque;


pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim();

    // Get cubes
    let mut cubes: Vec<(usize,usize,usize)> = vec![];
    for line in input.split("\n") {
        let coords: Vec<usize> = line
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        cubes.push((coords[0]+1,coords[1]+1,coords[2]+1)); // translate the cubes so we can have a
                                                           // buffer of empty areas in the grid for
                                                           // indexing
    }
    println!("Number of cubes: {:?}", cubes.len());

    // Place cubes in 3D grid
    let max_x: usize = cubes.iter().fold(0, |acc, (x,_,_)| if *x > acc {return *x;} else {return acc;});
    let max_y: usize = cubes.iter().fold(0, |acc, (_,y,_)| if *y > acc {return *y} else {return acc;});
    let max_z: usize = cubes.iter().fold(0, |acc, (_,_,z)| if *z > acc {return *z;} else {return acc;});
    println!("max x = {}", max_x);
    println!("max y = {}", max_y);
    println!("max z = {}", max_z);
    let mut grid: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; max_z+2]; max_y+2]; max_x+2];
    for cube in cubes.iter() {
        grid[cube.0][cube.1][cube.2] = true;
    }

    // Calculate surface areas
    let mut open_faces: u32 = 0;
    for &(x,y,z) in cubes.iter() {
        if !grid[x+1][y][z] {open_faces += 1;}
        if !grid[x+1][y][z] {open_faces += 1;}
        if !grid[x][y+1][z] {open_faces += 1;}
        if !grid[x][y-1][z] {open_faces += 1;}
        if !grid[x][y][z+1] {open_faces += 1;}
        if !grid[x][y][z-1] {open_faces += 1;}
    }
    Some(open_faces)
}

fn in_bounds(x: usize, y: usize, z: usize, grid: &Vec<Vec<Vec<bool>>>) -> bool {
    let in_bounds_x = x > 0 && x < grid.len();
    let in_bounds_y = y > 0 && y < grid[0].len();
    let in_bounds_z = z > 0 && z < grid[0][0].len();
    return in_bounds_x && in_bounds_y && in_bounds_z;
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.trim();

    // Get cubes
    let mut cubes: Vec<(usize,usize,usize)> = vec![];
    for line in input.split("\n") {
        let coords: Vec<usize> = line
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        cubes.push((coords[0]+2,coords[1]+2,coords[2]+2)); // translate the cubes so we can have a
                                                           // buffer of empty areas in the grid for
                                                           // indexing
    }
    println!("Number of cubes: {:?}", cubes.len());

    // Place cubes in 3D grid
    let max_x: usize = cubes.iter().fold(0, |acc, (x,_,_)| if *x > acc {return *x;} else {return acc;});
    let max_y: usize = cubes.iter().fold(0, |acc, (_,y,_)| if *y > acc {return *y} else {return acc;});
    let max_z: usize = cubes.iter().fold(0, |acc, (_,_,z)| if *z > acc {return *z;} else {return acc;});
    println!("max x = {}", max_x);
    println!("max y = {}", max_y);
    println!("max z = {}", max_z);
    let mut grid: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; max_z+2]; max_y+2]; max_x+2];
    for cube in cubes.iter() {
        grid[cube.0][cube.1][cube.2] = true;
    }

    // BFS to label open-air spots
    let mut exterior_grid: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; max_z+2]; max_y+2]; max_x+2];
    let mut q = VecDeque::<(usize,usize,usize)>::from([(1,1,1)]);
    exterior_grid[1][1][1] = true;
    while !q.is_empty() {
        let (x,y,z) = q.pop_front().unwrap();
        if grid[x][y][z] {continue;}
        for &(next_x, next_y, next_z) in [(x+1,y,z),(x-1,y,z),(x,y+1,z),(x,y-1,z),(x,y,z+1),(x,y,z-1)].iter() {
            if in_bounds(next_x, next_y, next_z, &grid) && !exterior_grid[next_x][next_y][next_z] {
                exterior_grid[next_x][next_y][next_z] = true;
                q.push_back((next_x, next_y, next_z));
            }
        }
    }

    // Calculate surface areas
    let mut open_faces: u32 = 0;
    for &(x,y,z) in cubes.iter() {
        if !grid[x+1][y][z] && exterior_grid[x+1][y][z] {open_faces += 1;}
        if !grid[x+1][y][z] && exterior_grid[x+1][y][z] {open_faces += 1;}
        if !grid[x][y+1][z] && exterior_grid[x][y+1][z] {open_faces += 1;}
        if !grid[x][y-1][z] && exterior_grid[x][y-1][z] {open_faces += 1;}
        if !grid[x][y][z+1] && exterior_grid[x][y][z+1] {open_faces += 1;}
        if !grid[x][y][z-1] && exterior_grid[x][y][z-1] {open_faces += 1;}
    }
    Some(open_faces)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
