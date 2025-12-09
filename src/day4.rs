use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

fn get_inp() -> Vec<String> {
    let mut inp = String::new();
    let mut res = Vec::new();
    while let Ok(bytes_read) = stdin().read_line(&mut inp)
        && bytes_read > 0
    {
        res.push(inp.trim().to_owned());
        inp = String::new();
    }
    res
}

fn indegrees(rows: &Vec<String>) -> HashMap<(i32, i32), i32> {
    let cols = rows[0].len();
    let mut res = HashMap::new();
    for r in 0..rows.len() {
        for c in 0..cols {
            // check all neighbors
            if rows[r].as_bytes()[c] == b'@' {
                let mut neighbors = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr != 0 || dc != 0 {
                            // not self
                            let rp = r as i32 + dr;
                            let cp = c as i32 + dc;
                            if 0 <= rp
                                && rp < rows.len() as i32
                                && 0 <= cp
                                && cp < cols as i32
                                && rows[rp as usize].as_bytes()[cp as usize] == b'@'
                            {
                                neighbors += 1;
                            }
                        }
                    }
                }
                res.insert((r as i32, c as i32), neighbors);
            }
        }
    }

    res
}

fn reachable(rows: &Vec<String>, mut indegrees: HashMap<(i32, i32), i32>) -> usize {
    // basically topsort
    // but instead of allowing once indegree == 0,
    // we now allow once indegree < 4
    let mut to_process: Vec<(i32, i32)> = indegrees
        .iter()
        .filter_map(|(k, v)| if *v < 4 { Some(*k) } else { None })
        .collect();
    let mut used: HashSet<(i32, i32), _> = to_process.iter().cloned().collect::<HashSet<_>>();
    let mut idx = 0;
    while idx < to_process.len() {
        let (r, c) = to_process[idx];
        // decrement all neighbors
        for dr in -1..=1 {
            for dc in -1..=1 {
                let rp = r + dr;
                let cp = c + dc;
                let key = (rp, cp);
                if indegrees.contains_key(&key) {
                    let deg = indegrees.get(&key).unwrap() - 1;
                    indegrees.insert(key, deg);
                    if deg == 3 {
                        used.insert(key);
                        to_process.push(key);
                    }
                }
            }
        }
        idx += 1;
    }

    to_process.len()
}

pub fn driver() {
    let rows = get_inp();
    let starter = indegrees(&rows);
    println!("{}", starter.values().filter(|v| **v < 4).count());
    println!("{}", reachable(&rows, starter));
}

#[cfg(test)]
mod tests {
    use super::*;
    const ROWS: [&str; 10] = [
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    #[test]
    fn test_part1_example() {
        let v = Vec::from(ROWS).into_iter().map(|s| s.to_owned()).collect();
        let starter = indegrees(&v);
        let res = starter.values().filter(|v| **v < 4).count();
        assert_eq!(res, 13);
    }

    #[test]
    fn test_part2_example() {
        let v = Vec::from(ROWS).into_iter().map(|s| s.to_owned()).collect();
        let starter = indegrees(&v);
        let res = reachable(&v, starter);
        assert_eq!(res, 43);
    }
}
