use std::{collections::HashSet, time::Duration};

use reqwest::blocking::Client;
use crate::{
    model::{CrtShEntry, Subdomain},
    Error,
};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

const CRT_API: &str = "https://crt.sh";


pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {

    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("{}/?q=%25.{}&output=json", CRT_API, target))
        .send()?
        .json()?;
    
    // clean and dedup results
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain| subdomain != target )
        .filter(|subdomain| !subdomain.contains('*'))
        .collect();
    
    subdomains.insert(target.to_string());

    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter(resolves)
        .collect();

    Ok(subdomains)
}

pub fn resolves(domain: &Subdomain) -> bool {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    let dns_resolver = Resolver::new(
        ResolverConfig::default(),
        opts,
    )
    .expect("subdomain resolver: building DNS client");

    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}