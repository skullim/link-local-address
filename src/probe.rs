use std::net::Ipv4Addr;

use crate::error::Result;
use async_arp::{
    ClientSpinner as ArpClientSpinner, ProbeInput, ProbeOutcome as ArpProbeOutcome, ProbeStatus,
};
use async_trait::async_trait;
use pnet::util::MacAddr;

pub struct NdpClient;
pub struct ProbeIpv6Outcome;

pub struct Outcome<Ip> {
    ip: Ip,
    status: ProbeStatus,
}

impl<Ip> Outcome<Ip> {
    pub fn target_ip(&self) -> &Ip {
        &self.ip
    }
    pub fn is_free(&self) -> bool {
        self.status == ProbeStatus::Free
    }
}

impl From<ArpProbeOutcome> for Outcome<Ipv4Addr> {
    fn from(value: ArpProbeOutcome) -> Self {
        Self {
            ip: value.target_ip,
            status: value.status,
        }
    }
}

#[async_trait]
pub trait ProbeHost<Ip> {
    async fn probe(&self, ip_slice: &[Ip]) -> Vec<Result<Outcome<Ip>>>;
}
pub type HostProber<Ip> = Box<dyn ProbeHost<Ip>>;

pub struct Ipv4HostProber {
    spinner: ArpClientSpinner,
    mac_addr: MacAddr,
}

impl Ipv4HostProber {
    pub fn new(spinner: ArpClientSpinner, mac_addr: MacAddr) -> Result<Self> {
        Ok(Self { spinner, mac_addr })
    }
}

#[async_trait]
impl ProbeHost<Ipv4Addr> for Ipv4HostProber {
    //@todo e-h
    async fn probe(&self, ip_slice: &[Ipv4Addr]) -> Vec<Result<Outcome<Ipv4Addr>>> {
        let inputs: Vec<_> = ip_slice
            .iter()
            .map(|ip| ProbeInput {
                sender_mac: self.mac_addr,
                target_ip: *ip,
            })
            .collect();

        self.spinner
            .probe_batch(&inputs)
            .await
            .into_iter()
            .filter_map(|outcome| outcome.ok())
            .map(Into::<Outcome<Ipv4Addr>>::into)
            .map(Ok)
            .collect()
    }
}
