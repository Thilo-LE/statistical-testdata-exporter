use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CmdArgs {
    /// which port is used
    #[arg(short, long, default_value_t = 7878)]
    pub port: u32,
    /// binding address
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub binding_address: String,
    /// mean or expected value
    #[arg(long, default_value_t = 100)]
    pub mean_value: i32,
    /// probability value for Binomial
    #[arg(long, default_value_t = 0.5)]
    pub p_value: f32,
    /// deviation or (max-min)/2
    #[arg(long, default_value_t = 10)]
    pub deviation_value: i32,
    /// count of elements
    #[arg(long, default_value_t = 10)]
    pub elements: i32,
    /// name of distribution all, gaussian, uniform, binomial, chisquared
    #[arg(long, default_value_t = String::from("all"))]
    pub distribution: String,
    /// namespace or prefix
    #[arg(long, default_value_t = String::from("statistical"))]
    pub prefix: String,
    /// dry-run print the current metric to stdout and exit
    #[arg(short, long, default_value_t = 'n')]
    pub dry_run: char,
}
