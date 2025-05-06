// taylor.rs
/// Calculate sine using Taylor series expansion
pub fn sine_taylor(x: f64, n: i32) -> f64 {
    let mut result = 0.0;
    for i in 0..n {
        let term = (-1f64).powi(i) * x.powi(2 * i + 1) / factorial(2 * i + 1) as f64;
        result += term;
    }
    result
}

/// Calculate cosine using Taylor series expansion
pub fn cosine_taylor(x: f64, n: i32) -> f64 {
    let mut result = 0.0;
    for i in 0..n {
        let term = (-1f64).powi(i) * x.powi(2 * i) / factorial(2 * i) as f64;
        result += term;
    }
    result
}

/// Helper function to calculate factorial
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}