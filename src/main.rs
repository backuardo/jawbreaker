use crate::ports::scan_ports;
use crate::subdomains::enumerate;
pub use config::ScannerConfig;
pub use error::Error;
use futures::stream::{self, StreamExt};
use model::Subdomain;
use std::env;

mod common_ports;
mod config;
mod error;
mod model;
mod ports;
mod subdomains;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage);
    }

    let target = args[1].as_str();
    let config = ScannerConfig::default();

    let http_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(config.max_redirects))
        .timeout(config.http_timeout)
        .build()
        .map_err(|e| Error::Reqwest(e.to_string()))?;

    let subdomains = enumerate(&http_client, target, &config).await?;

    let scan_results = stream::iter(subdomains)
        .map(|subdomain| scan_ports(subdomain, &config))
        .buffer_unordered(config.concurrent_subdomain_scans)
        .filter_map(|result| async move { result.ok() })
        .collect::<Vec<Subdomain>>()
        .await;

    for subdomain in scan_results {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            println!("    {}", port.port);
        }
        println!();
    }

    Ok(())
}
