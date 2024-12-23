use std::collections::{HashMap, HashSet};

fn register_node(from: &str, to: &str, nodes: &mut HashMap<String, HashSet<String>>) {
    if let Some(connections) = nodes.get_mut(from) {
        connections.insert(to.to_string());
    }
    else {
        let mut connections = HashSet::new();
        connections.insert(to.to_string());
        nodes.insert(from.to_string(), connections);
    }
}

fn cycle_id(node1: &String, node2: &String, node3: &String) -> String {
    let mut strings = vec![node1.to_string(), node2.to_string(), node3.to_string()];
    strings.sort();
    strings.concat()
}

#[allow(dead_code)]
pub fn part1() -> String {
    let connections = aoc::to_lines("input/day23.txt");
    let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();
    let mut unique_nodes: HashSet<String> = HashSet::new();

    for connection in connections {
        let parts: Vec<&str> = connection.split('-').collect();

        if let [node1, node2] = parts.as_slice() {
            unique_nodes.insert(node1.to_string());
            register_node(node1, node2, &mut nodes);
            unique_nodes.insert(node2.to_string());
            register_node(node2, node1, &mut nodes);
        } else {
            panic!("Input does not match the expected format");
        }
    }

    let mut unique_cycles: HashSet<String> = HashSet::new();
    for (node1, connections1) in &nodes {
        for (node2, connections2) in &nodes {
            if node1 == node2 {
                continue;
            }

            if !connections2.contains(node1) {
                continue;
            }

            let mut has_t = false;
            if let Some(letter) = node1.chars().nth(0) {
                if letter == 't' {
                    has_t = true;
                }
            }
            let mut has_t = false;
            if let Some(letter) = node2.chars().nth(0) {
                if letter == 't' {
                    has_t = true;
                }
            }

            for node3 in &unique_nodes {
                if node1 == node3 || node2 == node3 {
                    continue;
                }

                if !connections2.contains(node3) || !connections1.contains(node3) {
                    continue;
                }

                let mut has_t = has_t;
                if let Some(letter) = node3.chars().nth(0) {
                    if letter == 't' {
                        has_t = true;
                    }
                }

                if has_t {
                    unique_cycles.insert(cycle_id(node1, node2, node3));
                }
            }
        }
    }
    println!("{unique_cycles:?}");

    unique_cycles.len().to_string()
}

fn bron_kerbosch(
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    let mut p_clone = p.clone(); // Clone P since we modify it during iteration
    for v in p_clone.iter() {
        let mut new_r = r.clone();
        new_r.insert(v.clone());

        //let neighbors = graph.get(v).unwrap_or(&HashSet::new());
        let neighbors = if let Some(v) = graph.get(v) {
            v
        }
        else {
            &HashSet::new()
        };

        let new_p: HashSet<String> = p.intersection(neighbors).cloned().collect();
        let new_x: HashSet<String> = x.intersection(neighbors).cloned().collect();

        bron_kerbosch(&mut new_r, &mut new_p.clone(), &mut new_x.clone(), graph, cliques);

        p.remove(v);
        x.insert(v.clone());
    }
}

#[allow(dead_code)]
pub fn part2() -> String {
    let connections = aoc::to_lines("input/day23.txt");
    let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();
    let mut unique_nodes: HashSet<String> = HashSet::new();

    for connection in connections {
        let parts: Vec<&str> = connection.split('-').collect();

        if let [node1, node2] = parts.as_slice() {
            unique_nodes.insert(node1.to_string());
            register_node(node1, node2, &mut nodes);
            unique_nodes.insert(node2.to_string());
            register_node(node2, node1, &mut nodes);
        } else {
            panic!("Input does not match the expected format");
        }
    }

    let mut r = HashSet::new();
    let mut p: HashSet<String> = nodes.keys().cloned().collect();
    let mut x = HashSet::new();
    let mut cliques = Vec::new();

    // Find all maximal cliques
    bron_kerbosch(&mut r, &mut p, &mut x, &nodes, &mut cliques);

    // Print all cliques
    let mut max_clique: HashSet<String> = HashSet::new();
    for clique in cliques {
        if clique.len() > max_clique.len() {
            max_clique = clique.clone();
        }
    }

    let mut clique_vec = max_clique.iter().cloned().collect::<Vec<String>>();
    clique_vec.sort();
    clique_vec.join(",")
}