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
            let mut start = 0;
            let mut worth_str = String::new();
            for i in 0..k {
                // find max in remaining, leaving space for next chars
                let next_chars: usize = k - i - 1;
                let max = bank[start..bank.len() - next_chars]
                    .chars()
                    .enumerate()
                    // compare by number first, then smaller idx (gives max room in remaining)
                    .max_by(|(i1, v1), (i2, v2)| (v1, -(*i1 as i32)).cmp(&(v2, -(*i2 as i32))))
                    .unwrap();
                start += max.0 + 1;
                worth_str.push(max.1);
            }
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
