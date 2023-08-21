/// <p>$2520$ is the smallest number that can be divided by each of the numbers from $1$ to $10$ without any remainder.</p>
/// <p>What is the smallest positive number that is <dfn class="tooltip">evenly divisible<span class="tooltiptext">divisible with no remainder</span></dfn> by all of the numbers from $1$ to $20$?</p>

fn main() {
    'outer: for n in 2520.. {
        for d in 2..=20 {
            if n % d != 0 {
                continue 'outer;
            }
        }

        dbg!(n);
        return;
    }
}
