use std::{io, mem::swap};

fn get_inp(mut reader: impl io::BufRead) -> Vec<String> {
    let mut inp = String::new();
    let mut res = Vec::new();
    while let Ok(bytes_read) = reader.read_line(&mut inp)
        && bytes_read > 0
    {
        res.push(inp.trim().to_owned());
        inp.clear();
    }
    res
}
fn part1(inp: Vec<String>) -> i32 {
    let mut total = 0;
    let mut prev_row = Vec::new();
    for c in inp[0].as_bytes() {
        prev_row.push(*c == b'S');
    }
    let mut cur_row = vec![false; inp[0].len()];

    for line in inp.iter().skip(1) {
        cur_row.fill(false);
        for (i, c) in line.as_bytes().iter().enumerate() {
            if prev_row[i] {
                if *c == b'^' {
                    if i > 0 {
                        cur_row[i - 1] = true;
                    }
                    if i + 1 < line.len() {
                        cur_row[i + 1] = true;
                    }
                    total += 1;
                } else {
                    cur_row[i] = true;
                }
            }
        }
        swap(&mut cur_row, &mut prev_row);
    }
    total
}
pub fn driver() {
    let inp = get_inp(io::stdin().lock());
    println!("{}", part1(inp));
}
