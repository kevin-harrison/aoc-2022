use regex::Regex;
use std::{cmp, collections::HashSet};

pub fn part_one(input: &str) -> Option<i32> {
    let input = input.trim();
    let int_regex = Regex::new(r"(-?\d+)").unwrap();
    const ROW_TO_CHECK: i32 = 10;// change to 2000000 for solving
    let mut intervals: Vec<(i32, i32)> = vec![];
    let mut beacons_on_row = HashSet::<i32>::new();

    for line in input.split("\n") {
        let coords: Vec<i32> = int_regex
            .find_iter(line)
            .map(|num| num.as_str().parse::<i32>().unwrap())
            .collect();
        let sensor_x = coords[0];
        let sensor_y = coords[1];
        let beacon_x = coords[2];
        let beacon_y = coords[3];
        if beacon_y == ROW_TO_CHECK {beacons_on_row.insert(beacon_x);}

        // Add exclusionary interval of row ROW_TO_CHECK
        let width = (beacon_y - sensor_y).abs() + (beacon_x - sensor_x).abs() - (ROW_TO_CHECK - sensor_y).abs();
        if width >= 0 {
            intervals.push((sensor_x - width, sensor_x + width));
        }
    }

    // Merge intervals
    intervals.sort_by(|(a_low,_), (b_low,_)| a_low.cmp(b_low));
    let mut merged_intervals: Vec<(i32, i32)> = vec![];
    for i in 0..intervals.len() {
        if merged_intervals.is_empty() || intervals[i].0 > merged_intervals.last().unwrap().1 {
            merged_intervals.push(intervals[i]);
        } else {
            merged_intervals.last_mut().unwrap().1= cmp::max(merged_intervals.last().unwrap().1, intervals[i].1);
        }
    }
    
    // Answer is sum of interval ranges
    let mut answer = merged_intervals.iter().fold(0, |acc, (low, high)| acc + (high - low) + 1);
    answer -= beacons_on_row.len() as i32; // TODO: THIS IS NOT CORRECT IF BEACON ISNT IN ANY RANGE
    Some(answer)
}


pub fn part_two(input: &str) -> Option<i32> {
    let input = input.trim();
    let int_regex = Regex::new(r"(-?\d+)").unwrap();
    const RANGE: i32 = 20; // change to 4000000 for solving

    let mut lines: Vec<Vec<i32>> = vec![];
    for line in input.split("\n") {
            let coords: Vec<i32> = int_regex
                .find_iter(line)
                .map(|num| num.as_str().parse::<i32>().unwrap())
                .collect();
            lines.push(coords);
    }

    for row in 0..RANGE {
        let mut intervals: Vec<(i32, i32)> = vec![];
        let mut beacons_on_row = HashSet::<i32>::new();

        for coords in lines.iter() {
            
            if coords[3] == row {beacons_on_row.insert(coords[2]);}

            // Add exclusionary interval of row ROW_TO_CHECK
            let width = (coords[3] - coords[1]).abs() + (coords[2] - coords[0]).abs() - (row - coords[1]).abs();
            if width >= 0 {
                intervals.push((coords[0] - width, coords[0] + width));
            }
        }

        // Merge intervals
        intervals.sort_by(|(a_low,_), (b_low,_)| a_low.cmp(b_low));
        let mut merged_intervals: Vec<(i32, i32)> = vec![];
        for i in 0..intervals.len() {
            if merged_intervals.is_empty() || intervals[i].0 > merged_intervals.last().unwrap().1 {
                merged_intervals.push(intervals[i]);
            } else {
                merged_intervals.last_mut().unwrap().1= cmp::max(merged_intervals.last().unwrap().1, intervals[i].1);
            }
        } 
        

        let mut empty_spot = 0;
        for (low, high) in merged_intervals {
            if empty_spot > RANGE {break}
            if empty_spot < low -1 {
                return Some(4000000 * (empty_spot+1) + row);
            }
            if empty_spot <= high {empty_spot = high} 
        }
    }
    None
    
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
