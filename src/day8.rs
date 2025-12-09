use std::{
    collections::BinaryHeap,
    io::{self, stdin},
};

fn get_inp(mut reader: impl io::BufRead) -> Vec<(i64, i64, i64)> {
    let mut inp = String::new();
    let mut res = Vec::new();
    while let Ok(bytes_read) = reader.read_line(&mut inp)
        && bytes_read > 0
    {
        let split: Vec<i64> = inp.trim().split(",").map(|s| s.parse().unwrap()).collect();
        res.push((split[0], split[1], split[2]));
        inp.clear();
    }
    res
}
fn find_rep(i: usize, reps: &mut Vec<usize>) -> usize {
    if reps[i] != i {
        reps[i] = find_rep(reps[i], reps);
    }
    return reps[i];
}

fn union(i: usize, j: usize, reps: &mut Vec<usize>, sizes: &mut Vec<i64>) -> bool {
    let rep_i = find_rep(i, reps);
    let rep_j = find_rep(j, reps);
    if rep_i != rep_j {
        let size_i = sizes[rep_i];
        let size_j = sizes[rep_j];
        if size_i > size_j {
            // then i is new rep
            reps[rep_j] = rep_i;
            sizes[rep_i] += size_j;
        } else {
            // j is new rep
            reps[rep_i] = rep_j;
            sizes[rep_j] += size_i;
        }
        return true;
    }
    false
}

fn mst_sizes(points: Vec<(i64, i64, i64)>, steps: usize) -> (i64, i64) {
    let mut edges = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dx = points[i].0 - points[j].0;
            let dy = points[i].1 - points[j].1;
            let dz = points[i].2 - points[j].2;
            let d2 = dx * dx + dy * dy + dz * dz;
            edges.push((d2, i, j));
        }
    }
    edges.sort(); // sort by min dist

    let mut components = points.len();
    let mut reps = (0..points.len()).collect();
    let mut sizes = vec![1; points.len()];
    for i in 0..steps {
        components -= union(edges[i].1, edges[i].2, &mut reps, &mut sizes) as usize;
    }
    // then just get the sizes of the reps
    let mut used_reps = vec![false; points.len()];
    let mut largest_sizes = BinaryHeap::new();
    for i in 0..points.len() {
        let rep = find_rep(i, &mut reps);
        if !used_reps[rep] {
            largest_sizes.push(-sizes[rep]);
            used_reps[rep] = true;
            if largest_sizes.len() > 3 {
                largest_sizes.pop();
            }
        }
    }
    // finish mst
    let mut last_idx = steps - 1;
    while components > 1 {
        last_idx += 1;
        components -= union(edges[last_idx].1, edges[last_idx].2, &mut reps, &mut sizes) as usize;
    }
    // take product of x of last unioned things
    let px = points[edges[last_idx].1].0 * points[edges[last_idx].2].0;
    (-largest_sizes.iter().product::<i64>(), px)
}

pub fn driver() {
    let points = get_inp(stdin().lock());
    let (p1, p2) = mst_sizes(points, 1000);
    println!("{}", p1);
    println!("{}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INP: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689"};

    #[test]
    fn test_examples() {
        let points = get_inp(io::Cursor::new(INP));
        let (p1, p2) = mst_sizes(points, 10);
        assert_eq!(p1, 40);
        assert_eq!(p2, 25272);
    }
}
