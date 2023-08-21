/// <p>The prime factors of $13195$ are $5, 7, 13$ and $29$.</p>
/// <p>What is the largest prime factor of the number $600851475143$?</p>

fn main() {
    const N: u64 = 600_851_475_143;
    let mut cur = N;
    let mut mpd = 1;

    for i in 1.. {
        if i > cur {
            break;
        }

        if !is_prime(i) {
            continue;
        }

        if cur % i == 0 {
            cur /= i;
            mpd = i;
        }
    }

    dbg!(mpd);
}

fn is_prime(n: u64) -> bool {
    for d in 2..=((n as f64).sqrt().floor() as u64) {
        if n % d == 0 {
            return false;
        }
    }
    true
}
