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
fn split(inp: Vec<String>) -> (i32, i64) {
    let mut splits = 0;
    let mut prev_row = Vec::new();
    for c in inp[0].as_bytes() {
        prev_row.push((*c == b'S') as i64);
    }
    let mut cur_row = vec![0; inp[0].len()];

    for line in inp.iter().skip(1) {
        cur_row.fill(0);
        for (i, c) in line.as_bytes().iter().enumerate() {
            if prev_row[i] != 0 {
                if *c == b'^' {
                    if i > 0 {
                        cur_row[i - 1] += prev_row[i];
                    }
                    if i + 1 < line.len() {
                        cur_row[i + 1] = prev_row[i];
                    }
                    splits += 1;
                } else {
                    cur_row[i] += prev_row[i];
                }
            }
        }
        swap(&mut cur_row, &mut prev_row);
    }
    (splits, prev_row.iter().sum())
}
pub fn driver() {
    let inp = get_inp(io::stdin().lock());
    let (p1, p2) = split(inp);
    println!("{}", p1);
    println!("{}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INP: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ..............."
    };

    #[test]
    fn test_parts() {
        let inp = get_inp(io::Cursor::new(INP));
        let (p1, p2) = split(inp);
        assert_eq!(p1, 21);
        assert_eq!(p2, 40);
    }
}
