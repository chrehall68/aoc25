use std::io;

fn get_inp(mut reader: impl io::BufRead) -> Vec<(i64, i64)> {
    let mut inp = String::new();
    let mut res = Vec::new();
    while let Ok(bytes_read) = reader.read_line(&mut inp)
        && bytes_read > 0
    {
        let (p1, p2) = inp.trim().split_once(",").unwrap();
        res.push((p1.parse().unwrap(), p2.parse().unwrap()));
        inp.clear();
    }
    res
}
fn part1(inp: &Vec<(i64, i64)>) -> i64 {
    let mut best = 0;
    let mut smallest = i64::MAX;
    for i in 0..inp.len() {
        for j in i + 1..inp.len() {
            let dx = (inp[i].0 - inp[j].0).abs() + 1;
            let dy = (inp[i].1 - inp[j].1).abs() + 1;
            best = best.max(dx * dy);
            smallest = smallest.min(dx * dy);
        }
    }
    assert!(smallest > 2);
    best
}
fn rectangle_intersects(rect: (i64, i64, i64, i64), p1: (i64, i64), p2: (i64, i64)) -> bool {
    let (left, bottom, right, top) = rect;
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    if y1 != y2 {
        // vertical line
        // so check if it crosses the rectangle's top or bottom
        let (lower, upper) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        return ((lower <= bottom && upper > bottom) || (lower < top && upper >= top))
            && x1 > left
            && x1 < right;
    } else {
        // horizontal line
        // check if crosses the rectangle's left or right
        let (l, r) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        return ((l <= left && r > left) || (l < right && r >= right)) && y1 > bottom && y1 < top;
    }
}
fn polygon_contains(polygon: &Vec<(i64, i64)>, point: (i64, i64)) -> bool {
    let mut crosses = 0;
    for i in 0..polygon.len() {
        let (x1, y1) = polygon[i];
        let (_, y2) = polygon[(i + 1) % polygon.len()];
        // shoot a horizontal ray to the right of the point
        if y1 != y2 && x1 > point.0 && y1 < point.1 && point.1 < y2 {
            crosses += 1;
        }
    }
    crosses % 2 == 1
}
fn part2(inp: &Vec<(i64, i64)>) -> i64 {
    // an item can act as either top left or bottom left
    // only 500 items, so O(N**3) works
    // also, there is at least one green between items
    // so this works out
    let mut best = 0;
    for i in 0..inp.len() {
        for j in 0..inp.len() {
            // technically this assumes that lines aren't optimal
            // but that works since there's always at least one green between
            if i != j && inp[i].1 < inp[j].1 {
                // try pairing them together
                // they form a valid rectangle iff:
                // - no polygon lines intersect the rectangle
                // - the center of the rectangle is in the polygon
                let top = inp[j].1;
                let bottom = inp[i].1;
                let left = inp[i].0.min(inp[j].0);
                let right = inp[i].0.max(inp[j].0);
                // first, check that no corners are strictly inside the rectangle
                let safe = (0..inp.len()).all(|idx| {
                    !rectangle_intersects(
                        (left, bottom, right, top),
                        inp[idx],
                        inp[(idx + 1) % inp.len()],
                    )
                });
                // then check the center
                let center = ((left + right) / 2, (top + bottom) / 2);
                if safe && polygon_contains(inp, center) {
                    let value = (right - left + 1) * (top - bottom + 1);
                    best = best.max(value);
                }
            }
        }
    }

    best
}
pub fn driver() {
    let inp = get_inp(io::stdin().lock());
    println!("{}", part1(&inp));
    println!("{}", part2(&inp));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INP: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3"
    };

    #[test]
    fn test_part1() {
        let inp = get_inp(INP.as_bytes());
        assert_eq!(part1(&inp), 50);
    }
    #[test]
    fn test_part2() {
        let inp = get_inp(INP.as_bytes());
        assert_eq!(part2(&inp), 24);
    }
}
