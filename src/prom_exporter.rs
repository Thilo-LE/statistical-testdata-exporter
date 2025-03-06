use crate::{binomial::binomial_number, gauss::gauss_number};
use crate::uniform::uniform_number;
use crate::chisquared::chi_number;

use prometheus::{
    core::{AtomicF64, GenericGauge},
    Gauge, Opts, Registry,
};

use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::CmdArgs;

pub fn create_metrics(args: &CmdArgs) -> Result<Registry, &'static str> {
    let r = Registry::new();

    let scraping_start = Instant::now();

    if args.distribution == "all".to_string() {
        let gauss_sequence = gauss_number(args);

        for (i, number) in gauss_sequence.into_iter().enumerate() {
            r.register(Box::new(metric_item_gauss(number, i, args).clone()))
                .unwrap();
        }

        let uniform_sequence = uniform_number(args);

        for (i, number) in uniform_sequence.into_iter().enumerate() {
            r.register(Box::new(metric_item_uniform(number, i, args).clone()))
                .unwrap();
        }

        let chi_sequence = chi_number(args);

        for (i, number) in chi_sequence.into_iter().enumerate() {
            r.register(Box::new(metric_item_chi(number, i, args).clone()))
                .unwrap();
        }

        let binomial_sequence = binomial_number(args);

        for (i, number) in binomial_sequence.into_iter().enumerate() {
            r.register(Box::new(metric_item_binomial(number, i, args).clone()))
                .unwrap();
        }
    }
    else if args.distribution == "uniform".to_string() {
        let uniform_sequence = uniform_number(args);

        for (i, number) in uniform_sequence.into_iter().enumerate() {
            r.register(Box::new(metric_item_uniform(number, i, args).clone()))
                .unwrap();
        }
    }
    else if args.distribution == "chisquared".to_string() {
        let chi_sequence = chi_number(args);

        for (i, number) in chi_sequence.into_iter().enumerate() {
            r.register(Box::new(metric_item_chi(number, i, args).clone()))
                .unwrap();
        }
    }
    else if args.distribution == "binomial".to_string() {
        let binomial_sequence = binomial_number(args);

        for (i, number) in binomial_sequence.into_iter().enumerate() {
            r.register(Box::new(metric_item_binomial(number, i, args).clone()))
                .unwrap();
        }

    }
    else if args.distribution == "gaussian".to_string() { 
        let gauss_sequence = gauss_number(args);

        for (i, number) in gauss_sequence.into_iter().enumerate() {
            r.register(Box::new(metric_item_gauss(number, i, args).clone()))
                .unwrap();
        }
    }
    else {
        return Err("distribution is not supported");
    }

    let duration = scraping_start.elapsed().as_secs_f64();
    r.register(Box::new(
        metric_item_scrape_collector_duration(duration, args).clone(),
    ))
    .unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    r.register(Box::new(
        metric_item_scrape_timestamp_msec(timestamp, args).clone(),
    ))
    .unwrap();

    Ok(r)
}



fn metric_item_gauss(number: f64, i: usize, args: &CmdArgs) -> GenericGauge<AtomicF64> {
    let counter_opts = Opts::new("value_gaussian", "value of the requested distribution gaussian")
        .namespace(args.prefix.to_string())
        .const_label("mean", args.mean_value.to_string())
        .const_label("deviation", args.deviation_value.to_string())
        .const_label("element", i.to_string());

    let gauss_number = Gauge::with_opts(counter_opts).unwrap();

    gauss_number.add(number);

    gauss_number
}

fn metric_item_uniform(number: f64, i: usize, args: &CmdArgs) -> GenericGauge<AtomicF64> {
    let counter_opts = Opts::new("value_uniform", "value of the requested distribution uniform")
        .namespace(args.prefix.to_string())
        .const_label("min", (args.mean_value - args.deviation_value).to_string())
        .const_label("max", (args.mean_value + args.deviation_value).to_string())
        .const_label("element", i.to_string());

    let uniform_number = Gauge::with_opts(counter_opts).unwrap();

    uniform_number.add(number);

    uniform_number
}

