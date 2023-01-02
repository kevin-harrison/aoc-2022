use std::str::FromStr;
use self::Move::*;


#[derive(PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

pub trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for Move {
    fn beats(&self) -> Self {
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

pub trait BeatenBy {
    fn beaten_by(&self) -> Self;
}

impl BeatenBy for Move {
    fn beaten_by(&self) -> Self {
        match *self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

#[derive(Debug)]
pub struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
         match s {
         "A" => Ok(Rock),
         "B" => Ok(Paper),
         "C" => Ok(Scissors),
         "X" => Ok(Rock),
         "Y" => Ok(Paper),
         "Z" => Ok(Scissors),
         _ => Err(ParseMoveError)
      }
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;

    for line in input.split("\n") {
        if line.len() == 0 {break;}
        let (opponents_str, response_str) = (&line[0..1], &line[2..3]);
        let opponents_move = Move::from_str(opponents_str).unwrap();
        let response_move = Move::from_str(response_str).unwrap();
        
        if response_move.beats() == opponents_move {score += 6} // Win
        else if opponents_move.beats() != response_move {score += 3} // Draw
        match response_move {
            Rock => score += 1,
            Paper => score += 2,
            Scissors => score += 3
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;

    for line in input.split("\n") {
        if line.len() == 0 {break;}
        let (opponents_str, response_str) = (&line[0..1], &line[2..3]);
        let opponents_move = Move::from_str(opponents_str).unwrap();
        
        let response_move = match response_str {
            "X" => opponents_move.beats(),
            "Y" => {score += 3; opponents_move},
            "Z" => {score += 6; opponents_move.beaten_by()},
            _ => Rock // should never happen
        };

        match response_move {
            Rock => score += 1,
            Paper => score += 2,
            Scissors => score += 3
        }
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
