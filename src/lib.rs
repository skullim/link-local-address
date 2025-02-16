pub mod error;
pub mod net_config;
pub mod probe;

use crate::error::Result;
use ipnet::{Ipv4Net, Ipv6Net};
use itertools::{Chunks, IntoChunks, Itertools};
use probe::{HostProber, ProbeHost};
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

pub struct ChunksHandler<'a, Ip> {
    selector: IpChunksSelector<Ip>,
    chunks: Option<IntoChunks<Iter<'a, Ip>>>,
    chunk_size: NonZeroUsize,
}

impl<'a, Ip> ChunksHandler<'a, Ip> {
    pub fn new(selector: IpChunksSelector<Ip>, chunk_size: NonZeroUsize) -> Self {
        Self {
            selector,
            chunks: None,
            chunk_size,
        }
    }

    pub fn chunks(&'a mut self) -> Option<Chunks<'a, Iter<'a, Ip>>> {
        if self.chunks.is_none() {
            self.chunks = Some(self.selector.chunks(self.chunk_size));
        }
        self.chunks.as_ref().map(|chunks| chunks.into_iter())
    }
}

pub trait SelectIpChunks<Ip> {
    fn chunks(&self, size: NonZeroUsize) -> IntoChunks<Iter<'_, Ip>>;
}
pub type IpChunksSelector<Ip> = Box<dyn SelectIpChunks<Ip>>;

pub struct SequentialChunkSelector<Ip> {
    ip_range: Vec<Ip>,
}

impl<Ip> SequentialChunkSelector<Ip> {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Ip>,
    {
        Self {
            ip_range: iter.into_iter().collect(),
        }
    }
}

impl<Ip> SelectIpChunks<Ip> for SequentialChunkSelector<Ip> {
    fn chunks(&self, size: NonZeroUsize) -> IntoChunks<Iter<'_, Ip>> {
        self.ip_range.iter().chunks(size.into())
    }
}

pub struct FreeIpFinder<'a, Ip> {
    ip_chunks: Chunks<'a, Iter<'a, Ip>>,
    host_prober: HostProber<Ip>,
}

impl<'a, Ip> FreeIpFinder<'a, Ip> {
    fn new(ip_chunks: Chunks<'a, Iter<'a, Ip>>, host_prober: HostProber<Ip>) -> Self {
        Self {
            ip_chunks,
            host_prober,
        }
    }
}

impl<'a, Ip: Copy> FreeIpFinder<'a, Ip> {
    pub fn builder() -> FreeIpFinderBuilder<'a, Ip> {
        FreeIpFinderBuilder::default()
    }

    pub async fn find_next(&mut self) -> Option<Vec<Ip>> {
        if let Some(chunk) = self.ip_chunks.next() {
            let ips: Vec<_> = chunk.copied().collect();
            let outcomes = self.host_prober.probe(&ips).await;
            Some(
                outcomes
                    .into_iter()
                    .filter_map(|outcome| outcome.ok())
                    .filter(|ok_outcome| ok_outcome.is_free())
                    .map(|free_outcome| *free_outcome.target_ip())
                    .collect(),
            )
        } else {
            None
        }
    }
}

pub struct FreeIpFinderBuilder<'a, Ip> {
    ip_chunks: Option<Chunks<'a, Iter<'a, Ip>>>,
    host_prober: Option<HostProber<Ip>>,
}

impl<'a, Ip> Default for FreeIpFinderBuilder<'_, Ip> {
    fn default() -> Self {
        Self {
            ip_chunks: None,
            host_prober: None,
        }
    }
}

impl<'a, Ip> FreeIpFinderBuilder<'a, Ip> {
    pub fn with_ip_chunks(mut self, chunks: Chunks<'a, Iter<'a, Ip>>) -> Self {
        self.ip_chunks = Some(chunks);
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
    pub fn build(self) -> FreeIpFinder<'a, Ip> {
        FreeIpFinder::new(self.ip_chunks.unwrap(), self.host_prober.unwrap())
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
