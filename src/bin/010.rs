/// <p>The sum of the primes below $10$ is $2 + 3 + 5 + 7 = 17$.</p>
/// <p>Find the sum of all the primes below two million.</p>

fn main() {
    const TOP: u64 = 2_000_000;
    let mut sum = 5; // Sum of the first two primes, 2 and 3.

    // All subsequent primes are of the form 6k+1 or 6k-1.
    for i in (6..TOP).step_by(6) {
        if is_prime(i - 1) {
            sum += i - 1;
        }

        if i + 1 < TOP {
            if is_prime(i + 1) {
                sum += i + 1;
            }
        } else {
            break;
        }
    }

    dbg!(sum);
}

fn is_prime(n: u64) -> bool {
    for d in 2..=((n as f64).sqrt().floor() as u64) {
        if n % d == 0 {
            return false;
        }
    }
    true
}