fn metric_item_chi(number: f64, i: usize, args: &CmdArgs) -> GenericGauge<AtomicF64> {
    let counter_opts = Opts::new("value_chisquared", "value of the requested distribution chisquared")
        .namespace(args.prefix.to_string())
        .const_label("expected", args.mean_value.to_string())
        .const_label("element", i.to_string());

    let chi_number = Gauge::with_opts(counter_opts).unwrap();

    chi_number.add(number);

    chi_number
}

fn metric_item_binomial(number: f64, i: usize, args: &CmdArgs) -> GenericGauge<AtomicF64> {
    let counter_opts = Opts::new("value_binomial", "value of the requested distribution Binomial")
        .namespace(args.prefix.to_string())
        .const_label("trails", args.mean_value.to_string())
        .const_label("probability", args.p_value.to_string())
        .const_label("element", i.to_string());

    let binomial_number = Gauge::with_opts(counter_opts).unwrap();

    binomial_number.add(number);

    binomial_number
}

fn metric_item_scrape_collector_duration(
    duration_sec: f64,
    args: &CmdArgs,
) -> GenericGauge<AtomicF64> {
    let counter_opts = Opts::new(
        "scrape_collector_duration_seconds",
        "time duration of scraping",
    )
    .namespace(args.prefix.to_string());

    let scrape_duration = Gauge::with_opts(counter_opts).unwrap();

    scrape_duration.add(duration_sec);

    scrape_duration
}

fn metric_item_scrape_timestamp_msec(scrape_time: u128, args: &CmdArgs) -> GenericGauge<AtomicF64> {
    let counter_opts = Opts::new("scrape_timestamp_msec", "timestamp of scraping")
        .namespace(args.prefix.to_string());

    let scrape_timestamp = Gauge::with_opts(counter_opts).unwrap();

    scrape_timestamp.add(scrape_time as f64);

    scrape_timestamp
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_metric_as_gauss() {
        let args = CmdArgs {
            port: 7878,
            elements: 1,
            binding_address: "127.0.0.1".to_string(),
            mean_value: 100,
            p_value: 0.5,
            deviation_value: 10,
            distribution: "gauss".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        let item = metric_item_gauss(42 as f64, 0 as usize, &args);

        assert_eq!(item.get(), 42 as f64);
    }

    #[test]
    fn test_metric_as_uniform() {
        let args = CmdArgs {
            port: 7878,
            elements: 1,
            binding_address: "127.0.0.1".to_string(),
            mean_value: 100,
            p_value: 0.5,
            deviation_value: 10,
            distribution: "gauss".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        let item = metric_item_uniform(42 as f64, 0 as usize, &args);

        assert_eq!(item.get(), 42 as f64);
    }

    #[test]
    fn test_metric_as_chi() {
        let args = CmdArgs {
            port: 7878,
            elements: 1,
            binding_address: "127.0.0.1".to_string(),
            mean_value: 100,
            p_value: 0.5,
            deviation_value: 10,
            distribution: "gauss".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        let item = metric_item_chi(42 as f64, 0 as usize, &args);

        assert_eq!(item.get(), 42 as f64);
    }

    #[test]
    fn test_metric_as_binomial() {
        let args = CmdArgs {
            port: 7878,
            elements: 1,
            binding_address: "127.0.0.1".to_string(),
            mean_value: 100,
            p_value: 0.5,
            deviation_value: 10,
            distribution: "binomial".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        let item = metric_item_binomial(42 as f64, 0 as usize, &args);

        assert_eq!(item.get(), 42 as f64);
    }

    #[test]
    fn test_metric_duration() {
        let args = CmdArgs {
            port: 7878,
            elements: 1,
            binding_address: "127.0.0.1".to_string(),
            mean_value: 100,
            p_value: 0.5,            
            deviation_value: 10,
            distribution: "gauss".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        let item = metric_item_scrape_collector_duration(42 as f64, &args);

        assert_eq!(item.get(), 42 as f64);
    }

    #[test]
    fn test_metric_timestamp() {
        let args = CmdArgs {
            port: 7878,
            elements: 1,
            binding_address: "127.0.0.1".to_string(),
            mean_value: 100,
            p_value: 0.5,            
            deviation_value: 10,
            distribution: "gauss".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        let item = metric_item_scrape_timestamp_msec(123456789 as u128, &args);

        assert_eq!(item.get(), 123456789 as f64);
    }
}
