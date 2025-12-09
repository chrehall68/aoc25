use std::io;

#[derive(Debug, Clone, Copy)]
struct Rect {
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
}
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
fn rectangle_intersects(rect: Rect, p1: (i64, i64), p2: (i64, i64)) -> bool {
    let Rect {
        left,
        right,
        top,
        bottom,
    } = rect;
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    if y1 != y2 {
        // vertical line
        // so check if it crosses the rectangle's top or bottom
        let (b, t) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        return ((b <= bottom && t > bottom) || (b < top && t >= top)) && x1 > left && x1 < right;
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
fn validator(rect: Rect, points: &Vec<(i64, i64)>) -> bool {
    // check that the center of the rectangle is in the polygon
    // and check that no polygon lines intersect the rectangle
    // technically this assumes that lines aren't optimal
    // but that works since there's always at least one green between
    let center = ((rect.left + rect.right) / 2, (rect.bottom + rect.top) / 2);
    let intersects = (0..points.len())
        .any(|i| rectangle_intersects(rect, points[i], points[(i + 1) % points.len()]));
    polygon_contains(points, center) && !intersects
}
fn max_valid_pair(inp: &Vec<(i64, i64)>, validator: impl Fn(Rect) -> bool) -> i64 {
    let mut best = 0;
    for i in 0..inp.len() {
        for j in i + 1..inp.len() {
            if i != j {
                let r = Rect {
                    top: inp[i].1.max(inp[j].1),
                    bottom: inp[i].1.min(inp[j].1),
                    left: inp[i].0.min(inp[j].0),
                    right: inp[i].0.max(inp[j].0),
                };
                if validator(r) {
                    best = best.max((r.right - r.left + 1) * (r.top - r.bottom + 1));
                }
            }
        }
    }

    best
}
pub fn driver() {
    let inp = get_inp(io::stdin().lock());
    println!("{}", max_valid_pair(&inp, |_| true));
    println!("{}", max_valid_pair(&inp, |r| validator(r, &inp)));
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
        assert_eq!(max_valid_pair(&inp, |_| true), 50);
    }
    #[test]
    fn test_part2() {
        let inp = get_inp(INP.as_bytes());
        assert_eq!(max_valid_pair(&inp, |r| validator(r, &inp)), 24);
    }
}
