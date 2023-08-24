/// <p>$2^{15} = 32768$ and the sum of its digits is $3 + 2 + 7 + 6 + 8 = 26$.</p>
/// <p>What is the sum of the digits of the number $2^{1000}$?</p>

fn main() {
    const P: usize = 1000;
    let size = 1 + (P as f64 * 2_f64.log10()).floor() as usize;
    let mut digits = vec![0_usize; size];

    digits[0] = 2;
    for _ in 2..=P {
        let mut carry = 0;
        for i in 0..size {
            let double = 2 * digits[i] + carry;
            digits[i] = double % 10;
            carry = if double >= 10 { 1 } else { 0 };
        }
    }

    dbg!(digits.iter().sum::<usize>());
}
