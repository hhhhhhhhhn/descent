const EPSILON: f64 = 0.0000000000001;

pub fn gradient_descent<const N: usize>(f: fn([f64; N]) -> f64, rate: f64) -> [f64; N] {
    let mut guess: [f64; N] = [0.0; N];

    for _ in 0..1000 {
        let image = f(guess);
        guess = guess
            .iter()
            .enumerate()
            .map(|(i, n)| {n - rate*calculate_gradient(f, guess, image, i)})
            .collect_array();
    }
    return guess;
}

pub fn calculate_gradient<const N: usize>(f: fn([f64; N]) -> f64, guess: [f64; N], value_at_guess: f64, parameter_index: usize) -> f64 {
    let mut guess_plus_epsilon = guess;
    guess_plus_epsilon[parameter_index] += EPSILON;

    return (f(guess_plus_epsilon) - value_at_guess) / EPSILON;
}

trait CollectArray<T> : Iterator<Item = T> {
    fn collect_array<const N: usize>(&mut self) -> [T; N]
        where T: Copy + std::default::Default;
}

impl<I: Iterator<Item = T>, T> CollectArray<T> for I {
    fn collect_array<const N: usize>(&mut self) -> [T; N]
        where T: Copy + std::default::Default
    {
        let mut fixed: [T; N] = [Default::default(); N];
        let mut i = 0;
        while i < N {
            match self.next() {
                Some(value) => { fixed[i] = value },
                None => break
            }
            i = i+1;
        }
        return fixed;
    }
}
