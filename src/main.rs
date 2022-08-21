use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use std::time::Duration;

mod error;
pub use error::Error;
mod model;
mod ports;
mod subdomains;
use model::Subdomain;
mod common_ports;
use clap::{ArgAction, Parser};

#[derive(Parser)]
struct CliArgs {
    /// Domain to scan for subdomains
    #[clap(short = 'd', long = "domain")]
    domain: String,
    /// Scan open ports for all subdomains
    #[clap(short = 'p', long = "ports", action = ArgAction::SetTrue)]
    ports: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let args = CliArgs::parse();

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    pool.install(|| {
        if args.ports {
            let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, &args.domain)
                .unwrap()
                .into_par_iter()
                .map(ports::scan_ports)
                .collect();

            for subdomain in scan_result {
                println!("{}:", subdomain.domain);
                for port in &subdomain.open_ports {
                    println!("    {}", port.port);
                }
            }
        } else {
            let scan_result: Vec<Subdomain> =
                subdomains::enumerate(&http_client, &args.domain).unwrap();
            for subdomain in scan_result {
                println!("{}", subdomain.domain);
            }
        }
    });

    Ok(())
}
