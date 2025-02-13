use async_arp::{Client as ArpClient, ClientConfigBuilder as ArpClientConfigBuilder};
use link_local_address::{
    probe::Ipv4HostProber, FreeIpFinder, IterativeStrategy, LocalLinkNetProvider,
};
use pnet::util::MacAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let arp_client = ArpClient::new(ArpClientConfigBuilder::new("wlp4s0").build()).unwrap();
    let ipv4_prober = Ipv4HostProber::new(arp_client, MacAddr::zero()).unwrap();

    let mut finder = FreeIpFinder::builder()
        .with_strategy(IterativeStrategy::new(LocalLinkNetProvider::provide_ipv4()))
        .with_host_prober(ipv4_prober)
        .build();
    let next_free = finder.find_next().await.unwrap();
    println!("{}", next_free);
}
