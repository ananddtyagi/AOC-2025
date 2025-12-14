use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");

    let banks = contents.split("\n");
    let mut nums = Vec::<i32>::new();

    for bank in banks {
        let mut max = 0;
        let batteries: Vec<i32> = bank
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect();

        for i in 1..batteries.len() {
            if batteries[i] > batteries[max] {
                max = i;
            }
        }

        let mut second_max = 0;
        if max == batteries.len() - 1 {
            for i in 1..max {
                if batteries[i] > batteries[second_max] {
                    second_max = i;
                }
            }
            let val = batteries[second_max] * 10 + batteries[max];
            nums.push(val);
        } else {
            second_max = max + 1;
            for i in max + 1..batteries.len() {
                if batteries[i] > batteries[second_max] {
                    second_max = i
                }
            }
            let val = batteries[max] * 10 + batteries[second_max];
            nums.push(val);
        }
    }
    println!("{}", nums.iter().sum::<i32>())
}
