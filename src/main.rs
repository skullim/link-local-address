use std::{num::NonZero, time::Duration};

use async_arp::{
    Client as ArpClient, ClientConfigBuilder as ArpClientConfigBuilder,
    ClientSpinner as ArpClientSpinner,
};
use link_local_address::{
    net_config::LinkLocalInterfaceConfigurator, probe::Ipv4HostProber, ChunksHandler, FreeIpFinder,
    LocalLinkNetProvider, SequentialChunkSelector,
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
    let chunks_selector = SequentialChunkSelector::new(LocalLinkNetProvider::provide_ipv4());
    let mut handler = ChunksHandler::new(Box::new(chunks_selector), NonZero::new(16).unwrap());

    let mut finder = FreeIpFinder::builder()
        .with_ip_chunks(handler.chunks().unwrap())
        .with_host_prober(ipv4_prober)
        .build();

    if let Some(next_free) = finder.find_next().await {
        println!("{:?}", next_free);
    }

    let configurator = LinkLocalInterfaceConfigurator::new("dummy2").unwrap();
    if let Some(next_free) = finder.find_next().await {
        println!("{:?}", next_free);
        let ip = next_free.first().unwrap();
        println!("Before: {:?}", configurator.addresses().unwrap());
        configurator.configure((*ip).into()).unwrap();
        println!("After: {:?}", configurator.addresses().unwrap());
    }
}
