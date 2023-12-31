/// <p>By listing the first six prime numbers: $2, 3, 5, 7, 11$, and $13$, we can see that the $6$th prime is $13$.</p>
/// <p>What is the $10\,001$st prime number?</p>

fn main() {
    let mut count = 1;
    for n in (3..).step_by(2) {
        if is_prime(n) {
            count += 1;

            if count == 10001 {
                dbg!(n);
                return;
            }
        }
    }
}

fn is_prime(n: u64) -> bool {
    for d in 2..=((n as f64).sqrt().floor() as u64) {
        if n % d == 0 {
            return false;
        }
    }
    true
}
