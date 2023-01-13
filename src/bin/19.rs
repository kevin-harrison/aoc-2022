use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost: (u32, u32),
    geode_robot_cost: (u32, u32),
    max_ore_use_per_turn: u32,
    max_clay_use_per_turn: u32,
    max_obsidian_use_per_turn: u32,
}

#[derive(Debug, Clone, Copy)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32, 
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32
}

#[derive(Debug)]
struct ParseBlueprintError;

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\d+").unwrap();
        let parsed_ints: Vec<u32>  = re
            .find_iter(s)
            .map(|d| d.as_str().parse().unwrap())
            .collect();
        let blueprint = Blueprint {
            id: parsed_ints[0],
            ore_robot_cost: parsed_ints[1],
            clay_robot_cost: parsed_ints[2],
            obsidian_robot_cost: (parsed_ints[3], parsed_ints[4]),
            geode_robot_cost: (parsed_ints[5],parsed_ints[6]),
            max_ore_use_per_turn: *[parsed_ints[1], parsed_ints[2], parsed_ints[3], parsed_ints[5]].iter().max().unwrap(),
            max_clay_use_per_turn: parsed_ints[4],
            max_obsidian_use_per_turn: parsed_ints[6]
        };
        println!("{:?}", blueprint);
        Ok(blueprint)
    }    
}

impl Blueprint {
    fn get_quality(&self) -> u32 {
        let starting_state = State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0
        };
        return self.quality(24, starting_state, &mut 0); 
    }

    fn quality(&self, time: u32, mut state: State, best: &mut u32) -> u32 {
        // Base case
        if time == 0 {
            return state.geodes;
        }
        // Abondon search path if already found a better one
        if state.geodes > *best {
            *best = state.geodes;
        }
        let upperbound = state.geodes + ((time * (state.geode_robots + state.geode_robots + time)) / 2);
        if upperbound < *best {
            return 0;
        }
        

        let can_build_ore_robot =      state.ore >= self.ore_robot_cost                                                        && state.ore_robots < self.max_ore_use_per_turn; 
        let can_build_clay_robot =     state.ore >= self.clay_robot_cost                                                       && state.clay_robots < self.max_clay_use_per_turn;
        let can_build_obsidian_robot = (state.ore >= self.obsidian_robot_cost.0) && (state.clay >= self.obsidian_robot_cost.1) && state.obsidian_robots < self.max_obsidian_use_per_turn;
        let can_build_geode_robot =    (state.ore >= self.geode_robot_cost.0) && (state.obsidian >= self.geode_robot_cost.1);

        state.ore += state.ore_robots;
        state.clay += state.clay_robots;
        state.obsidian += state.obsidian_robots;
        state.geodes += state.geode_robots;

        // Choose best turn action
        let mut max_geodes = self.quality(time-1, state, best); // Do nothing this turn
        let mut next_state;

        // Build ore robot
        if can_build_ore_robot {
            next_state = state.clone();
            next_state.ore -= self.ore_robot_cost;
            next_state.ore_robots += 1;
            max_geodes = max_geodes.max(self.quality(time-1, next_state, best));
        }
        // Build clay robot
        if can_build_clay_robot {
            next_state = state.clone();
            next_state.ore -= self.clay_robot_cost;
            next_state.clay_robots += 1;
            max_geodes = max_geodes.max(self.quality(time-1, next_state, best));
        }
        // Build obsidian robot
        if can_build_obsidian_robot {
            next_state = state.clone();
            next_state.ore -= self.obsidian_robot_cost.0;
            next_state.clay -= self.obsidian_robot_cost.1;
            next_state.obsidian_robots += 1;
            max_geodes = max_geodes.max(self.quality(time-1, next_state, best));
        }
        // Build geode robot
        if can_build_geode_robot {
            next_state = state.clone();
            next_state.ore -= self.geode_robot_cost.0;
            next_state.obsidian -= self.geode_robot_cost.1;
            next_state.geode_robots += 1;
            max_geodes = max_geodes.max(self.quality(time-1, next_state, best));
        }
        return max_geodes;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim();
    for blueprint_str in input.split("\n") {
        let blueprint = Blueprint::from_str(blueprint_str).expect("Couldn't parse a blueprint");
        println!("Quality was {}", blueprint.get_quality());
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
