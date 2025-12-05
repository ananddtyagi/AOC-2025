use std::fs;

fn isError(num: i64) -> bool {
    let s = num.to_string();
    for i in 0..(s.len() / 2) {
        if s.len() % (i + 1) != 0 {
            continue;
        }
        if s[0..=i].repeat(s.len() / (i + 1)) == s {
            return true;
        }
    }
    return false;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error");
    let ranges: Vec<&str> = contents.split(",").collect();
    let mut duplicates: Vec<i64> = Vec::new();

    for range in ranges {
        let (s, e) = range.split_once("-").unwrap();
        let start: i64 = s.parse().unwrap();
        let end: i64 = e.parse().unwrap();

        for i in start..=end {
            if isError(i) {
                duplicates.push(i);
            }
        }
    }
    println!("{:?}", duplicates);
    println!("{}", duplicates.iter().sum::<i64>());
}
