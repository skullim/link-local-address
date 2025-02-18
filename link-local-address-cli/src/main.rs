use std::{num::NonZeroUsize, str::FromStr, time::Duration};

use clap::Parser;
use link_local_address::{Ipv4Handler, Ipv4HandlerConfig, Ipv4ScanConfig, NetConfigurator, Result};
use log::{info, warn};
use pnet::util::MacAddr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The network interface to operate on (e.g., `eth0`, `wlan0`).
    #[arg(short, long)]
    interface: String,

    /// The MAC address of the device used for ARP-based conflict prevention.
    #[arg(short, long, value_parser=parse_mac_addr)]
    mac_addr: MacAddr,

    /// The number of retry attempts for ARP scanning.
    #[arg(short, long, default_value_t = 5)]
    retries: usize,

    /// Timeout for receiving ARP responses in milliseconds.
    #[arg(short='t', long, value_parser=parse_duration_ms, default_value ="500")]
    response_timeout: Duration,

    /// Timeout for caching ARP responses in seconds.
    #[arg(short, long, value_parser=parse_duration_s, default_value ="60")]
    cache_timeout: Duration,

    /// The number of IP addresses to process in each batch.
    #[arg(short, long, default_value_t = NonZeroUsize::new(32).unwrap())]
    batch_size: NonZeroUsize,
}

fn parse_mac_addr(arg: &str) -> Result<MacAddr> {
    Ok(MacAddr::from_str(arg)?)
}

fn parse_duration_ms(arg: &str) -> Result<Duration> {
    let ms = arg.parse()?;
    Ok(Duration::from_millis(ms))
}

fn parse_duration_s(arg: &str) -> Result<Duration> {
    let s = arg.parse()?;
    Ok(Duration::from_secs(s))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info)?;
    let mut handler = {
        let args = Args::parse();
        let interface = Box::leak(args.interface.into_boxed_str());
        Ipv4Handler::new(
            Ipv4HandlerConfig::builder()
                .scan(
                    Ipv4ScanConfig::builder()
                        .n_retries(args.retries)
                        .cache_timeout(args.cache_timeout)
                        .response_timeout(args.response_timeout)
                        .build(),
                )
                .interface(interface)
                .mac_addr(args.mac_addr)
                .batch_size(args.batch_size)
                .build(),
        )?
    };

    while let Some(next_free) = handler.next_free_ip_batch().await? {
        if let Some(ip) = next_free.first() {
            info!("Found first free ip: {:?}", ip);
            let configurator = NetConfigurator::new("dummy0")?;
            configurator.add((*ip).into())?;
            info!(
                "Interface addresses after adding link-local ip {:?}: {:?}",
                *ip,
                configurator.addresses()?
            );
            return Ok(());
        }
    }
    warn!("Failed to assign any valid link-local ip, no free ip was found");
    Ok(())
}
