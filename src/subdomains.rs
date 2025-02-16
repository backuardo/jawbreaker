use crate::{
    model::{CrtShEntry, Subdomain},
    Error, ScannerConfig,
};
use futures::stream::{self, StreamExt};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

pub async fn enumerate(
    http_client: &reqwest::Client,
    target: &str,
    config: &ScannerConfig,
) -> Result<Vec<Subdomain>, Error> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()
        .await?
        .json()
        .await
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
        .filter(|subdomain: &String| subdomain != target && !subdomain.contains('*'))
        .collect();
    subdomains.insert(target.to_string());

    let semaphore = Arc::new(Semaphore::new(config.concurrent_resolves));

    let subdomains: Vec<Subdomain> = stream::iter(subdomains)
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .map(|subdomain| {
            let sem = Arc::clone(&semaphore);
            async move {
                let _permit = sem.acquire().await.unwrap();
                (subdomain.clone(), resolves(&subdomain, config).await)
            }
        })
        .buffer_unordered(config.concurrent_resolves)
        .filter_map(|(subdomain, resolves)| async move {
            if resolves {
                Some(subdomain)
            } else {
                None
            }
        })
        .collect()
        .await;

    Ok(subdomains)
}

async fn resolves(domain: &Subdomain, config: &ScannerConfig) -> bool {
    let mut opts = ResolverOpts::default();
    opts.timeout = config.dns_timeout;

    let dns_resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), opts);

    match timeout(
        config.dns_timeout,
        dns_resolver.lookup_ip(domain.domain.as_str()),
    )
    .await
    {
        Ok(Ok(_)) => true,
        _ => false,
    }
}
