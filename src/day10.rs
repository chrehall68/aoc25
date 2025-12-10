use good_lp::*;
use std::{
    collections::HashMap,
    io::{self, stdin},
};

fn get_inp(mut reader: impl io::BufRead) -> Vec<(i32, Vec<i32>, Vec<i32>)> {
    // each describes a test case
    // desired number, possible switches, joltages
    let mut inp = String::new();
    let mut res = Vec::new();
    while let Ok(bytes_read) = reader.read_line(&mut inp)
        && bytes_read > 0
    {
        let parts: Vec<&str> = inp.split_whitespace().collect();
        let mut desired = 0;
        for (i, &c) in parts[0].as_bytes().iter().skip(1).enumerate() {
            if c != b']' {
                if c == b'#' {
                    desired |= 1 << i;
                }
            }
        }

        let mut buttons = Vec::new();
        for i in 1..parts.len() - 1 {
            let spaces = parts[i].trim_matches('(').trim_matches(')');
            let mut button = 0;
            for space in spaces.split(',') {
                button |= 1 << space.parse::<usize>().unwrap();
            }
            buttons.push(button);
        }

        let junctions = parts[parts.len() - 1]
            .trim_matches('{')
            .trim_matches('}')
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        res.push((desired, buttons, junctions));

        inp.clear();
    }
    res
}

fn min_presses(desired: i32, options: &Vec<i32>) -> i32 {
    let mut bests: HashMap<i32, i32> = HashMap::new();
    bests.insert(0, 0);
    let mut cur = vec![0];
    for &option in options.iter() {
        let mut new_row = Vec::new();
        for &possib in cur.iter() {
            let key = possib ^ option;
            let existing = bests.get(&key);
            let possib_val = *bests.get(&possib).unwrap() + 1;
            if existing == None {
                // totally new option
                new_row.push(key);
                bests.insert(key, possib_val);
            } else if *existing.unwrap() > possib_val {
                // new best for that
                bests.insert(key, possib_val);
            }
        }
        cur.append(&mut new_row);
    }
    *bests.get(&desired).unwrap()
}
fn part1(cases: &Vec<(i32, Vec<i32>, Vec<i32>)>) -> i32 {
    cases
        .iter()
        .map(|(desired, buttons, _)| min_presses(*desired, buttons))
        .sum()
}
fn solve_lp(buttons: &Vec<i32>, junctions: &Vec<i32>) -> i32 {
    let mut vars = variables!();
    let var_vec: Vec<_> = (0..buttons.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();
    let constraints: Vec<Constraint> = junctions
        .iter()
        .enumerate()
        .map(|(i, &desired)| {
            let mut lhs: Expression = 0.into();
            for (button_idx, &button) in buttons.iter().enumerate() {
                if button & (1 << i) != 0 {
                    lhs += var_vec[button_idx]
                }
            }
            lhs.eq(desired)
        })
        .collect();
    let objective: Expression = var_vec.iter().sum();
    let solution = vars
        .minimise(&objective)
        .using(default_solver)
        .with_all(constraints)
        .solve()
        .unwrap();
    solution.eval(&objective).round() as i32
}
fn part2(cases: &Vec<(i32, Vec<i32>, Vec<i32>)>) -> i32 {
    cases
        .iter()
        .map(|(_, buttons, junctions)| solve_lp(buttons, junctions))
        .sum()
}

pub fn driver() {
    let inp = get_inp(stdin().lock());
    println!("{}", part1(&inp));
    println!("{}", part2(&inp));
}
