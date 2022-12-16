use std::{vec, collections::HashMap, cmp::max};

use petgraph::{Graph, Undirected, prelude::NodeIndex};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

// the map of valve indices to flow rates,
// map of valve names to valve indices,
// and the graph of tunnel connections
type Data = (HashMap<NodeIndex,usize>,
             HashMap<String,NodeIndex>,
            Graph<usize, usize, Undirected>);

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
    // the actual graph
    let mut graph: Graph<usize,usize,Undirected> = Graph::new_undirected();
    // name of one node to another node
    let mut edges = vec![];
    for line in input.lines() {
        let (valve_str, tunnel_str) = line.split_once("; ").unwrap();

        // Valve AA has flow rate=0
        let (valve_str, flow_rate_str) = valve_str.split_once(" has flow rate=").unwrap();
        let (_, valve_str) = valve_str.split_once(" ").unwrap();
        let valve_str = valve_str.to_string();
        let flow_rate:usize = flow_rate_str.parse()?;

        let node_index = graph.add_node(flow_rate);
        rate_map.insert(node_index, flow_rate);
        valve_name_map.insert(valve_str.clone(), node_index);

        // tunnels lead to valves DD, II, BB
        // grammar: there might only be one tunnel
        let tunnel_str = match tunnel_str.strip_prefix("tunnels lead to valves ") {
            Some(many_tunnel) => many_tunnel,
            None => tunnel_str.strip_prefix("tunnel leads to valve ").unwrap(),
        };
        let connected: Vec<&str> = tunnel_str.split(", ").collect();

        for destination_str in connected {
            edges.push((valve_str.clone(), destination_str.to_string()));
        }
    }
    // all the nodes are added: now add the edges
    for (a_name, b_name) in edges{
        let &a = valve_name_map.get(&a_name).unwrap();
        let &b = valve_name_map.get(&b_name).unwrap();
        // all edges have weight 1
        // it's an undirected graph so use update_edge so we don't have duplicates
        graph.update_edge(a, b, 1);
    }

    Ok((rate_map, valve_name_map, graph))
}

// brute force DFS: it's too late for smarts
// seems to be working on paths up to len 20-ish but too slow for longer
// returns the highest pressure it saw in paths of length 30
fn dfs(g: &Graph<usize, usize, Undirected>, 
    cur_node:NodeIndex, cur_path:&mut Vec<NodeIndex>, 
    indices_to_rates:&HashMap<NodeIndex,usize>, total_rate:usize, cumulative_pressure:usize, opened_on_prev_step:bool) // this feels bad ._.
    -> usize
{
    // want total pressure by minute 30
    let maxlen = 25;

    // did we open a valve last node?
    let mut updated_rate = total_rate;
    let mut updated_pressure = cumulative_pressure;
    if opened_on_prev_step {
        let prev_node = cur_path.last().unwrap();
        updated_rate += indices_to_rates.get(prev_node).unwrap();
    }

    // the max if all valves are open
    let max_rate: usize = indices_to_rates.values().sum();
    // if we've hit max rates, no point in continuing down
    // return with the pressure that would accumulate in the remaining time
    if updated_rate == max_rate {
        let remaining = maxlen - cur_path.len();
        updated_pressure += max_rate * remaining;
        return updated_pressure;
    }

    // otherwise, increment by one timestep and continue on
    updated_pressure += updated_rate;

    // is this the first time we encounter this valve?
    let opened_this_step = !cur_path.contains(&cur_node);

    // update the path
    cur_path.push(cur_node);
    if cur_path.len() >= maxlen {
        assert!(cur_path.len() == maxlen);//should never get past 30???
        cur_path.pop();
        return updated_pressure;
    }

    // continue down the path
    let mut max_seen = updated_pressure;
    for neighbor in g.neighbors(cur_node)
    {
        let p = dfs(g, neighbor, cur_path, 
            indices_to_rates, 
            updated_rate, updated_pressure, opened_this_step);
        max_seen = max(max_seen, p);
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

    //dbg!(&rate_map);

    // start at valve AA
    let &cur_node = valve_name_map.get("AA").unwrap();
    let mut cur_path = vec![];
    dfs(&tunnels, cur_node, &mut cur_path, &rate_map, 0, 0, false)
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