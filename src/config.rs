use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ScannerConfig {
    pub http_timeout: Duration,
    pub dns_timeout: Duration,
    pub port_timeout: Duration,
    pub thread_count: usize,
    pub max_redirects: usize,
    pub max_ports: usize,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            http_timeout: Duration::from_secs(5),
            dns_timeout: Duration::from_secs(4),
            port_timeout: Duration::from_secs(3),
            thread_count: std::cmp::min(256, num_cpus::get() * 4),
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

    pub fn with_thread_count(mut self, count: usize) -> Self {
        self.thread_count = count;
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
