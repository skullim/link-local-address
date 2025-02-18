use std::net::Ipv4Addr;

use crate::error::Result;
use async_arp::{
    ClientSpinner as ArpClientSpinner, ProbeInput, ProbeOutcome as ArpProbeOutcome, ProbeStatus,
};
use async_trait::async_trait;
use pnet::util::MacAddr;

pub(super) struct Outcome<Ip> {
    ip: Ip,
    status: ProbeStatus,
}

impl<Ip> Outcome<Ip> {
    pub(super) fn target_ip(&self) -> &Ip {
        &self.ip
    }
    pub(super) fn is_free(&self) -> bool {
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
pub(super) trait ProbeHost<Ip> {
    async fn probe(&self, ip_slice: &[Ip]) -> Result<Vec<Outcome<Ip>>>;
}

#[derive(Debug)]
pub(super) struct Ipv4HostProber {
    spinner: ArpClientSpinner,
    mac_addr: MacAddr,
}

impl Ipv4HostProber {
    pub(super) fn new(spinner: ArpClientSpinner, mac_addr: MacAddr) -> Self {
        Self { spinner, mac_addr }
    }
}

#[async_trait]
impl ProbeHost<Ipv4Addr> for Ipv4HostProber {
    async fn probe(&self, ip_slice: &[Ipv4Addr]) -> Result<Vec<Outcome<Ipv4Addr>>> {
        let inputs: Vec<_> = ip_slice
            .iter()
            .map(|ip| ProbeInput {
                sender_mac: self.mac_addr,
                target_ip: *ip,
            })
            .collect();

        Ok(self
            .spinner
            .probe_batch(&inputs)
            .await?
            .into_iter()
            .map(Into::<Outcome<Ipv4Addr>>::into)
            .collect())
    }
}
