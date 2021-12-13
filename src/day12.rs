use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::BuildHasherDefault;
use std::str::FromStr;

struct Graph<'a> {
    adjacency_list: HashMap<&'a str, HashSet<&'a str>>
}

impl<'a> From<&'a str> for Graph<'a> {
    fn from(s: &'a str) -> Self {
        let mut adjacency_list = HashMap::new();
        s.lines()
            .map(|line| line.split('-'))
            .for_each(|split| {
                match split.collect::<Vec<_>>()[..] {
                    [lhs, rhs] => {
                        // outgoing
                        adjacency_list
                            .entry(lhs)
                            .or_insert_with(|| HashSet::new())
                            .insert(rhs);

                        // incoming
                        adjacency_list
                            .entry(rhs)
                            .or_insert_with(|| HashSet::new())
                            .insert(lhs);
                    }
                    _ => panic!()
                }
            });

        Graph::new(adjacency_list)
    }
}

impl<'a> Graph<'a> {
    fn new(adjacency_list: HashMap<&'a str, HashSet<&'a str>>) -> Self {
        Graph { adjacency_list }
    }

    fn adjacent_nodes_for(&self, node: &str) -> Option<&HashSet<&str>> {
        self.adjacency_list.get(node)
    }
}

pub fn passage_pathing(data: &str) {
    let graph = Graph::from(data);
    println!("{:?}", graph.adjacency_list);
    println!("{}", enumerate_paths(&graph))
}

fn enumerate_paths(graph: &Graph) -> usize {
    let mut next_nodes: VecDeque<(&str, HashSet<&str>, bool)> = VecDeque::new();
    let mut initial_visited = HashSet::new();
    initial_visited.insert("start");

    next_nodes.push_back(("start", initial_visited, true));
    let mut num_paths = 0;

    while !next_nodes.is_empty() {
        let (node, visited, allow_second_traversal) = next_nodes.pop_front().unwrap();

        if node == "end" {
            num_paths += 1;
            continue
        }

        if let Some(adjacent_nodes) = graph.adjacent_nodes_for(node) {
            for adjacent_node in adjacent_nodes {
                if !visited.contains(adjacent_node) {
                    let mut visited_new = visited.clone();
                    if &adjacent_node.to_lowercase().as_str() == adjacent_node && adjacent_node != &"end" {
                        visited_new.insert(adjacent_node);
                    }

                    next_nodes.push_back((adjacent_node, visited_new, allow_second_traversal));
                } else if visited.contains(adjacent_node) && allow_second_traversal && adjacent_node != &"start" && adjacent_node != &"end" {
                    next_nodes.push_back((adjacent_node, visited.clone(), false));
                }
            }
        }
    }

    num_paths
}
