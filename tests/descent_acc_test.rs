use descent::gradient_descent_acc;
use vectorize::vectorize;

fn within_a_tenth(actual: f64, expected: f64) -> bool {
    actual - expected < 0.1 && expected - actual < 0.1
}

#[test]
fn test_gradient_descent_acc_simple() {
    fn simple(params: [f64; 1]) -> f64 {
        return (params[0]-1.0) * (params[0]-1.0);
    }
    let result = gradient_descent_acc(simple, 0.01, 0.8);
    assert!(within_a_tenth(result[0], 1.0));
}

#[test]
fn test_gradient_descent_acc_complex() {
    fn complex(params: [f64; 3]) -> f64 {
        let x = params[0];
        let y = params[1];
        let z = params[2];

        return x*x*x*x + (y - 4.0)*(y - 3.0) - (y + 3.0)*(x - 2.0) + (z*z * y*y);
    }
    let result = gradient_descent_acc(complex, 0.01, 0.8);
    assert!(within_a_tenth(result[0], 1.14));
    assert!(within_a_tenth(result[1], 3.07));
    assert!(within_a_tenth(result[2], 0.0));
}

#[test]
fn test_gradient_acc_vectorized() {
    fn quadratic_regression(a: f64, b: f64, c: f64) -> f64 {
        let f = |x: f64| (a*x*x + b*x + c);
        return (f(1.0) - 1.0).powf(2.0) + (f(0.0) - 2.0).powf(2.0) + (f(-1.0) - 2.0).powf(2.0);
    }
    let result = gradient_descent_acc(vectorize!(quadratic_regression, 3), 0.01, 0.8);
    assert!(within_a_tenth(result[0], -0.5));
    assert!(within_a_tenth(result[1], -0.5));
    assert!(within_a_tenth(result[2], 2.0));
}
