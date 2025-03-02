use std::io::Read;
use std::io::Write;
use std::vec::Vec;

use prometheus::{Encoder, TextEncoder};
mod prom_exporter;

mod gauss;
mod uniform;
mod chisquared;
mod binomial;

use std::net::TcpListener;
use std::net::TcpStream;

mod args_cmd;
use args_cmd::CmdArgs;
use clap::Parser;

fn main() {
    let args = CmdArgs::parse();
    println!("Arguments are -> {:?}", args);

    let addr = format!("{}:{}", args.binding_adress, args.port);

    if args.dry_run == 'y' {
        execute_dry_run(&args);
    } else {
        execute_http(&addr, &args);
    }
}

fn execute_dry_run(args: &CmdArgs) {
    println!("{}", String::from_utf8(get_metrics(args)).unwrap());
}

fn execute_http(addr: &String, args: &CmdArgs) {
    let listener = TcpListener::bind(addr).unwrap_or_else(|err| {
        println!("Error when start the TcpListener: {}", err);
        std::process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, args);
    }
}

fn get_metrics(args: &CmdArgs) -> Vec<u8> {
    let register = crate::prom_exporter::create_metrics(args);

    let mut buffer = vec![];
    let encoder = TextEncoder::new();

    let metric_families = register.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    buffer
}

fn handle_connection(mut stream: TcpStream, args: &CmdArgs) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let accepted_uri = b"GET /metrics HTTP/1.1\r\n";

    let (status_line, content) = if buffer.starts_with(accepted_uri) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            String::from_utf8(get_metrics(args)).unwrap(),
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            String::from("404 not found"),
        )
    };

    let response = format!("{}{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
