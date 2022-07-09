#![feature(fn_traits)]
use itertools::Itertools;

const EPSILON: f64 = 0.0000000000001;

pub fn gradient_descent<const N: usize>(f: fn([f64; N]) -> f64, rate: f64) -> [f64; N] {
    let mut guess: [f64; N] = [0.0; N];

    for _ in 0..1000 {
        let image = f(guess);
        let new_guess: Vec<f64> = guess
            .iter()
            .enumerate()
            .map(|(i, n)| {n - rate*calculate_gradient(f, guess, image, i)})
            .collect();
        for (i, n) in new_guess.iter().enumerate() {
            guess[i] = *n
        }
    }
    return guess;
}

pub fn calculate_gradient<const N: usize>(f: fn([f64; N]) -> f64, guess: [f64; N], value_at_guess: f64, parameter_index: usize) -> f64 {
    let mut guess_plus_epsilon = guess;
    guess_plus_epsilon[parameter_index] += EPSILON;

    return (f(guess_plus_epsilon) - value_at_guess) / EPSILON;
}

#[macro_export]
macro_rules! vectorize {
    ($function:ident, $size:expr) => {
    |params: [f64; $size]| -> f64 {
            return std::ops::Fn::call(&$function, params.iter().map(|x| *x).collect_tuple().unwrap())
        }
    }
}

//fn main() {
//    println!("{:?}", vectorize!(abc, 3)([2.0, 2.0, 2.0]));
//    println!("{:?}", gradient_descent(simple, 0.01));
//    println!("{:?}", gradient_descent(test, 0.01));
//    println!("{:?}", gradient_descent(vectorize!(abc, 3), 0.01));
//}
//
//fn abc(a: f64, b: f64, c: f64) -> f64 {
//    return (a*a - 5.0)*(b*b + 2.0)*(c*c - 3.0)
//}
//
//fn simple(params: [f64; 1]) -> f64 {
//    return (params[0]-1.0) * (params[0]-1.0);
//}
//
//fn test(params: [f64; 3]) -> f64 {
//    let x = params[0];
//    let y = params[1];
//    let z = params[2];
//
//    return x*x*x*x + (y - 4.0)*(y - 3.0) - (y + 3.0)*(x - 2.0) + (z*z * y*y);
//}
