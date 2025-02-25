use async_arp::{
    Client as ArpClient, ClientConfigBuilder as ArpClientConfigBuilder,
    ClientSpinner as ArpClientSpinner,
};
use mac_address::mac_address_by_name;
use std::{net::Ipv4Addr, num::NonZeroUsize, time::Duration};

use pnet::util::MacAddr;
use typed_builder::TypedBuilder;

use crate::{
    batcher::IpBatcher, error::Result, finder::FreeIpFinder, net::Net, probe::Ipv4HostProber,
    selector::SequentialIpSelector,
};

#[derive(TypedBuilder, Debug, Clone, Copy)]
pub struct Ipv4ScanConfig {
    #[builder(default = 5)]
    n_retries: usize,
    #[builder(default=Duration::from_millis(500))]
    response_timeout: Duration,
    #[builder(default=Duration::from_secs(60))]
    cache_timeout: Duration,
}

impl Default for Ipv4ScanConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(TypedBuilder, Debug, Clone, Copy)]
pub struct Ipv4HandlerConfig {
    #[builder(default)]
    scan: Ipv4ScanConfig,
    #[builder(default = NonZeroUsize::new(32).unwrap())]
    batch_size: NonZeroUsize,
    interface: &'static str,
}

#[derive(Debug)]
pub struct Ipv4Handler {
    finder: FreeIpFinder<Ipv4Addr, SequentialIpSelector<Ipv4Addr>, Ipv4HostProber>,
}

impl Ipv4Handler {
    pub fn new(config: Ipv4HandlerConfig) -> Result<Self> {
        let arp_client = ArpClient::new(
            ArpClientConfigBuilder::new(config.interface)
                .with_response_timeout(config.scan.response_timeout)
                .with_cache_timeout(config.scan.cache_timeout)
                .build(),
        )
        .map_err(|err| format!("failed to instantiate scanner, reason: {}", err))?;
        let spinner = ArpClientSpinner::new(arp_client).with_retries(config.scan.n_retries);
        let mac_addr_bytes = mac_address_by_name(config.interface)
            .map_err(|err| {
                format!(
                    "failed to obtain mac address from interface name, reason: {}",
                    err
                )
            })?
            .ok_or("network interface does not have assigned mac address")?
            .bytes();
        let prober = Ipv4HostProber::new(
            spinner,
            MacAddr::new(
                mac_addr_bytes[0],
                mac_addr_bytes[1],
                mac_addr_bytes[2],
                mac_addr_bytes[3],
                mac_addr_bytes[4],
                mac_addr_bytes[5],
            ),
        );
        let selector = SequentialIpSelector::new(Net::ipv4());
        let ip_batcher = IpBatcher::new(config.batch_size, selector);
        let finder = FreeIpFinder::new(ip_batcher, prober);

        Ok(Self { finder })
    }

    pub async fn next_free_ip_batch(&mut self) -> Result<Option<Vec<Ipv4Addr>>> {
        while let Some(v) = self.finder.find_next().await? {
            if !v.is_empty() {
                return Ok(Some(v));
            }
        }
        Ok(None)
    }
}
