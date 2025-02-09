use std::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net, Ipv6Net};

fn main() {
    let net4 = Ipv4Net::new(Ipv4Addr::new(169, 254, 0, 0), 16).unwrap();

    let hosts: Vec<_> = net4
        .hosts()
        .filter(|ip| {
            let octets = ip.octets();
            !(octets[2] == 0 || octets[2] == 255)
        })
        .collect();

    for ip in hosts {
        println!("{}", ip);
    }
}
