use std::io::stdin;

fn get_inp() -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut inp = String::new();
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    let mut processing_ranges = true;
    while let Ok(bytes_read) = stdin().read_line(&mut inp)
        && bytes_read != 0
    {
        if bytes_read == 1 {
            processing_ranges = false;
            continue;
        }
        if processing_ranges {
            let (start, end) = inp.trim().split_once("-").unwrap();
            ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        } else {
            ids.push(inp.trim().parse().unwrap());
        }
        inp = String::new();
    }

    (ranges, ids)
}
fn merge(ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut sorted: Vec<(i64, i64)> = ranges.clone().into_iter().collect();
    sorted.sort();
    let mut unioned: Vec<(i64, i64)> = Vec::new();
    sorted.iter().for_each(|k| {
        if !unioned.is_empty() && unioned[unioned.len() - 1].1 >= k.0 {
            let back_idx = unioned.len() - 1;
            unioned[back_idx].1 = i64::max(unioned[unioned.len() - 1].1, k.1);
        } else {
            unioned.push(k.clone());
        }
    });
    unioned
}
fn part1(ranges: &Vec<(i64, i64)>, ids: &Vec<i64>) -> i64 {
    ids.iter()
        .map(|id| {
            if let Err(i) = ranges.binary_search(&(*id, i64::MAX))
                && i > 0
                && ranges[i - 1].0 <= *id
                && ranges[i - 1].1 >= *id
            {
                1
            } else {
                0
            }
        })
        .sum()
}
fn part2(ranges: &Vec<(i64, i64)>) -> i64 {
    ranges.iter().map(|(start, end)| *end - *start + 1).sum()
}
pub fn driver() {
    let (ranges, idx) = get_inp();
    let merged = merge(&ranges);
    println!("{}", part1(&merged, &idx));
    println!("{}", part2(&merged));
}

#[cfg(test)]
mod tests {
    use super::*;

    const RANGES: [(i64, i64); 4] = [(3, 5), (10, 14), (16, 20), (12, 18)];
    const IDS: [i64; 6] = [1, 5, 8, 11, 17, 32];

    #[test]
    fn test_merge() {
        let res = merge(&Vec::from(RANGES));
        assert_eq!(res, Vec::from([(3, 5), (10, 20)]));
    }

    #[test]
    fn test_part1() {
        let merged = merge(&Vec::from(RANGES));
        let res = part1(&merged, &Vec::from(IDS));
        assert_eq!(res, 3);
    }

    #[test]
    fn test_part2() {
        let merged = merge(&Vec::from(RANGES));
        let res = part2(&merged);
        assert_eq!(res, 14);
    }
}
