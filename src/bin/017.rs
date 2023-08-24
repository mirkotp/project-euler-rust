/// <p>If the numbers $1$ to $5$ are written out in words: one, two, three, four, five, then there are $3 + 3 + 5 + 4 + 4 = 19$ letters used in total.</p>
/// <p>If all the numbers from $1$ to $1000$ (one thousand) inclusive were written out in words, how many letters would be used? </p>
/// <br><p class="note"><b>NOTE:</b> Do not count spaces or hyphens. For example, $342$ (three hundred and forty-two) contains $23$ letters and $115$ (one hundred and fifteen) contains $20$ letters. The use of "and" when writing out numbers is in compliance with British usage.</p>

fn main() {
    println!("{}", (1..=1000).map(|n| count(n)).sum::<u32>());
}

fn count(n: u32) -> u32 {
    match n {
        1 | 2 | 6 | 10 => 3,
        4 | 5 | 9 => 4,
        3 | 7 | 8 => 5,
        11 | 12 => 6,
        15 | 16 => 7,
        13 | 14 | 18 | 19 => 8,
        17 => 9,
        40 | 50 | 60 => 5,
        41..=69 => 5 + count(n % 10),
        20 | 30 | 80 | 90 => 6 + n % 10,
        21..=39 | 81..=99 => 6 + count(n % 10),
        70 => 7,
        71..=79 => 7 + count(n % 10),
        100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900 => 7 + count(n / 100),
        101..=999 => 10 + count(n / 100) + count(n % 100),
        1000 => 11,
        _ => panic!("Unexpected number: {}", n),
    }
}
