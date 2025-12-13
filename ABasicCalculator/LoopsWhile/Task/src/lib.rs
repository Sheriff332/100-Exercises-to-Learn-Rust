// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.
    let mut product:u32 = 1;
    let mut counter:u32 = 0;
    while counter<n {
        counter+=1;
        product=product*counter;
    }
    product
}
