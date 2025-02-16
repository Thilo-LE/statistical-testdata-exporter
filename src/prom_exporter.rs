use crate::gauss::gauss_number;

use prometheus::{
    core::{AtomicF64, GenericGauge},
    Gauge, Opts, Registry,
};

use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::CmdArgs;

pub fn create_metrics(args: &CmdArgs) -> Registry {
    let r = Registry::new();

    create_metrics_gauss(&r, args);
    r
}

fn create_metrics_gauss(r: &Registry, args: &CmdArgs) {
    let scraping_start = Instant::now();

    let gauss_sequence = gauss_number(args);

    for (i, number) in gauss_sequence.into_iter().enumerate() {
        r.register(Box::new(metric_item_gauss(number, i, args).clone()))
            .unwrap();
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
}

fn metric_item_gauss(number: f64, i: usize, args: &CmdArgs) -> GenericGauge<AtomicF64> {
    let counter_opts = Opts::new("value", "value of the requested distribution")
        .namespace(args.prefix.to_string())
        .const_label("min", args.min_value.to_string())
        .const_label("max", args.max_value.to_string())
        .const_label("element", i.to_string())
        .const_label("distribution", args.distribution.to_string());

    let gauss_number = Gauge::with_opts(counter_opts).unwrap();

    gauss_number.add(number);

    gauss_number
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
            binding_adress: "127.0.0.1".to_string(),
            min_value: 10,
            max_value: 100,
            distribution: "gauss".to_string(),
            prefix: "statistical".to_string(),
            dry_run: 'n',
        };

        let item = metric_item_gauss(42 as f64, 0 as usize, &args);

        assert_eq!(item.get(), 42 as f64);
    }

    #[test]
    fn test_metric_duration() {
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

        let item = metric_item_scrape_collector_duration(42 as f64, &args);

        assert_eq!(item.get(), 42 as f64);
    }

    #[test]
    fn test_metric_timestamp() {
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

        let item = metric_item_scrape_timestamp_msec(123456789 as u128, &args);

        assert_eq!(item.get(), 123456789 as f64);
    }
}
