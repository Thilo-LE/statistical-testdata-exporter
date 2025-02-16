use rand_distr::{Normal, Distribution};

use crate::CmdArgs;


pub fn gauss_number(args: &CmdArgs) -> Vec<f64> {
    let mut numbers: Vec<f64> = Vec::new();

    let normal = Normal::new(2.0, 3.0).unwrap();

    for _ in 0..args.elements {
        numbers.push(normal.sample(&mut rand::rng()) as f64);
    }

    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_in_range() {
        let args = CmdArgs {
            port: 7878,
            elements: 1,
            binding_adress: "127.0.0.1".to_string(),
            min_value: 10,
            max_value: 100,
            distribution: "gauss".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        let sequence = gauss_number(&args);
        let number = sequence[0];

        assert!(number < args.max_value as f64);
        assert!(number > args.min_value as f64);
    }

    #[test]
    fn test_count_of_elements() {
        let args = CmdArgs {
            port: 7878,
            elements: 20,
            binding_adress: "127.0.0.1".to_string(),
            min_value: 10,
            max_value: 100,
            distribution: "gauss".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        assert_eq!(gauss_number(&args).len(), 20);
    }
}
