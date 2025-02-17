use std::{num::NonZero, time::Duration};

use async_arp::{
    Client as ArpClient, ClientConfigBuilder as ArpClientConfigBuilder,
    ClientSpinner as ArpClientSpinner,
};
use link_local_address::{
    net_config::LinkLocalInterfaceConfigurator, probe::Ipv4HostProber, FreeIpFinder, IpBatcher,
    LocalLinkNetProvider, SequentialIpSelector,
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
    let selector = SequentialIpSelector::new(LocalLinkNetProvider::provide_ipv4());
    let ip_batcher = IpBatcher::new(NonZero::new(16).unwrap(), selector);

    let mut finder = FreeIpFinder::builder()
        .with_ip_batcher(ip_batcher)
        .with_host_prober(ipv4_prober)
        .build();

    if let Some(next_free) = finder.find_next().await {
        println!("{:?}", next_free);
    }

    if let Some(next_free) = finder.find_next().await {
        println!("{:?}", next_free);

        let configurator = LinkLocalInterfaceConfigurator::new("dummy2").unwrap();
        let ip = next_free.first().unwrap();
        println!("Before: {:?}", configurator.addresses().unwrap());
        configurator.configure((*ip).into()).unwrap();
        println!("After: {:?}", configurator.addresses().unwrap());
    }
}
