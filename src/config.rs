use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ScannerConfig {
    pub http_timeout: Duration,
    pub dns_timeout: Duration,
    pub port_timeout: Duration,
    pub scan_delay: Duration,
    pub concurrent_subdomain_scans: usize,
    pub concurrent_port_scans: usize,
    pub concurrent_resolves: usize,
    pub max_redirects: usize,
    pub max_ports: usize,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            http_timeout: Duration::from_secs(5),
            dns_timeout: Duration::from_secs(4),
            port_timeout: Duration::from_secs(3),
            scan_delay: Duration::from_millis(10),
            concurrent_subdomain_scans: 256,
            concurrent_port_scans: 100,
            concurrent_resolves: 50,
            max_redirects: 4,
            max_ports: 1000,
        }
    }
}


impl ScannerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_http_timeout(mut self, timeout: Duration) -> Self {
        self.http_timeout = timeout;
        self
    }

    pub fn with_dns_timeout(mut self, timeout: Duration) -> Self {
        self.dns_timeout = timeout;
        self
    }

    pub fn with_port_timeout(mut self, timeout: Duration) -> Self {
        self.port_timeout = timeout;
        self
    }

    pub fn with_scan_delay(mut self, delay: Duration) -> Self {
        self.scan_delay = delay;
        self
    }

    pub fn with_concurrent_subdomain_scans(mut self, count: usize) -> Self {
        self.concurrent_subdomain_scans = count;
        self
    }

    pub fn with_concurrent_port_scans(mut self, count: usize) -> Self {
        self.concurrent_port_scans = count;
        self
    }

    pub fn with_concurrent_resolves(mut self, count: usize) -> Self {
        self.concurrent_resolves = count;
        self
    }

    pub fn with_max_redirects(mut self, max: usize) -> Self {
        self.max_redirects = max;
        self
    }

    pub fn with_max_ports(mut self, max: usize) -> Self {
        self.max_ports = max;
        self
    }
}