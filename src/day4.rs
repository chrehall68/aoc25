use std::io::stdin;

fn get_inp() -> Vec<String> {
    let mut inp = String::new();
    let mut res = Vec::new();
    while let Ok(bytes_read) = stdin().read_line(&mut inp)
        && bytes_read > 0
    {
        res.push(inp.trim().to_owned());
        inp = String::new();
    }
    res
}

fn part1(rows: &Vec<String>) -> i64 {
    let mut total = 0;
    let cols = rows[0].len();
    for r in 0..rows.len() {
        for c in 0..cols {
            // check all neighbors
            if rows[r].as_bytes()[c] == b'@' {
                let mut neighbors = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr != 0 || dc != 0 {
                            // not self
                            let rp = r as i32 + dr;
                            let cp = c as i32 + dc;
                            if 0 <= rp
                                && rp < rows.len() as i32
                                && 0 <= cp
                                && cp < cols as i32
                                && rows[rp as usize].as_bytes()[cp as usize] == b'@'
                            {
                                neighbors += 1;
                            }
                        }
                    }
                }
                if neighbors < 4 {
                    total += 1;
                }
            }
        }
    }

    total
}
pub fn driver() {
    let rows = get_inp();
    println!("{}", part1(&rows));
}
