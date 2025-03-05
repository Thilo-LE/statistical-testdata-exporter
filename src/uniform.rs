use rand::Rng;

use crate::CmdArgs;

pub fn uniform_number(args: &CmdArgs) -> Vec<f64> {
    let mut numbers: Vec<f64> = Vec::new();

    let mut random_set = rand::rng();
    let min_value = args.mean_value - args.deviation_value;
    let max_value = args.mean_value + args.deviation_value;

    for _ in 0..args.elements {
        numbers.push(random_set.random_range(min_value..max_value) as f64);
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
            binding_address: "127.0.0.1".to_string(),
            mean_value: 50,
            p_value: 0.5,            
            deviation_value: 10,
            distribution: "uniform".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        let sequence = uniform_number(&args);
        let number = sequence[0];

        assert!(number < (args.mean_value + args.deviation_value) as f64);
        assert!(number >= (args.mean_value - args.deviation_value) as f64);
    }

    #[test]
    fn test_count_of_elements() {
        let args = CmdArgs {
            port: 7878,
            elements: 20,
            binding_address: "127.0.0.1".to_string(),
            mean_value: 50,
            p_value: 0.5,            
            deviation_value: 10,
            distribution: "uniform".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        assert_eq!(uniform_number(&args).len(), 20);
    }
}
