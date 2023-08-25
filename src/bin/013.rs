/// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./res/013").unwrap();
    let reader = BufReader::new(file);

    let mut sum = vec![0; 50];
    for line in reader.lines() {
        let s = line.unwrap();

        for (i, c) in s.chars().rev().enumerate() {
            let res = sum[i] + (c as u8) - 48;
            sum[i] = res % 10;
            let mut carry = res / 10;
            let mut current = i;

            while carry != 0 {
                current += 1;

                if current == sum.len() {
                    sum.push(carry);
                    break;
                }

                let res = sum[current] + carry;
                sum[current] = res % 10;
                carry = res / 10;
            }
        }
    }
    dbg!(sum.len());
    let result = sum
        .iter()
        .rev()
        .take(10)
        .map(|n| *n + 48)
        .collect::<Vec<_>>();

    dbg!(String::from_utf8_lossy(&result));
}
