use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error");
    let mut dial_pos = 50;

    let turns = contents.split("\n");
    let mut count = 0;
    for turn in turns {
        let dir = &turn[0..1];
        let amount = turn[1..].parse::<i32>().unwrap();
        match dir {
            "R" => dial_pos = (dial_pos + amount) % 100,
            "L" => dial_pos = (dial_pos - amount) % 100,
            _ => println!("Error"),
        }

        if dial_pos < 0 {
            dial_pos = 100 + dial_pos;
        }

        if dial_pos == 0 {
            count += 1
        }
    }
    println!("{count}");
}
