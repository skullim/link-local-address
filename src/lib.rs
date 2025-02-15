pub mod error;
pub mod probe;

use crate::error::Result;
use ipnet::{Ipv4Net, Ipv6Net};
use probe::{HostProber, ProbeHost};
use std::fmt::Debug;
use std::net::Ipv6Addr;
use std::num::NonZeroUsize;
use std::slice::Iter;
use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

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

//iterate and check the range one by one, random, some other smarter methods
pub trait SelectIpStrategy<T> {
    fn iter(&self) -> Iter<T>;
}
pub type IpSelectionStrategy<Ip> = Box<dyn SelectIpStrategy<Ip>>;

pub struct IterativeStrategy<T> {
    ip_range: Vec<T>,
}

impl<T> IterativeStrategy<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            ip_range: iter.into_iter().collect(),
        }
    }
}

impl<T> SelectIpStrategy<T> for IterativeStrategy<T> {
    fn iter(&self) -> Iter<T> {
        self.ip_range.iter()
    }
}

pub struct FreeIpFinder<Ip> {
    strategy: IpSelectionStrategy<Ip>,
    host_prober: HostProber<Ip>,
    batch_size: NonZeroUsize,
}

impl<Ip: Copy> FreeIpFinder<Ip> {
    pub fn builder() -> FreeIpFinderBuilder<Ip> {
        FreeIpFinderBuilder::default()
    }

    pub async fn find_next(&mut self) -> Result<Vec<Ip>> {
        let probe_ips: Vec<_> = self
            .strategy
            .iter()
            .take(self.batch_size.into())
            .copied()
            .collect();
        let outcomes = self.host_prober.probe(&probe_ips).await;

        Ok(outcomes
            .into_iter()
            .filter_map(|outcome| outcome.ok())
            .filter(|ok_outcome| ok_outcome.is_free())
            .map(|free_outcome| *free_outcome.target_ip())
            .collect())
    }

    //@todo makes more sense to return in some batches, otherwise polling 65k futures in Ipv4 case is not the smartest idea,
    // not to mention Ipv6 range
    // async fn find_all(&self) -> Result<Vec<Ip>> {
    //     let future_probes = self
    //         .strategy
    //         .iter()
    //         .map(|ip| async move { self.host_prober.probe(*ip).await.unwrap() });
    //     let outcomes = futures::future::join_all(future_probes).await;

    //     Ok(outcomes
    //         .into_iter()
    //         .zip(self.strategy.iter())
    //         .filter(|(outcome, _)| outcome.is_free())
    //         .map(|(_, ip)| *ip)
    //         .collect())
    // }
}

pub struct FreeIpFinderBuilder<Ip> {
    strategy: Option<IpSelectionStrategy<Ip>>,
    host_prober: Option<HostProber<Ip>>,
    batch_size: NonZeroUsize,
}

impl<Ip> Default for FreeIpFinderBuilder<Ip> {
    fn default() -> Self {
        Self {
            strategy: None,
            host_prober: None,
            batch_size: unsafe { NonZeroUsize::new_unchecked(1) },
        }
    }
}

impl<Ip> FreeIpFinderBuilder<Ip> {
    pub fn with_strategy<S>(mut self, strategy: S) -> Self
    where
        S: SelectIpStrategy<Ip> + 'static,
    {
        self.strategy = Some(Box::new(strategy));
        self
    }

    pub fn with_host_prober<P>(mut self, host_prober: P) -> Self
    where
        P: ProbeHost<Ip> + 'static,
    {
        self.host_prober = Some(Box::new(host_prober));
        self
    }

    pub fn with_batch_size<T>(mut self, batch_size: T) -> Self
    where
        T: TryInto<NonZeroUsize>,
        T::Error: Debug,
    {
        self.batch_size = batch_size.try_into().unwrap();
        self
    }

    //@todo: improve API by state pattern to always have those fields set
    pub fn build(self) -> FreeIpFinder<Ip> {
        FreeIpFinder {
            strategy: self.strategy.unwrap(),
            host_prober: self.host_prober.unwrap(),
            batch_size: self.batch_size,
        }
    }
}

struct NetworkInterfaceConfigurator {
    name: String,
}

impl NetworkInterfaceConfigurator {
    pub fn new(name: &str) -> Self {
        todo!()
    }

    pub fn assign_ip<I>(&mut self, ip_addr: I)
    where
        I: Into<IpAddr>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tdd_ip_find() {
        todo!()
    }
}
