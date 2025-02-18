use link_local_address::{Ipv4Handler, Ipv4HandlerConfig, Ipv4ScanConfig, NetConfigurator, Result};
use pnet::util::MacAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut handler = Ipv4Handler::new(
        Ipv4HandlerConfig::builder()
            .scan(Ipv4ScanConfig::default())
            .interface("wlp4s0")
            .mac_addr(MacAddr::zero())
            .build(),
    )?;

    while let Some(next_free) = handler.next_free_ip_batch().await {
        if let Some(ip) = next_free.first() {
            println!("first free ip: {:?}", ip);
            let configurator = NetConfigurator::new("dummy0")?;
            println!("Before: {:?}", configurator.addresses()?);
            configurator.add((*ip).into())?;
            println!("After: {:?}", configurator.addresses()?);
            break;
        }
    }
    Ok(())
}
