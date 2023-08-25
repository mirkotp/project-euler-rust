/// <p>What is the index of the first term in the Fibonacci sequence to contain $1000$ digits?</p>

fn main() {
    // Consecutive elements of the Fibonacci sequence tend
    // to approximate the golden ratio, (1 + sqrt(5))/2.
    // We can use this information to obtain the index of the
    // first element with 1000 digits.

    let gr = (1. + 5_f64.sqrt()) / 2.;
    let index = (1. + 999. * 10_f64.log(gr)).ceil() as usize;
    dbg!(index);
}
