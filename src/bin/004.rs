/// <p>A palindromic number reads the same both ways. The largest palindrome made from the product of two $2$-digit numbers is $9009 = 91 \times 99$.</p>
/// <p>Find the largest palindrome made from the product of two $3$-digit numbers.</p>

fn main() {
    let mut big = 0;
    for i in 100..=999 {
        for j in 100..=999 {
            let mul = i * j;
            if is_palindrome(mul) && mul > big {
                big = mul;
            }
        }
    }

    dbg!(big);
}

fn is_palindrome(n: u32) -> bool {
    if n == 0 {
        return true;
    }

    let pow = n.ilog(10);
    let reverse = {
        let mut reverse = 0;
        let mut current = n;

        for _ in 0..=pow {
            reverse *= 10;
            reverse += current % 10;
            current /= 10;
        }

        reverse
    };

    n == reverse
}
