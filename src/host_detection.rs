use crate::{arp::Client, IpAddrType};

//@todo use ArpPinger for IpV4, and NDP (Neighbour Discovery Protocol) for IpV6
struct HostProber {
    arp_pinger: Client,
}

impl HostProber {
    fn new(ip_addr_type: &IpAddrType) -> Self {
        todo!()
    }

    fn probe(&self) -> ProbeOutcome {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ProbeOutcome {
    Free,
    Occupied,
}
