/// <p>The sequence of triangle numbers is generated by adding the natural numbers. So the $7$<sup>th</sup> triangle number would be $1 + 2 + 3 + 4 + 5 + 6 + 7 = 28$. The first ten terms would be:
/// $$1, 3, 6, 10, 15, 21, 28, 36, 45, 55, \dots$$</p>
/// <p>Let us list the factors of the first seven triangle numbers:</p>
/// \begin{align}
/// \mathbf 1 &amp;\colon 1\\
/// \mathbf 3 &amp;\colon 1,3\\
/// \mathbf 6 &amp;\colon 1,2,3,6\\
/// \mathbf{10} &amp;\colon 1,2,5,10\\
/// \mathbf{15} &amp;\colon 1,3,5,15\\
/// \mathbf{21} &amp;\colon 1,3,7,21\\
/// \mathbf{28} &amp;\colon 1,2,4,7,14,28
/// \end{align}
/// <p>We can see that $28$ is the first triangle number to have over five divisors.</p>
/// <p>What is the value of the first triangle number to have over five hundred divisors?</p>

fn main() {
    let mut triangular = 0;
    for i in 1.. {
        triangular += i;
        if count_div(triangular) > 500 {
            dbg!(triangular);
            return;
        }
    }
}

fn count_div(n: u32) -> u32 {
    let mut count = 2;
    for i in 2..=(n as f64).sqrt().floor() as u32 {
        if n % i == 0 {
            count += if i * i == n { 1 } else { 2 };
        }
    }
    count
}