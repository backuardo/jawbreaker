use crate::{
    model::{CrtShEntry, Subdomain},
    Error, ScannerConfig,
};
use reqwest::blocking::Client;
use std::collections::HashSet;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

pub fn enumerate(
    http_client: &Client,
    target: &str,
    config: &ScannerConfig,
) -> Result<Vec<Subdomain>, Error> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()
        .map_err(|e| Error::Reqwest(e.to_string()))?;

    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains('*'))
        .collect();
    subdomains.insert(target.to_string());

    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter(|subdomain| resolves(subdomain, config))
        .collect();

    Ok(subdomains)
}

fn resolves(domain: &Subdomain, config: &ScannerConfig) -> bool {
    let mut opts = ResolverOpts::default();
    opts.timeout = config.dns_timeout;

    let dns_resolver = match Resolver::new(ResolverConfig::default(), opts) {
        Ok(resolver) => resolver,
        Err(_) => return false,
    };

    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
