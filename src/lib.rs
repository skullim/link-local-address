pub mod error;
pub mod net_config;
pub mod probe;

use ipnet::{Ipv4Net, Ipv6Net};
use probe::{HostProber, ProbeHost};
use std::net::Ipv6Addr;
use std::num::NonZeroUsize;
use std::{net::Ipv4Addr, str::FromStr};

pub struct LocalLinkNetProvider;

impl LocalLinkNetProvider {
    pub fn provide_ipv4() -> Vec<Ipv4Addr> {
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

    pub fn provide_ipv6() -> Vec<Ipv6Addr> {
        Ipv6Net::from_str("fe80::/64")
            .unwrap()
            .hosts()
            .map(Into::into)
            .collect()
    }
}

pub trait SelectIp<Ip> {
    fn select(&mut self) -> Option<&Ip>;
}
pub type IpSelector<Ip> = Box<dyn SelectIp<Ip>>;

pub struct SequentialIpSelector<Ip> {
    ip_range: Vec<Ip>,
    index: usize,
}

impl<Ip> SequentialIpSelector<Ip> {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Ip>,
    {
        Self {
            ip_range: iter.into_iter().collect(),
            index: 0,
        }
    }
}

impl<Ip> SelectIp<Ip> for SequentialIpSelector<Ip> {
    fn select(&mut self) -> Option<&Ip> {
        let index = self.index;
        if index < self.ip_range.len() {
            self.index += 1;
            Some(&self.ip_range[index])
        } else {
            None
        }
    }
}

pub struct FreeIpFinder<Ip, S> {
    ip_batcher: IpBatcher<Ip, S>,
    host_prober: HostProber<Ip>,
}

impl<Ip, S> FreeIpFinder<Ip, S> {
    fn new(ip_batcher: IpBatcher<Ip, S>, host_prober: HostProber<Ip>) -> Self {
        Self {
            ip_batcher,
            host_prober,
        }
    }
}

impl<Ip: Clone, S: SelectIp<Ip>> FreeIpFinder<Ip, S> {
    pub fn builder() -> FreeIpFinderBuilder<Ip, S> {
        FreeIpFinderBuilder::default()
    }

    pub async fn find_next(&mut self) -> Option<Vec<Ip>> {
        if let Some(batch) = self.ip_batcher.next_batch() {
            let outcomes = self.host_prober.probe(batch).await;
            Some(
                outcomes
                    .into_iter()
                    .filter_map(|outcome| outcome.ok())
                    .filter(|ok_outcome| ok_outcome.is_free())
                    .map(|free_outcome| free_outcome.target_ip().clone())
                    .collect(),
            )
        } else {
            None
        }
    }
}

pub struct IpBatcher<Ip, S> {
    batch: Vec<Ip>,
    size: usize,
    selector: S,
}

impl<Ip: Clone, S: SelectIp<Ip>> IpBatcher<Ip, S> {
    pub fn new(size: NonZeroUsize, selector: S) -> Self {
        let batch = Vec::with_capacity(size.into());
        Self {
            batch,
            size: size.into(),
            selector,
        }
    }

    fn next_batch(&mut self) -> Option<&[Ip]> {
        self.batch.clear();
        for _ in 0..self.size {
            if let Some(next_ip) = self.selector.select() {
                self.batch.push(next_ip.clone());
            } else {
                return Some(&self.batch);
            }
        }
        Some(&self.batch)
    }
}

pub struct FreeIpFinderBuilder<Ip, S> {
    ip_batcher: Option<IpBatcher<Ip, S>>,
    host_prober: Option<HostProber<Ip>>,
}

impl<Ip, S> Default for FreeIpFinderBuilder<Ip, S> {
    fn default() -> Self {
        Self {
            ip_batcher: None,
            host_prober: None,
        }
    }
}

impl<Ip, S> FreeIpFinderBuilder<Ip, S> {
    pub fn with_ip_batcher(mut self, batcher: IpBatcher<Ip, S>) -> Self {
        self.ip_batcher = Some(batcher);
        self
    }

    pub fn with_host_prober<P>(mut self, host_prober: P) -> Self
    where
        P: ProbeHost<Ip> + 'static,
    {
        self.host_prober = Some(Box::new(host_prober));
        self
    }

    //@todo: improve API by state pattern to always have those fields set
    pub fn build(self) -> FreeIpFinder<Ip, S> {
        FreeIpFinder::new(self.ip_batcher.unwrap(), self.host_prober.unwrap())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn tdd_ip_find() {
        todo!()
    }
}
