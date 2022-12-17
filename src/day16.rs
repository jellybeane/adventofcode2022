use std::{vec, collections::HashMap, cmp::max};

use petgraph::{Graph, Undirected, prelude::NodeIndex};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

// the map of valve indices to flow rates,
// map of valve names to valve indices,
// and the graph of tunnel connections
type Data = (HashMap<NodeIndex,usize>,
             HashMap<String,NodeIndex>,
            Graph<String, usize, Undirected>);

// Format is 
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    // map of valves to their flow rates
    let mut rate_map = HashMap::new();
    // map of valve names to graph indices
    let mut valve_name_map = HashMap::new();
    // map of graph indices to valve names, for convenience
    let mut valve_index_map = HashMap::new();
    // the actual graph
    let mut graph: Graph<String,usize,Undirected> = Graph::new_undirected();

    let mut edges = HashMap::new();
    for line in input.lines() {
        let (valve_str, tunnel_str) = line.split_once("; ").unwrap();

        // Valve AA has flow rate=0
        let (valve_str, flow_rate_str) = valve_str.split_once(" has flow rate=").unwrap();
        let (_, valve_str) = valve_str.split_once(" ").unwrap();
        let valve_str = valve_str.to_string();
        let flow_rate:usize = flow_rate_str.parse()?;

        let node_index = graph.add_node(valve_str.clone());
        rate_map.insert(node_index, flow_rate);
        valve_name_map.insert(valve_str.clone(), node_index);
        valve_index_map.insert(node_index, valve_str.clone());

        // tunnels lead to valves DD, II, BB
        // grammar: there might only be one tunnel
        let tunnel_str = match tunnel_str.strip_prefix("tunnels lead to valves ") {
            Some(many_tunnel) => many_tunnel,
            None => tunnel_str.strip_prefix("tunnel leads to valve ").unwrap(),
        };
        // put the names of the destinations since they might not have assigned indices yet
        let connected: Vec<&str> = tunnel_str.split(", ").collect();
        edges.insert(node_index, connected);

    }
    // all the nodes are added: now add the edges
    for source in graph.node_indices() {
        // Valves with flow rate 0 can be replaced by longer tunnels
        // except AA because that's where we start
        let &source_rate = rate_map.get(&source).unwrap();
        let source_name = valve_index_map.get(&source).unwrap();
        if source_rate == 0 && source_name != "AA" {
            continue;
        }

        let destinations = edges.get(&source).unwrap();
        let mut destinations: Vec<(String, Vec<NodeIndex>)> = destinations
            .iter().map(|&d| (d.to_string(), vec![source])).collect();

        while !destinations.is_empty() {
            let (destination_name, predecessors) = destinations.pop().unwrap();
            let &destination = valve_name_map.get(&destination_name).unwrap();
            let &dest_rate = rate_map.get(&destination).unwrap();
            if dest_rate != 0 || destination_name == "AA" {
                // a regular tunnel from source to dest
                graph.update_edge(source, destination, predecessors.len());
            }
            else {
                // treat this valve as part of a longer tunnel
                let mut further_predecessors = predecessors.clone();
                further_predecessors.push(destination);
                for &further_name in edges.get(&destination).unwrap() {
                    let &further = valve_name_map.get(further_name).unwrap();
                    if !predecessors.contains(&further) {
                        destinations.push((further_name.to_string(), further_predecessors.clone()))
                    }
                }
            }
        }
    }

    Ok((rate_map, valve_name_map, graph))
}

// brute force DFS
// returns the highest pressure it saw in 30 steps
fn dfs(g: &Graph<String, usize, Undirected>, 
    cur_node:NodeIndex, cur_path:&mut Vec<NodeIndex>, cur_step:usize, max_steps:usize,
    indices_to_rates:&HashMap<NodeIndex,usize>, total_rate:usize, cumulative_pressure:usize) // this feels bad ._.
    -> usize
{
    // want total pressure by minute 30
    if cur_step >= max_steps {
        return cumulative_pressure;
    }

    // the max if all valves are open
    let max_rate: usize = indices_to_rates.values().sum();
    // if we've hit max rates, no point in continuing down
    // return with the pressure that would accumulate in the remaining time
    if total_rate == max_rate {
        let remaining = max_steps - cur_step;
        let updated_pressure = cumulative_pressure + (max_rate * remaining);
        return updated_pressure;
    }

    // continue down the path
    // two actions we can take: open valve or move
    // open valve the first time it is seen
    let mut max_seen = cumulative_pressure;
    if !cur_path.contains(&cur_node) {
        let &my_rate = indices_to_rates.get(&cur_node).unwrap();
        cur_path.push(cur_node);
        max_seen = dfs(g, cur_node, cur_path, cur_step+1, 
            max_steps, indices_to_rates, 
            total_rate + my_rate, total_rate + cumulative_pressure);
    }
    // move to neighbor
    else {
        cur_path.push(cur_node);
        // backtracking hack: the best path probably only backtracks at dead ends
        let prev_node = if cur_path.len() > 0 {
            Some(*(cur_path.last().unwrap()))
        }
        else {
            None
        };
        for neighbor in g.neighbors(cur_node)
        {   
            if Some(neighbor) != prev_node || g.neighbors(cur_node).count() == 1 {
                // there should only be one edge connecting cur_node to neigbor
                let edge = g.edges_connecting(cur_node, neighbor).next().unwrap();
                let mut updated_pressure = cumulative_pressure;
                let mut updated_step = cur_step;
                for _ in 0..*edge.weight() {
                    updated_pressure += total_rate;
                    updated_step += 1;
                    if cur_step >= max_steps {
                        return updated_pressure;
                    }
                }
                let p = dfs(g, neighbor, cur_path, updated_step,
                    max_steps, indices_to_rates, 
                    total_rate, updated_pressure );
                max_seen = max(max_seen, p);
            }
        }
    }
    
    // reset this node before returning
    cur_path.pop();

    max_seen

}

// Takes 1 minute to move through a tunnel
// Can only open valves you are at
// What is the most pressure you can release in 30 minutes?
#[aoc(day16, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let (rate_map, valve_name_map, tunnels) = input.clone();
    dbg!(&tunnels);
    //dbg!(&rate_map);

    // start at valve AA
    let &cur_node = valve_name_map.get("AA").unwrap();
    let mut cur_path = vec![];
    dfs(&tunnels, cur_node, &mut cur_path, 0, 30, 
        &rate_map, 0, 0)
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 1651);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}