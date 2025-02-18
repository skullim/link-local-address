use async_arp::{
    Client as ArpClient, ClientConfigBuilder as ArpClientConfigBuilder,
    ClientSpinner as ArpClientSpinner,
};
use std::{net::Ipv4Addr, num::NonZeroUsize, time::Duration};

use pnet::util::MacAddr;
use typed_builder::TypedBuilder;

use crate::{
    batcher::IpBatcher, error::Result, finder::FreeIpFinder, net::Net, probe::Ipv4HostProber,
    selector::SequentialIpSelector,
};

#[derive(TypedBuilder, Debug)]
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

#[derive(TypedBuilder, Debug)]
pub struct Ipv4HandlerConfig {
    #[builder(default)]
    scan: Ipv4ScanConfig,
    #[builder(default = NonZeroUsize::new(32).unwrap())]
    batch_size: NonZeroUsize,
    interface: &'static str,
    mac_addr: MacAddr,
}

pub struct Ipv4Handler {
    finder: FreeIpFinder<Ipv4Addr, SequentialIpSelector<Ipv4Addr>>,
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
        let prober = Ipv4HostProber::new(spinner, config.mac_addr);

        let selector = SequentialIpSelector::new(Net::ipv4());
        let ip_batcher = IpBatcher::new(config.batch_size, selector);
        let finder = FreeIpFinder::new(ip_batcher, Box::new(prober));

        Ok(Self { finder })
    }

    pub async fn next_free_ip_batch(&mut self) -> Option<Vec<Ipv4Addr>> {
        while let Some(v) = self.finder.find_next().await {
            if !v.is_empty() {
                return Some(v);
            }
        }
        None
    }
}
