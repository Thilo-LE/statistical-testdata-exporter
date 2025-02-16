use rand_distr::{num_traits::float, Distribution, Normal};

use crate::CmdArgs;

pub fn gauss_number(args: &CmdArgs) -> Vec<f64> {
    let mut numbers: Vec<f64> = Vec::new();

    let normal = Normal::new(args.mean_value as f32, args.deviation_value as f32).unwrap();

    for _ in 0..args.elements {
        numbers.push(normal.sample(&mut rand::rng()) as f64);
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
            deviation_value: 100,
            distribution: "gauss".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        assert_eq!(gauss_number(&args).len(), 20);
    }
}
