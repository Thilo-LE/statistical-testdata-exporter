use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CmdArgs {
    /// which port is used
    #[arg(short, long, default_value_t = 7878)]
    pub port: u32,
    /// binding adress
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub binding_adress: String,
    /// lower boundary
    #[arg(long, default_value_t = 100)]
    pub min_value: i32,
    /// upper boundary
    #[arg(long, default_value_t = 1000)]
    pub max_value: i32,
    /// count of elements
    #[arg(long, default_value_t = 10)]
    pub elements: i32,
    /// name of distribution gaussian, uniform, bernoulli
    #[arg(long, default_value_t = String::from("gaussian"))]
    pub distribution: String,
    /// namespace or prefix
    #[arg(long, default_value_t = String::from("statistical"))]
    pub prefix: String,
    /// dry-run print the current metric to stdout and exit
    #[arg(short, long, default_value_t = 'n')]
    pub dry_run: char,
}
