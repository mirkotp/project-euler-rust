/// <p>A Pythagorean triplet is a set of three natural numbers, $a \lt b \lt c$, for which,
/// $$a^2 + b^2 = c^2.$$</p>
/// <p>For example, $3^2 + 4^2 = 9 + 16 = 25 = 5^2$.</p>
/// <p>There exists exactly one Pythagorean triplet for which $a + b + c = 1000$.<br>Find the product $abc$.</p>

fn main() {
    for c in 1..1000 as u32 {
        for b in 1..c {
            for a in 1..b {
                if a + b + c == 1000 && a.pow(2) + b.pow(2) == c.pow(2) {
                    dbg!(a, b, c, a * b * c);
                    return;
                }
            }
        }
    }
}
