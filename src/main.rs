use std::time::Duration;

use async_arp::{
    Client as ArpClient, ClientConfigBuilder as ArpClientConfigBuilder,
    ClientSpinner as ArpClientSpinner,
};
use link_local_address::{
    probe::Ipv4HostProber, FreeIpFinder, IterativeStrategy, LocalLinkNetProvider,
};
use pnet::util::MacAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let arp_client = ArpClient::new(
        ArpClientConfigBuilder::new("wlp4s0")
            .with_response_timeout(Duration::from_millis(500))
            .build(),
    )
    .unwrap();
    let spinner = ArpClientSpinner::new(arp_client).with_retries(5);
    let ipv4_prober = Ipv4HostProber::new(spinner, MacAddr::zero()).unwrap();

    let mut finder = FreeIpFinder::builder()
        //@todo add batching to strategy
        .with_strategy(IterativeStrategy::new(LocalLinkNetProvider::provide_ipv4()))
        .with_host_prober(ipv4_prober)
        .with_batch_size(16)
        .build();
    let next_free = finder.find_next().await.unwrap();
    println!("{:?}", next_free);
}
