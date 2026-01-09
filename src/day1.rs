use std::io;

// returns pairs of (dir (-1, 1), amt)
fn get_inp() -> Vec<(i32, i32)> {
    // read from stdin
    let mut vec = Vec::new();
    let mut input = String::new();

    while let Ok(read_amt) = io::stdin().read_line(&mut input)
        && read_amt != 0
    {
        let c_dir = input.chars().nth(0).expect("Not enough chars");
        let int_dir = if c_dir == 'L' { -1 } else { 1 };
        let amt: i32 = input[1..].trim().parse().expect("Failed to parse number");
        vec.push((int_dir, amt));
        input = String::new(); // realloc space
    }

    vec
}

fn part1(transforms: &Vec<(i32, i32)>) {
    let mut count = 0;
    let mut pos = 50;
    transforms.iter().for_each(|(dir, amt)| {
        pos = (pos + dir * amt % 100) % 100;
        if pos == 0 {
            count += 1;
        }
    });
    println!("{count}")
}

fn part2(transforms: &Vec<(i32, i32)>) {
    let mut count = 0;
    let mut pos = 50;
    transforms.iter().for_each(|(dir, amt)| {
        // if it's negative, use the negative representation
        let repr = if *dir == -1 { (100 - pos) % 100 } else { pos };
        let sum = repr+amt;
        let rem = sum % 100;
        count += sum / 100;
        // convert back to positive representation
        pos = if *dir == -1 { (100 - rem) % 100 } else { rem };
    });

    println!("{count}");
}

pub fn driver() {
    let transforms = get_inp();
    println!("Part 1");
    part1(&transforms);
    println!("Part 2");
    part2(&transforms);
}
