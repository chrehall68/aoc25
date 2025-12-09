use std::io::stdin;

fn get_inp() -> Vec<String> {
    let mut inp = String::new();
    let mut res = Vec::new();
    while let Ok(bytes_read) = stdin().read_line(&mut inp)
        && bytes_read != 0
    {
        res.push(inp.trim().to_owned());
        inp = String::new();
    }

    res
}

// returns the sum of the largest k-subsequence in each bank
fn sum_max(banks: &Vec<String>, k: usize) -> i64 {
    banks
        .iter()
        .map(|bank| {
            let mut stack = Vec::new();
            bank.char_indices().for_each(|(i, c)| {
                let remaining = bank.len() - i;
                while !stack.is_empty() && remaining + stack.len() > k && stack[stack.len() - 1] < c
                {
                    stack.pop();
                }
                stack.push(c);
            });
            let worth_str: String = stack.iter().take(k).collect();
            let worth: i64 = worth_str.parse().unwrap();
            worth
        })
        .sum()
}

pub fn driver() {
    let banks = get_inp();
    // part 1
    println!("{}", sum_max(&banks, 2));
    // part 2
    println!("{}", sum_max(&banks, 12));
}

#[cfg(test)]
mod tests {
    use super::*;

    const JOLTAGES: [&str; 4] = [
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    #[test]
    fn test_part1_example() {
        let banks = Vec::from(JOLTAGES)
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        assert_eq!(sum_max(&banks, 2), 357);
    }

    #[test]
    fn test_part2_example() {
        let banks = Vec::from(JOLTAGES)
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        assert_eq!(sum_max(&banks, 12), 3121910778619);
    }
}
