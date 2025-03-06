use std::io::Read;
use std::io::Write;
use std::vec::Vec;
use std::str;

use regex::Regex;

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

    let addr = format!("{}:{}", args.binding_address, args.port);

    if let Err(evaluate_args) = evaluate_args(&args) {
        panic!("{}", evaluate_args); 
    }

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
            "HTTP/1.1 200 OK\r\nContent-type: text/plain\r\n\r\n",
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

fn evaluate_args(args: &CmdArgs) -> Result<bool, &'static str> {

    if let Err(err_str) = evaluate_binding_address(args.binding_address.clone()) {
        return Err(err_str); 
    }

    if let Err(err_str) = evaluate_pvalue(args.p_value) {
        return Err(err_str);
    }

    Ok(true)
}

fn evaluate_binding_address(binding_address: String) -> Result<bool, &'static str> {

    let re = Regex::new(r"\b((25[0-5])|(2[0-4][0-9])|(1[0-9][0-9])|([1-9][0-9])|([0-9]))[.]((25[0-5])|(2[0-4][0-9])|(1[0-9][0-9])|([1-9][0-9])|([0-9]))[.]((25[0-5])|(2[0-4][0-9])|(1[0-9][0-9])|([1-9][0-9])|([0-9]))[.]((25[0-5])|(2[0-4][0-9])|(1[0-9][0-9])|([1-9][0-9])|([0-9]))\b").unwrap();


    if re.is_match(binding_address.as_str()) {
        Ok(true)
    }
    else {
        Err("the binding_address doesn't have the IPv4-format <0.0.0.0> to <255.255.255.255>")
    }
}

fn evaluate_pvalue(pvalue: f32) -> Result<bool, &'static str> {
    if pvalue >= 0 as f32 && pvalue <= 1 as f32 {
        Ok(true)
    }
    else {
        Err("p-value must be between 0 <= p-value <= 1")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_evaluate_binding_address_ok() {

        assert!(evaluate_binding_address("123.56.89.255".to_string()).is_ok());
    }

    #[test]
    fn test_evaluate_binding_address_err() {

        assert!(evaluate_binding_address("".to_string()).is_err());
        assert!(evaluate_binding_address("123".to_string()).is_err());
        assert!(evaluate_binding_address("abcdesf".to_string()).is_err());
        assert!(evaluate_binding_address("256.100.2.255".to_string()).is_err());
    }

    #[test]
    fn test_pvalue_ok() {
        assert!(evaluate_pvalue(0.2 as f32).is_ok());
    }

    #[test]
    fn test_pvalue_err() {
        assert!(evaluate_pvalue(1.2 as f32).is_err());
        assert!(evaluate_pvalue(-0.2 as f32).is_err());
    }
}
