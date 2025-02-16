use rand_distr::{ Distribution, ChiSquared};

use crate::CmdArgs;

pub fn chi_number(args: &CmdArgs) -> Vec<f64> {
    let mut numbers: Vec<f64> = Vec::new();

    let chi = ChiSquared::new(1 as f32).unwrap();

    for _ in 0..args.elements {
        let chi_value = chi.sample(&mut rand::rng()) as f64;
        numbers.push((args.mean_value as f64) * (1 as f64 + chi_value) );
    }

    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lower_limit() {
        let args = CmdArgs {
            port: 7878,
            elements: 20,
            binding_adress: "127.0.0.1".to_string(),
            mean_value: 10,
            deviation_value: 100,
            distribution: "chisquared".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        assert!(chi_number(&args).len() >= args.mean_value as usize);
    }

    #[test]
    fn test_count_of_elements() {
        let args = CmdArgs {
            port: 7878,
            elements: 20,
            binding_adress: "127.0.0.1".to_string(),
            mean_value: 10,
            deviation_value: 100,
            distribution: "chisqaured".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        assert_eq!(chi_number(&args).len(), 20);
    }
}
