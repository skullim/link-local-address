use std::net::Ipv4Addr;

use crate::error::Result;
use async_arp::{Client as ArpClient, ProbeInput, ProbeOutcome as ProbeIpv4Outcome, ProbeStatus};
use async_trait::async_trait;
use derive_more::From;
use pnet::util::MacAddr;

pub struct NdpClient;
pub struct ProbeIpv6Outcome;

#[async_trait]
pub trait ProbeHost<Ip> {
    async fn probe(&self, ip: Ip) -> Result<ProbeOutcome>;
}
pub type HostProber<Ip> = Box<dyn ProbeHost<Ip>>;

#[derive(From)]
pub enum ProbeOutcome {
    Ipv4(ProbeIpv4Outcome),
    Ipv6(ProbeIpv6Outcome),
}

impl ProbeOutcome {
    pub fn is_free(&self) -> bool {
        match self {
            Self::Ipv4(outcome) => outcome.status == ProbeStatus::Free,
            Self::Ipv6(outcome) => todo!(),
        }
    }
}

pub struct Ipv4HostProber {
    client: ArpClient,
    mac_addr: MacAddr,
}

impl Ipv4HostProber {
    pub fn new(client: ArpClient, mac_addr: MacAddr) -> Result<Self> {
        Ok(Self { client, mac_addr })
    }
}

#[async_trait]
impl ProbeHost<Ipv4Addr> for Ipv4HostProber {
    //@todo e-h
    async fn probe(&self, ip: Ipv4Addr) -> Result<ProbeOutcome> {
        Ok(self
            .client
            .probe(ProbeInput {
                sender_mac: self.mac_addr,
                target_ip: ip,
            })
            .await
            .unwrap()
            .into())
    }
}
