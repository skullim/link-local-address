use crate::error::Result;
use std::net::IpAddr;

use ipnet::IpNet;
use netconfig::Interface;

#[derive(Debug)]
pub struct NetConfigurator {
    interface: Interface,
}

impl NetConfigurator {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Self {
            interface: Interface::try_from_name(name).map_err(|err| {
                format!("failed to instantiate net configurator, reason: {}", err)
            })?,
        })
    }

    /// Ensures that `host_ip` is the only assigned IP address on the interface.
    ///
    /// If the interface has any existing IP addresses, they will be removed before
    /// adding the new one. This guarantees that only `host_ip` remains assigned.
    pub fn add_unique(&self, host_ip: IpAddr) -> Result<()> {
        for net in self.addresses()? {
            self.remove(net)?;
        }
        self.add(host_ip)
    }

    pub fn add(&self, host_ip: IpAddr) -> Result<()> {
        let prefix_len = match host_ip {
            IpAddr::V4(_) => 16,
            IpAddr::V6(_) => 10,
        };
        let net = IpNet::new_assert(host_ip, prefix_len);
        Ok(self
            .interface
            .add_address(net)
            .map_err(|err| format!("failed to add {} network, reason: {}", net, err))?)
    }

    pub fn addresses(&self) -> Result<Vec<IpNet>> {
        Ok(self
            .interface
            .addresses()
            .map_err(|err| format!("failed to obtain interface address, reason: {}", err))?)
    }

    fn remove(&self, net: IpNet) -> Result<()> {
        Ok(self
            .interface
            .remove_address(net)
            .map_err(|err| format!("failed to remove address, reason: {}", err))?)
    }
}

#[cfg(test)]
mod tests {
    use crate::net_configurator::NetConfigurator;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    static INTERFACE_NAME: &str = "dummy0";

    #[test]
    fn test_configure_ipv4() {
        let configurator = NetConfigurator::new(INTERFACE_NAME).unwrap();
        let ip = IpAddr::V4(Ipv4Addr::new(169, 254, 149, 255));

        let result = configurator.add(ip);
        assert!(result.is_ok(), "Failed to configure IPv4 address");
        let addresses = configurator.addresses().unwrap();

        let configured_nets: Vec<_> = addresses
            .into_iter()
            .filter(|net| net.contains(&ip))
            .collect();

        assert_eq!(configured_nets.len(), 1);
        assert_eq!(configured_nets.first().unwrap().prefix_len(), 16);
    }

    #[test]
    fn test_configure_ipv6() {
        let configurator = NetConfigurator::new(INTERFACE_NAME).unwrap();
        let ip = IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 1, 1, 1, 1));

        let result = configurator.add(ip);
        assert!(result.is_ok(), "Failed to configure IPv6 address");
        let addresses = configurator.addresses().unwrap();

        let configured_nets: Vec<_> = addresses
            .into_iter()
            .filter(|net| net.contains(&ip))
            .collect();

        assert_eq!(configured_nets.len(), 1);
        assert_eq!(configured_nets.first().unwrap().prefix_len(), 10);
    }

    #[test]
    fn test_unique_add_ipv4() {
        let configurator = NetConfigurator::new(INTERFACE_NAME).unwrap();
        configurator
            .add(IpAddr::V4(Ipv4Addr::new(169, 254, 149, 252)))
            .unwrap();
        configurator
            .add(IpAddr::V4(Ipv4Addr::new(169, 254, 149, 253)))
            .unwrap();
        assert_eq!(configurator.addresses().unwrap().len(), 2);

        let unique_ip = IpAddr::V4(Ipv4Addr::new(169, 254, 149, 253));
        configurator.add_unique(unique_ip).unwrap();

        let addresses = configurator.addresses().unwrap();
        assert_eq!(addresses.len(), 1);
        let assigned_ip = addresses.first().unwrap();
        assert_eq!(assigned_ip.addr(), unique_ip);
    }
}
