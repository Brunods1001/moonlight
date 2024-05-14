use std::ops::{Add, Mul};
use std::iter::Sum;
pub trait Distribution<T> {
    fn mean(&self) -> T;
    fn variance(&self) -> T;
    fn pdf(&self, x: T) -> T; // Probability Density Function for continuous distributions
    fn pmf(&self, x: T) -> T; // Probability Mass Function for discrete distributions
}

struct Normal {
    mu: f64,    // mean
    sigma: f64, // standard deviation
}

impl Distribution<f64> for Normal {
    fn mean(&self) -> f64 {
        self.mu
    }

    fn variance(&self) -> f64 {
        self.sigma * self.sigma
    }

    fn pdf(&self, x: f64) -> f64 {
        let s = self.sigma;
        let m = self.mu;
        (1.0 / (s * (2.0 * std::f64::consts::PI).sqrt())) * (-0.5 * ((x - m) / s).powi(2)).exp()
    }

    fn pmf(&self, _x: f64) -> f64 {
        unimplemented!("PMF is not defined for a continuous distribution like Normal.")
    }
}

/// The Bernoulli distribution is a discrete distribution with two possible outcomes.
struct Bernoulli {
    p: f64, // probability of success
}

impl Distribution<f64> for Bernoulli {
    fn mean(&self) -> f64 {
        self.p
    }

    fn variance(&self) -> f64 {
        self.p * (1.0 - self.p)
    }

    fn pdf(&self, x: f64) -> f64 {
        if x == 0.0 {
            1.0 - self.p
        } else if x == 1.0 {
            self.p
        } else {
            0.0
        }
    }

    fn pmf(&self, x: f64) -> f64 {
        self.pdf(x)
    }
}

/// The Binomial distribution is a discrete distribution with n independent Bernoulli trials.
/// It is the sum of n independent Bernoulli random variables.
struct Binomial {
    n: u32, // number of trials
    p: f64, // probability of success
}

impl Distribution<f64> for Binomial {
    fn mean(&self) -> f64 {
        self.n as f64 * self.p
    }

    fn variance(&self) -> f64 {
        self.n as f64 * self.p * (1.0 - self.p)
    }

    fn pdf(&self, x: f64) -> f64 {
        if x < 0.0 || x > self.n as f64 {
            return 0.0;
        }
        let p = self.p;
        let binomial_coefficient = (0..=self.n)
            .map(|k| {
                let n_minus_k = self.n - k;
                let n_fact: u32 = (1..=self.n).product();
                let k_fact: u32 = (1..=k).product();
                let n_minus_k_fact: u32 = (1..=n_minus_k).product();
                n_fact / (k_fact * n_minus_k_fact)
            })
            .nth(x as usize)
            .unwrap();
        (binomial_coefficient as f64) * p.powf(x) * (1.0 - p).powf((self.n as f64) - x)
    }

    fn pmf(&self, x: f64) -> f64 {
        self.pdf(x)
    }
}

fn calculate_expectation<T, D>(dist: &D, range: &[T]) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Sum<T> + Copy,
    D: Distribution<T>,
{
    range.iter()
         .map(|&x| x * dist.pdf(x)) // Multiplication here must conform to T: Mul<Output = T>
         .sum() // Summation must conform to T: Sum
}




fn main() {
    println!("An example of statistical distribution in Rust.");
    let normal = Normal { mu: 0.0, sigma: 1.0 };
    let range = (-10..=10).map(|x| x as f64).collect::<Vec<f64>>();
    let expectation = calculate_expectation(&normal, &range);
    println!("Expectation: {}", expectation);
    println!("Variance: {}", normal.variance());
    println!("PDF at 0: {}", normal.pdf(0.0));

    println!("-----------------------------------");
    println!("Bernoulli Distribution");
    let bernoulli = Bernoulli { p: 0.5 };
    println!("Expectation: {}", bernoulli.mean());
    println!("Variance: {}", bernoulli.variance());
    println!("PDF at 0: {}", bernoulli.pdf(0.0));
    println!("PDF at 1: {}", bernoulli.pdf(1.0));

    println!("-----------------------------------");

    println!("Binomial Distribution");
    let binomial = Binomial { n: 10, p: 0.1 };
    println!("Expectation: {}", binomial.mean());
    println!("Variance: {}", binomial.variance());
    println!("PDF at 0: {}", binomial.pdf(0.0));
    println!("PDF at 1: {}", binomial.pdf(1.0));
    println!("PDF at 5: {}", binomial.pdf(5.0));
    println!("PDF at 10: {}", binomial.pdf(10.0));
}
