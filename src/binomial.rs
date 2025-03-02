use rand_distr::{Binomial, Distribution};

use crate::CmdArgs;

pub fn binomial_number(args: &CmdArgs) -> Vec<f64> {
    let mut numbers: Vec<f64> = Vec::new();
// Todo mean_value must be usize, check it before rust panicked
// Todo p_value has to be <= 1 or rust panicked
    let binomial = Binomial::new(args.mean_value as u64, args.p_value as f64).unwrap();

    for _ in 0..args.elements {
        numbers.push(binomial.sample(&mut rand::rng()) as f64);
    }

    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_of_elements() {
        let args = CmdArgs {
            port: 7878,
            elements: 20,
            binding_adress: "127.0.0.1".to_string(),
            mean_value: 10,
            p_value: 0.5,
            deviation_value: 100,
            distribution: "binomial".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        assert_eq!(binomial_number(&args).len(), 20);
    }
}
