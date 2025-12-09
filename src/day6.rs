use std::io::{self, stdin};

fn get_inp<R>(mut reader: R) -> (Vec<Vec<i64>>, Vec<Vec<i64>>, Vec<u8>)
where
    R: io::BufRead,
{
    // read in all lines
    let mut lines = Vec::new();
    let mut inp = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut inp)
        && bytes_read > 0
    {
        let to_push = inp.strip_suffix("\n").unwrap_or(&inp).to_owned();
        lines.push(to_push);
        inp = String::new();
    }

    // process everything besides last line
    let mut row_nums = Vec::new();
    row_nums.resize(lines[0].split_whitespace().count(), Vec::new());
    for row in 0..lines.len() - 1 {
        lines[row]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .enumerate()
            .for_each(|(col, num)| row_nums[col].push(num));
    }
    let mut col_nums = vec![Vec::new()];
    for col in 0..lines[0].len() {
        if (0..lines.len() - 1).all(|row| lines[row].as_bytes()[col] == b' ') {
            col_nums.push(Vec::new()); // start of new section
        } else {
            let s: String = (0..lines.len() - 1)
                .map(|row| char::from(lines[row].as_bytes()[col]))
                .collect();
            let idx = col_nums.len() - 1;
            col_nums[idx].push(s.trim().parse().unwrap());
        }
    }
    let ops = lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| s.as_bytes()[0])
        .collect();
    (row_nums, col_nums, ops)
}
fn op_reduce(nums: &Vec<Vec<i64>>, ops: &Vec<u8>) -> i64 {
    nums.iter().enumerate().fold(0, |cur, (col, vec)| {
        let f = match ops[col] {
            b'*' => |a: i64, b: i64| a * b,
            b'+' => |a: i64, b: i64| a + b,
            _ => panic!("Invalid operator"),
        };
        cur + vec.clone().into_iter().reduce(f).unwrap()
    })
}
pub fn driver() {
    let (row_nums, col_nums, ops) = get_inp(stdin().lock());
    println!("{}", op_reduce(&row_nums, &ops));
    println!("{}", op_reduce(&col_nums, &ops));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const STR: &str = indoc! {"
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +  "};

    #[test]
    fn test_get_inp() {
        let reader = io::Cursor::new(STR);
        let (row_nums, col_nums, ops) = get_inp(reader);
        let expected_row_nums = vec![
            vec![123, 45, 6],
            vec![328, 64, 98],
            vec![51, 387, 215],
            vec![64, 23, 314],
        ];
        let expected_col_nums = vec![
            vec![1, 24, 356],
            vec![369, 248, 8],
            vec![32, 581, 175],
            vec![623, 431, 4],
        ];
        let expected_ops = vec![b'*', b'+', b'*', b'+'];
        assert_eq!(row_nums, expected_row_nums);
        assert_eq!(col_nums, expected_col_nums);
        assert_eq!(ops, expected_ops);
    }

    #[test]
    fn test_part1() {
        let reader = io::Cursor::new(STR);
        let (row_nums, _, ops) = get_inp(reader);
        assert_eq!(op_reduce(&row_nums, &ops), 4277556);
    }
    #[test]
    fn test_part2() {
        let reader = io::Cursor::new(STR);
        let (_, col_nums, ops) = get_inp(reader);
        assert_eq!(op_reduce(&col_nums, &ops), 3263827);
    }
}
