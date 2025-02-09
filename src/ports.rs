use crate::{
    common_ports::MOST_COMMON_PORTS,
    model::{Port, Subdomain},
    Error, ScannerConfig,
};
use rayon::prelude::*;
use std::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};

pub fn scan_ports(mut subdomain: Subdomain, config: &ScannerConfig) -> Result<Subdomain, Error> {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .map_err(|e| Error::AddressError(e.to_string()))?
        .collect();

    if socket_addresses.is_empty() {
        return Ok(subdomain);
    }

    subdomain.open_ports = MOST_COMMON_PORTS
        .iter()
        .take(config.max_ports)
        .par_bridge()
        .map(|port| scan_port(socket_addresses[0], *port, config))
        .filter_map(Result::ok)
        .filter(|port| port.is_open)
        .collect();

    Ok(subdomain)
}

fn scan_port(
    mut socket_address: SocketAddr,
    port: u16,
    config: &ScannerConfig,
) -> Result<Port, Error> {
    socket_address.set_port(port);

    let is_open = TcpStream::connect_timeout(&socket_address, config.port_timeout)
        .map(|_| true)
        .unwrap_or(false);

    Ok(Port { port, is_open })
}
