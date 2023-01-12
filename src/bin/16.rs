use std::{cmp, collections::HashMap};
use regex::Regex;
use itertools::Itertools;


struct ValveGraph {
    labels: HashMap<String, usize>,
    pressures: Vec<u32>,
    adj_list: Vec<Vec<(usize, u32)>>,
    num_valves: usize
}


impl ValveGraph {
    fn get_valve_label(&self, index: usize) -> &String {
        return self.labels.iter().find(|&(_,&v)| v == index).expect("Valve with this index doesn't exist").0;
    }

    fn add_valve(&mut self, label: &str, pressure: u32) {
        self.labels.insert(label.to_string(), self.num_valves);
        self.pressures.push(pressure);
        self.adj_list.push(vec![]);
        self.num_valves += 1;
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: u32) {
        self.adj_list[from].push((to, weight));
    }

    fn add_path(&mut self, from: usize, to: usize, weight: u32) {
        let edge = self.adj_list[from].iter().filter(|(edge_to, _)| *edge_to == to).next();
        match edge {
            Some((_, edge_weight)) => {
                // replace edge if path is shorter
                if weight < *edge_weight {
                   self.adj_list[from].retain(|(edge_to, _)| *edge_to != to);  
                   self.adj_list[to].retain(|(edge_from, _)| *edge_from != from);  
                   self.add_edge(from, to, weight);
                   self.add_edge(to, from, weight);
                } 
            }
            None => {
                self.add_edge(from, to, weight);
                self.add_edge(to, from, weight);
            }
        }
        
    }

    fn add_edge_by_label(&mut self, from: &str, to: &str, weight: u32) {
        let &from = self.labels.get(from).expect("Valve with this label not found!");
        let &to = self.labels.get(to).expect("Valve with this label not found!");
        self.add_edge(from, to, weight);
    }

    fn parse_valves(&mut self, input: &str) {
        let re = Regex::new(r"\d+").unwrap();
        for line in input.split("\n") {
            let pressure = re
                .find(line)
                .map(|x| x.as_str().parse::<u32>().unwrap())
                .unwrap();
            self.add_valve(&line[6..8], pressure);
        }
    }

    fn compress_graph(&mut self) { 
        // Starting room has 0 pressure but we dont want to remove it
        self.pressures[0] = 1;

        for i in 1..self.num_valves {
            if self.pressures[i] == 0 {
                println!("{}", self.get_valve_label(i));
                println!("Original edges: {:?}", self.adj_list[i]);

                // Connect neighbors
                let new_edges = self.adj_list[i].clone().into_iter().combinations(2);
                for pair in new_edges {
                    let from = pair[0].0;
                    let to = pair[1].0;
                    let weight = pair[0].1 + pair[1].1;
                    println!("New edge {:?}", (from, to, weight));
                    //self.add_path(from, to, weight);
                    self.add_edge(from, to, weight);
                    self.add_edge(to, from, weight);
                }

                // Remove incoming edges
                for j in 0..self.adj_list[i].len() {
                    let incoming_node = self.adj_list[i][j].0;
                    println!("Removing incoming edge {} to {} ", incoming_node, i);
                    self.adj_list[incoming_node].retain(|(n, _)| *n != i);
                }
                println!("");
            }
        }

        // Calculate index remap
        let mut new_labels = HashMap::<String,usize>::new();
        let mut index_mapping = HashMap::<usize,usize>::new();
        for (new_index, (old_index, _)) in self.pressures.iter().enumerate().filter(|(_, p)| **p > 0).enumerate() {
            index_mapping.insert(old_index, new_index);
            let label = self.get_valve_label(old_index);
            new_labels.insert(label.clone(), new_index);
        }

        // Remove 0-pressure valves
        self.labels = new_labels;
        let mut valves_to_keep = self.pressures.iter().map(|&p| p > 0);
        self.adj_list.retain(|_| valves_to_keep.next().unwrap());
        self.pressures.retain(|&p| p > 0);
        self.pressures[0] = 0; // reset starting room
        self.num_valves = self.pressures.len();
        
        // Remap indices
        for i in 0..self.num_valves {
            for j in 0..self.adj_list[i].len() {
                let (index, weight) = self.adj_list[i][j];
                self.adj_list[i][j] = (*index_mapping.get(&index).unwrap(), weight);
            }
        }                
    }
}

