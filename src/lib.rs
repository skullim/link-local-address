mod arp;
pub mod error;
mod host_detection;

use crate::error::Result;
use arp::Client;
use ipnet::{Ipv4Net, Ipv6Net};
use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
    time::Duration,
};

pub enum IpAddrType {
    V4,
    V6,
}

struct LocalLinkNetProvider;

impl LocalLinkNetProvider {
    pub fn provide(ip_addr_type: IpAddrType) -> Vec<IpAddr> {
        match ip_addr_type {
            IpAddrType::V4 => Self::provide_ipv4(),
            IpAddrType::V6 => Self::provide_ipv6(),
        }
    }

    fn provide_ipv4() -> Vec<IpAddr> {
        Ipv4Net::new(Ipv4Addr::new(169, 254, 0, 0), 16)
            .unwrap()
            .hosts()
            .filter(|ip| {
                let octets = ip.octets();
                !(octets[2] == 0 || octets[2] == 255)
            })
            .map(Into::into)
            .collect()
    }

    fn provide_ipv6() -> Vec<IpAddr> {
        Ipv6Net::from_str("fe80::/64")
            .unwrap()
            .hosts()
            .map(Into::into)
            .collect()
    }
}

//iterate and check the range one by one, random, some other smarter methods
pub trait IpRangeSearchStrategy {
    fn search(&mut self) -> Option<IpAddr>;
}

pub struct IterativeStrategy {
    ip_range: Vec<IpAddr>,
}

impl IterativeStrategy {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = IpAddr>,
    {
        Self {
            ip_range: iter.into_iter().collect(),
        }
    }
}

impl IpRangeSearchStrategy for IterativeStrategy {
    fn search(&mut self) -> Option<IpAddr> {
        todo!()
    }
}

pub struct FreeIpFinderBuilder {
    interface: String,
    ip_addr_type: IpAddrType,
    search_strategy: Box<dyn IpRangeSearchStrategy>,
    //@todo use HostDetector instead
    pinger: Client,
}

impl FreeIpFinderBuilder {
    pub fn new(interface: &str) -> Result<Self> {
        Ok(Self {
            interface: interface.into(),
            ip_addr_type: IpAddrType::V4,
            search_strategy: Box::new(IterativeStrategy::new(LocalLinkNetProvider::provide(
                IpAddrType::V4,
            ))),
            pinger: Client::new(
                interface.into(),
                Duration::from_secs(1),
                Duration::from_secs(60),
            )?,
        })
    }

    pub fn build(self) -> FreeIpFinder {
        FreeIpFinder {
            ip_addr_type: self.ip_addr_type,
            interface: self.interface,
            search_strategy: self.search_strategy,
            pinger: self.pinger,
        }
    }
}

pub struct FreeIpFinder {
    ip_addr_type: IpAddrType,
    interface: String,
    //iterate and check the range one by one, random, some other smarter methods
    search_strategy: Box<dyn IpRangeSearchStrategy>,
    pinger: Client,
}

impl FreeIpFinder {
    pub fn builder(interface: &str) -> Result<FreeIpFinderBuilder> {
        FreeIpFinderBuilder::new(interface)
    }

    pub async fn find(&self) -> Result<IpAddr> {
        todo!()
    }
}

struct NetworkInterfaceConfigurator {
    name: String,
}

impl NetworkInterfaceConfigurator {
    pub fn new(name: &str) -> Self {
        todo!()
    }

    pub fn assign_ip(&mut self, ip_addr: &IpAddr) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tdd_ip_find() {
        //let next_ip_result = FreeIpFinder::builder("eth0").build().find().await;
        todo!()
    }
}
