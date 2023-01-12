use lazy_static::lazy_static;

enum Direction {
    Left,
    Right
}

lazy_static! {
    static ref ROCKS: [Vec<u8>; 5] = [
        vec![0b00111100],
        vec![0b00010000,
             0b00111000,
             0b00010000],
        vec![0b00111000,
             0b00001000,
             0b00001000],
        vec![0b00100000,
             0b00100000,
             0b00100000,
             0b00100000],
        vec![0b00110000,
             0b00110000]
    ];
}

struct Chamber{
    state: Vec<u8>,
    max_height: usize,
    rock: Vec<u8>,
    rock_bottom: usize,
    rock_index: usize,
    jets: Vec<Direction>,
    jet_index: usize,
}

impl Chamber {
    const BOTTOM: u8 = 0b11111111;
    const SEGMENT:u8 = 0b00000001;

    fn drop_rock(&mut self) {
        // Get new rock
        self.rock = ROCKS[self.rock_index].clone();
        self.rock_bottom = self.max_height + 4;
        
        // Add segments to fit new rock
        for _ in self.state.len()..self.max_height+8 {
            self.state.push(Chamber::SEGMENT);
        }

        // Simulate rock movements until a collision is found 
        let mut collision = false;
        while !collision {
            match self.jets[self.jet_index] {
                Direction::Left => self.move_rock_left(),
                Direction::Right => self.move_rock_right()
            }
            self.jet_index = (self.jet_index + 1) % self.jets.len();
            
            collision = self.collides(&self.rock, self.rock_bottom - 1);
            if collision {
                self.add_rock(); 
            } else {
                self.rock_bottom -= 1;
            }
        }
        // Set up index for next function call
        self.rock_index = (self.rock_index + 1) % ROCKS.len();
        
    }
    
    fn add_rock (&mut self) {
        // Add rock to tower
        for (i, line) in self.rock.iter().enumerate() {
            self.state[self.rock_bottom + i] |= line; 
        }      
        // Update max height
        self.max_height = std::cmp::max(self.max_height, self.rock_bottom + self.rock.len() - 1);
    }

    fn collides(&self, rock: &Vec<u8>, height: usize) -> bool {
        for (i, line) in rock.iter().enumerate() {
            if (self.state[height + i] & line) != 0 { return true; }
        }
        return false;
    }

    fn move_rock_right(&mut self) {
        // Move rock right
        for line in self.rock.iter_mut() {
            *line >>= 1;
        }
        // If rock collides with anything move it back left
        if self.collides(&self.rock, self.rock_bottom) {
            for line in self.rock.iter_mut() {
                *line <<= 1;
            }          
        } 
    }   

    fn move_rock_left(&mut self) {
        // If MSB is 1 the rock hits the left wall
        for line in self.rock.iter() {
            if (line & (1<<7)) != 0 {return;}
        }
        // Move rock left
        for line in self.rock.iter_mut() {
            *line <<= 1;
        }
        // If rock collides with anything move it back left
        if self.collides(&self.rock, self.rock_bottom) {
            for line in self.rock.iter_mut() {
                *line >>= 1;
            }           
        } 
    }

    fn print(&self) {
        for (i, line) in self.state.iter().enumerate().rev() {
            let in_rock_lines = i >= self.rock_bottom && i < (self.rock_bottom + self.rock.len());
            for j in (0..8).rev() {
                let print_rock_piece = in_rock_lines && ((self.rock[i - self.rock_bottom] >> j) & 1) != 0;
                let print_tower_peice = (line >> j) & 1 != 0;
                let symbol = match (print_rock_piece, print_tower_peice) {
                    (true, _) => "@",
                    (false, true) => "#",
                    (false, false) => "."
                };
                print!("{}", symbol);
            } 
            println!();
        }
        println!();
    } 
}


pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim();
    // Parse jets input
    let mut jet_directions: Vec<Direction> = vec![]; 
    for jet_direction in input.chars() {
        match jet_direction {
            '<' => jet_directions.push(Direction::Left),
            '>' => jet_directions.push(Direction::Right),
            _ => panic!("input parse error")
        }
    }
    // Simulate 2022 rocks falling
    let mut c = Chamber {
        state: vec![Chamber::BOTTOM],
        max_height: 0,
        rock: vec![],
        rock_bottom: 0,
        rock_index: 0,
        jets: jet_directions,
        jet_index: 0
    };
    for _ in 0..2022 {
        c.drop_rock();
    }
    Some(c.max_height)
}

fn find_cycle(seq: &Vec<usize>) -> Option<(usize, usize)>{
    for i in 0..seq.len() {
        let suffix = &seq[i..];
        for r in 2..suffix.len()/2 {
            if suffix[0..r] == suffix[r..2*r] {
                println!("found potential seq at {} {}", i ,r);
                let mut cycle_lasted_to_end = true;
                for y in 1..(suffix.len()/r)-1 {
                    if suffix[0..r] != suffix[y*r..(y+1)*r] {
                        cycle_lasted_to_end = false;
                        break;
                    }
                }
                if cycle_lasted_to_end { return Some((i,r))}
            }
        }
    }
    return None;
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();
    // Parse jets input
    let mut jet_directions: Vec<Direction> = vec![]; 
    for jet_direction in input.chars() {
        match jet_direction {
            '<' => jet_directions.push(Direction::Left),
            '>' => jet_directions.push(Direction::Right),
            _ => panic!("input parse error")
        }
    }
    let mut c = Chamber {
        state: vec![Chamber::BOTTOM],
        max_height: 0,
        rock: vec![],
        rock_bottom: 0,
        rock_index: 0,
        jets: jet_directions,
        jet_index: 0
    };
    // Simulate enough rocks falling to find a repeating pattern
    let mut heights: Vec<usize> = vec![];
    let mut prev_height = 0;
    for _ in 0..10000 {
        c.drop_rock();
        heights.push(c.max_height - prev_height);
        prev_height = c.max_height;
    }
    // Calculate cycle from relative height changes in tower (repeating pattern in heights Vec)
    // We want to find the tower height after n = 10^12 rocks have fallen
    // ∃k∈Z s.t. n = cycle_start + (k * cycle_length) + remainder
    // So the answer = Height after N rocks = height after cycle_start rocks +
    //                                        k * height of rocks in 1 cycle +
    //                                        height of remainder rocks into a cycle
    let (cycle_start, cycle_len) = find_cycle(&heights).expect("Couldn't find cycle");
    let n: usize = 1000000000000;
    let k: usize = n / cycle_len;
    let r: usize = n - (cycle_start + (k * cycle_len)); 
    let answer: usize = heights[..cycle_start].iter().sum::<usize>() + (k * heights[cycle_start..cycle_start+cycle_len].iter().sum::<usize>()) + heights[cycle_start..cycle_start+r].iter().sum::<usize>();
    println!("n={} k={} r={}", n, k, r);
    println!("{}", answer);

    Some(answer)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
