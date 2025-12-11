use std::{
    collections::{HashMap, HashSet},
    io::{self, stdin},
};

use itertools::Itertools;

fn get_inp(mut reader: impl io::BufRead) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let mut inp = String::new();
    let mut res = HashMap::new();
    let mut all_nodes: HashSet<String> = HashSet::new();
    while let Ok(bytes_read) = reader.read_line(&mut inp)
        && bytes_read > 0
    {
        let (a, b) = inp.split_once(":").unwrap();
        let key = a.trim();
        let connections: Vec<String> = b.trim().split(" ").map(|s| s.to_owned()).collect();
        res.insert(key.to_owned(), connections.clone());
        all_nodes.insert(key.to_owned());
        all_nodes.extend(connections);
        inp.clear();
    }
    (res, all_nodes.into_iter().collect())
}

fn dfs(
    s: &String,
    used: &mut HashSet<String>,
    exploring: &mut HashSet<String>,
    adj: &HashMap<String, Vec<String>>,
    result: &mut Vec<String>,
) {
    used.insert(s.clone());
    exploring.insert(s.clone());
    for conn in adj.get(s).unwrap_or(&vec![]).iter() {
        if !used.contains(conn) {
            // dfs that first
            dfs(conn, used, exploring, adj, result);
        }
        if exploring.contains(conn) {
            panic!("Found a cycle!");
        }
    }
    exploring.remove(s);
    result.push(s.clone());
}

fn topsort(adj: &HashMap<String, Vec<String>>, nodes: &Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    let mut used = HashSet::new();
    let mut exploring = HashSet::new();
    for s in nodes {
        if !used.contains(s) {
            dfs(s, &mut used, &mut exploring, adj, &mut res)
        }
    }
    res.reverse();
    res
}

fn count_ways(
    adj: &HashMap<String, Vec<String>>,
    order: &Vec<String>,
    source: &str,
    dest: &str,
) -> i64 {
    let mut ways: HashMap<String, i64> = HashMap::new();
    for node in order.iter() {
        ways.insert(node.clone(), 0);
    }
    ways.insert(source.to_owned(), 1);
    for node in order.iter() {
        // it can reach all its connections
        let my_ways = *ways.get(node).unwrap_or(&0);
        for conn in adj.get(node).unwrap_or(&vec![]).iter() {
            *ways.get_mut(conn).unwrap() += my_ways;
        }
    }
    *ways.get(dest).unwrap()
}
fn count_paths(
    adj: &HashMap<String, Vec<String>>,
    order: &Vec<String>,
    destinations: Vec<&str>,
) -> i64 {
    let mut total = 0;
    // TODO - make this more efficient since we only need to consider
    // them in top order
    for perm in destinations.iter().permutations(destinations.len()) {
        let mut ways = count_ways(adj, order, "svr", perm[0]);
        for i in 1..perm.len() {
            ways *= count_ways(adj, order, perm[i - 1], perm[i]);
        }
        ways *= count_ways(adj, order, perm[perm.len() - 1], "out");
        total += ways;
    }
    total
}

pub fn driver() {
    let (adjacency_list, nodes) = get_inp(stdin().lock());
    let top_order = topsort(&adjacency_list, &nodes);
    println!("{}", count_ways(&adjacency_list, &top_order, "you", "out"));
    println!(
        "{}",
        count_paths(&adjacency_list, &top_order, vec!["dac", "fft"])
    );
}
