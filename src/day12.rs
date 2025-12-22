use std::io::{BufRead, stdin};

type Shape = [[bool; 3]; 3];
fn get_inp(mut reader: impl BufRead) -> (Vec<Shape>, Vec<(i32, i32, Vec<i32>)>) {
    let mut inp = String::new();
    let mut shapes = Vec::new();
    let mut cases = Vec::new();
    while let Ok(bytes_read) = reader.read_line(&mut inp)
        && bytes_read > 0
    {
        if inp.trim().ends_with(":") {
            inp.clear();
            let mut a = [[false; 3]; 3];
            for i in 0..3 {
                reader.read_line(&mut inp).expect("Failed to read");
                for j in 0..3 {
                    a[i][j] = inp.as_bytes()[j] == b'#';
                }
                inp.clear();
            }
            // clear the blank line
            reader
                .read_line(&mut inp)
                .expect("Failed to clear blank line");
            inp.clear();
            shapes.push(a);
        } else {
            // it's a test case
            let (dims, amounts) = inp.split_once(":").unwrap();
            let (width, height) = dims.split_once("x").unwrap();
            let numerical_amounts: Vec<i32> = amounts
                .trim()
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            cases.push((
                width.parse().unwrap(),
                height.parse().unwrap(),
                numerical_amounts,
            ));
            inp.clear();
        }
    }
    (shapes, cases)
}

fn part1(_shapes: &Vec<Shape>, cases: &Vec<(i32, i32, Vec<i32>)>) -> i64 {
    cases
        .iter()
        .map(|(width, height, amounts)| {
            let available_three = width / 3 * height / 3;
            let used = amounts.iter().sum();
            (available_three >= used) as i64
        })
        .sum()
}
pub fn driver() {
    let inp = get_inp(stdin().lock());
    println!("Part 1: {}", part1(&inp.0, &inp.1));
}
