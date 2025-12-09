use std::io::stdin;

fn get_inp() -> Vec<(i64, i64)> {
    let mut input = String::new();
    let mut res = Vec::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().split(",").for_each(|section| {
        let parts: Vec<_> = section
            .split("-")
            .map(|range| range.parse::<i64>().expect("Failed to parse number"))
            .collect();
        res.push((parts[0], parts[1]));
    });
    res
}

// returns first rep that works, else 1
fn count_reps(num: i64) -> i32 {
    let s = num.to_string();
    for rep in 2..=s.len() {
        // check if something appears rep times
        // it does if it divides the size and each section matches
        if s.len() % rep == 0 {
            // check each section
            let chunk = s.len() / rep;
            let mut start = chunk;
            let mut good = true;
            while start < s.len() {
                good &= s[start..start + chunk] == s[(start - chunk)..start];
                start += chunk;
            }
            if good {
                return rep as i32;
            }
        }
    }
    1
}

fn sum_if<F>(ranges: &Vec<(i64, i64)>, condition: F) -> i64
where
    F: Fn(i32) -> bool,
{
    let mut count = 0;
    ranges.clone().into_iter().for_each(|(start, end)| {
        for num in start..=end {
            if condition(count_reps(num)) {
                count += num;
            }
        }
    });
    count
}

pub fn driver() {
    let ranges = get_inp();
    // part 1
    println!("{}", sum_if(&ranges, |count| count == 2));
    // part 2
    println!("{}", sum_if(&ranges, |count| count >= 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const RANGES: [(i64, i64); 11] = [
        (11, 22),
        (95, 115),
        (998, 1012),
        (1188511880, 1188511890),
        (222220, 222224),
        (1698522, 1698528),
        (446443, 446449),
        (38593856, 38593862),
        (565653, 565659),
        (824824821, 824824827),
        (2121212118, 2121212124),
    ];

    #[test]
    fn test_count_reps() {
        assert_eq!(count_reps(123), 1);
        assert_eq!(count_reps(1212), 2);
        assert_eq!(count_reps(1221), 1);
        assert_eq!(count_reps(213213213), 3);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(sum_if(&Vec::from(&RANGES), |count| count == 2), 1227775554);
    }
    #[test]
    fn test_part2_example() {
        assert_eq!(sum_if(&Vec::from(&RANGES), |count| count >= 2), 4174379265)
    }
}
