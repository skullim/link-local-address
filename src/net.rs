use std::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{Ipv4Net, Ipv6Net};

pub(super) struct Net;

impl Net {
    const IPV4_NET: Ipv4Net = Ipv4Net::new_assert(Ipv4Addr::new(169, 254, 0, 0), 16);
    #[allow(dead_code)]
    const IPV6_NET: Ipv6Net = Ipv6Net::new_assert(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0), 64);

    pub(super) fn ipv4() -> Vec<Ipv4Addr> {
        Self::IPV4_NET
            .hosts()
            .filter(|ip| {
                let octets = ip.octets();
                !(octets[2] == 0 || octets[2] == 255)
            })
            .map(Into::into)
            .collect()
    }

    #[allow(dead_code)]
    pub(super) fn ipv6() -> Vec<Ipv6Addr> {
        Self::IPV6_NET.hosts().map(Into::into).collect()
    }
}
