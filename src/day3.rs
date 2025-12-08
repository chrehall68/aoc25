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

fn part1(banks: &Vec<String>) {
    let total: i32 = banks
        .iter()
        .map(|bank| {
            // find max in everything up to last char
            let first_max = bank[..bank.len() - 1]
                .chars()
                .enumerate()
                // compare by number first, then smaller idx (gives max room in remaining)
                .max_by(|(i1, v1), (i2, v2)| (v1, -(*i1 as i32)).cmp(&(v2, -(*i2 as i32))))
                .unwrap();
            // then find max in remaining
            let second_max = bank[first_max.0 + 1..].chars().max().unwrap();
            let worth_str = format!("{}{}", first_max.1, second_max);
            let worth = worth_str.parse::<i32>().unwrap();
            worth
        })
        .sum();
    println!("{total}");
}
pub fn driver() {
    let banks = get_inp();
    part1(&banks);
}
