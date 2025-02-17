use std::{num::NonZero, time::Duration};

use async_arp::{
    Client as ArpClient, ClientConfigBuilder as ArpClientConfigBuilder,
    ClientSpinner as ArpClientSpinner,
};
use link_local_address::{
    batcher::IpBatcher, finder::FreeIpFinder, link_local_config::LinkLocalConfig,
    net::LocalLinkNet, probe::Ipv4HostProber, selector::SequentialIpSelector,
};
use pnet::util::MacAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let arp_client = ArpClient::new(
        ArpClientConfigBuilder::new("wlp4s0")
            .with_response_timeout(Duration::from_millis(100))
            .build(),
    )
    .unwrap();
    let spinner = ArpClientSpinner::new(arp_client).with_retries(1);
    let ipv4_prober = Ipv4HostProber::new(spinner, MacAddr::zero()).unwrap();
    let selector = SequentialIpSelector::new(LocalLinkNet::ipv4());
    let ip_batcher = IpBatcher::new(NonZero::new(16).unwrap(), selector);

    let mut finder = FreeIpFinder::new(ip_batcher, Box::new(ipv4_prober));

    if let Some(next_free) = finder.find_next().await {
        println!("{:?}", next_free);

        let configurator = LinkLocalConfig::new("dummy0").unwrap();
        let ip = next_free.first().unwrap();
        println!("Before: {:?}", configurator.addresses().unwrap());
        configurator.configure((*ip).into()).unwrap();
        println!("After: {:?}", configurator.addresses().unwrap());
    }
}
