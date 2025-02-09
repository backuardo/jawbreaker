use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use std::env;

mod config;
mod error;
pub use config::ScannerConfig;
pub use error::Error;
mod model;
mod ports;
mod subdomains;
use model::Subdomain;
mod common_ports;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage);
    }

    let target = args[1].as_str();
    let config = ScannerConfig::default();
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(config.max_redirects))
        .timeout(config.http_timeout)
        .build()
        .map_err(|e| Error::Reqwest(e.to_string()))?;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.thread_count)
        .build()
        .map_err(|e| Error::ThreadPool(e.to_string()))?;

    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target, &config)?
            .into_par_iter()
            .map(|subdomain| ports::scan_ports(subdomain, &config))
            .filter_map(|result| result.ok())
            .collect();

        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("    {}", port.port);
            }
            println!();
        }

        Ok::<(), Error>(())
    })?;

    Ok(())
}