fn max_pressure(graph: &ValveGraph, cache: &mut Vec<i32>, mut used: u128, position: usize, time: u128) -> i32 {
    // Base cases
    if time == 0 {return 0;}
    if used == (1 << graph.num_valves) - 1 {return 0;}
    
    // Check for precalcuated result. Limits: time < 31, #valves < 116
    let state_key = (used << (5 + 7)) | ((position as u128) << 5) | time; 
    //let state_key = (used * graph.num_valves as u128*31*2) + ((position as u128) *31*2) + time;
    if cache[state_key as usize] >= 0 {return cache[state_key as usize];}

    // Get best movement option
    let best_movement = graph.adj_list[position]
        .iter()
        // TODO; only check neighbors reachable with time left
        .filter(|(_, weight)| time >= *weight as u128)
        .map(|&(new_position, weight)| max_pressure(graph, cache, used, new_position, time - weight as u128))
        .max()
        .unwrap_or(0);
    
    // Get pressure gained by opening current valve
    let not_opened = (used & (1 << position)) == 0;
    let valve_pressure = match not_opened {
        false => 0,
        true => {
            used |= 1 << position;
            graph.pressures[position] * ((time as u32) - 1)
        }
    };

    let answer = cmp::max(valve_pressure as i32 + max_pressure(graph, cache, used, position, time - 1), best_movement);
    cache[state_key as usize] = answer;
    return answer;
}

fn max_pressure2(graph: &ValveGraph, cache: &mut HashMap<u128, u32>, mut used: u128, position: usize, time: u128) -> u32 {
    // Base cases
    if time == 0 {return 0;}
    if used == (1 << graph.num_valves) - 1 {return 0;}
    
    // Check for precalcuated result. Limits: time < 31, #valves < 116
    let state_key = (used << (5 + 7)) | ((position as u128) << 5) | time; 
    //let state_key = (used * graph.num_valves as u128*31*2) + ((position as u128) *31*2) + time;
    //if cache[state_key as usize] >= 0 {return cache[state_key as usize];}
    match cache.get(&state_key) {
        Some(answer) => {return *answer},
        None => ()
    }

    // Get best movement option
    let best_movement = graph.adj_list[position]
        .iter()
        .filter(|(_, weight)| time >= *weight as u128)
        .map(|&(new_position, weight)| max_pressure2(graph, cache, used, new_position, time - (weight as u128)))
        .max()
        .unwrap_or(0);
    
    // Get pressure gained by opening current valve
    let not_opened = (used & (1 << position)) == 0;
    let valve_pressure = match not_opened {
        false => 0,
        true => {
            used |= 1 << position;
            graph.pressures[position] * ((time as u32) - 1)
        }
    };

    let answer = cmp::max(valve_pressure + max_pressure2(graph, cache, used, position, time - 1), best_movement);
    let uhoh = cache.insert(state_key, answer);
    match uhoh {
        Some(_) => panic!("UHOH"),
        None => ()
    }
    return answer;
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim();
    let mut graph: ValveGraph = ValveGraph { 
        labels: HashMap::new(), 
        pressures: vec![], 
        adj_list: vec![], 
        num_valves: 0 
        };
    graph.parse_valves(input);

    let valve_regex = Regex::new(r"([A-Z]{2})").unwrap();
    for line in input.split("\n") {
        for label_match in valve_regex.find_iter(line).skip(1) {
            graph.add_edge_by_label(&line[6..8], label_match.as_str(), 1);
        }
    }

    graph.compress_graph();
    println!("{:?}", graph.labels);
    println!("{:?}", graph.pressures);
    let adjs: Vec<Vec<(&String, u32)>> = graph.adj_list
        .iter()
        .map(|edges| edges.iter().map(|(e, w)| (graph.get_valve_label(*e), *w)).collect())
        .collect();
    for edge in adjs.iter() {
        println!("{:?}", edge);
    }

    //let answer = max_pressure(&graph, &mut vec![-1; (1<<graph.num_valves) * graph.num_valves * 31 * 2], 0, 0, 30);
    let answer = max_pressure2(&graph, &mut HashMap::new(), 0, 0, 30);
    println!("answer={}", answer);
    None
}


pub fn part_two(input: &str) -> Option<u32> {
    None
}

 fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
