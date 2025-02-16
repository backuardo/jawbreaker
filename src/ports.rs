use crate::{
    common_ports::MOST_COMMON_PORTS,
    model::{Port, Subdomain},
    Error, ScannerConfig,
};
use futures::stream::{self, StreamExt};
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn scan_ports(
    mut subdomain: Subdomain,
    config: &ScannerConfig,
) -> Result<Subdomain, Error> {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .map_err(|e| Error::AddressError(e.to_string()))?
        .collect();

    if socket_addresses.is_empty() {
        return Ok(subdomain);
    }

    let base_addr = socket_addresses[0];

    let ports_stream = stream::iter(MOST_COMMON_PORTS.iter().take(config.max_ports).copied());

    let open_ports: Vec<Port> = ports_stream
        .map(|port| scan_port(base_addr, port, config))
        .buffer_unordered(config.concurrent_port_scans)
        .filter_map(|result| async move {
            match result {
                Ok(port) if port.is_open => Some(port),
                _ => None,
            }
        })
        .collect()
        .await;

    subdomain.open_ports = open_ports;
    Ok(subdomain)
}

async fn scan_port(
    mut socket_address: SocketAddr,
    port: u16,
    config: &ScannerConfig,
) -> Result<Port, Error> {
    socket_address.set_port(port);

    if config.scan_delay > Duration::ZERO {
        tokio::time::sleep(Duration::from_millis(fastrand::u64(
            0..config.scan_delay.as_millis() as u64,
        )))
        .await;
    }

    let is_open = match timeout(config.port_timeout, TcpStream::connect(socket_address)).await {
        Ok(Ok(_)) => true,
        _ => false,
    };

    Ok(Port { port, is_open })
}
