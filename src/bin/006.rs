/// <p>The sum of the squares of the first ten natural numbers is,</p>
/// $$1^2 + 2^2 + ... + 10^2 = 385.$$
/// <p>The square of the sum of the first ten natural numbers is,</p>
/// $$(1 + 2 + ... + 10)^2 = 55^2 = 3025.$$
/// <p>Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.</p>
/// <p>Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.</p>

fn main() {
    const N: u128 = 100;
    let squared_sum = ((N * (N + 1)) / 2).pow(2);
    let sum_of_squares =
        ((2f64 * N as f64 + 1f64) * (N as f64 + 1f64) * (N as f64 / 6f64)).round() as u128;

    dbg!(squared_sum - sum_of_squares);
}
