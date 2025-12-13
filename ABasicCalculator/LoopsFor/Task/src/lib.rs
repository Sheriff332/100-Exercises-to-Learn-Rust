// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for  i in 2..=n {
        product *= i;
    }
    product
}
