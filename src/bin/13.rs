use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Value(i32),
}

#[derive(Debug)]
struct ParsePacketError;

impl Packet {
    fn next_token(s: &str, token_start: usize) -> &str {
        // No tokens start with a comma
        if &s[token_start..token_start + 1] == "," {
            panic![]
        }

        let mut token_end = token_start + 1;
        // Token is a list, find closing bracket
        if &s[token_start..token_start + 1] == "[" {
            let mut depth = 1;
            for character in &mut s[token_start + 1..].chars() {
                match character {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => (),
                }
                token_end += 1;
                if depth == 0 {
                    break;
                }
            }
        }
        // Token is a value, find delimiter
        else {
            for character in &mut s[token_start + 1..].chars() {
                match character {
                    ',' => break,
                    ']' => break,
                    _ => (),
                }
                token_end += 1;
            }
        }
        return &s[token_start..token_end];
    }

    fn parse_num(s: &str, token_start: &mut usize) -> Self {
        let num_str = Packet::next_token(s, *token_start);
        *token_start += num_str.len();
        return Packet::Value(num_str.parse::<i32>().unwrap());
    }

    fn parse_list(s: &str, token_start: &mut usize) -> Self {
        let list_str = Packet::next_token(s, *token_start);
        let list_end = *token_start + list_str.len();
        let mut list: Vec<Packet> = Vec::new();
        *token_start += 1; // consume [

        while *token_start < list_end {
            match &s[*token_start..*token_start + 1] {
                "," => *token_start += 1,
                "]" => *token_start += 1,
                "[" => list.push(Packet::parse_list(s, token_start)),
                _ => list.push(Packet::parse_num(s, token_start)),
            }
        }

        return Packet::List(list);
    }
}

impl FromStr for Packet {
    type Err = ParsePacketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Packet::parse_list(s, &mut 0))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(a), Packet::Value(b)) => return a.cmp(b),
            (Packet::Value(a), Packet::List(_)) => {
                return Packet::List(vec![Packet::Value(*a)]).cmp(other)
            }
            (Packet::List(_), Packet::Value(a)) => {
                return self.cmp(&Packet::List(vec![Packet::Value(*a)]))
            }
            (Packet::List(s), Packet::List(o)) => {
                let mut i = 0;
                let mut j = 0;
                while i < s.len() && j < o.len() {
                    let ord = s[i].cmp(&o[j]);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                    i += 1;
                    j += 1;
                }

                return s.len().cmp(&o.len());
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let packets: Vec<&str> = input.split("\n").filter(|&line| !line.is_empty()).collect();
    let mut answer = 0;

    for (i, packet_pair) in packets.chunks(2).enumerate() {
        let left = Packet::from_str(packet_pair[0]).unwrap();
        let right = Packet::from_str(packet_pair[1]).unwrap();

        if left < right {
            answer += i + 1;
        }
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<usize> {
    // Parse packets
    let mut packets: Vec<Packet> = input
        .split("\n")
        .filter(|&line| !line.is_empty())
        .map(|packet| Packet::from_str(packet).unwrap())
        .collect();
   
    // Add divider packets
    let divider_packet_1 = Packet::from_str("[[2]]").unwrap();
    let divider_packet_2 = Packet::from_str("[[6]]").unwrap();
    packets.push(divider_packet_1.clone());
    packets.push(divider_packet_2.clone());
    
    // Find sorted indices of divider packets
    packets.sort();
    let mut answer = 1;
    for (i, packet) in packets.iter().enumerate() {
        if packet == &divider_packet_1 || packet == &divider_packet_2 {
            answer *= i + 1;
        }
    }
    Some(answer)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
