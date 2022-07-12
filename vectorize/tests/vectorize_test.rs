use vectorize::vectorize;

fn f(a: f64, b: f64) -> f64 {
    a + b
}

#[test]
fn test_vectorize() {
    let vectorized = vectorize!(f, 2);
    assert!(vectorized([2.0, 3.0]) == f(2.0, 3.0))
}
