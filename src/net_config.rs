use crate::error::Result;
use std::net::IpAddr;

use ipnet::IpNet;
use netconfig::Interface;

pub struct LinkLocalInterfaceConfigurator {
    interface: Interface,
}

impl LinkLocalInterfaceConfigurator {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Self {
            interface: Interface::try_from_name(name).map_err(|err| err.to_string())?,
        })
    }

    pub fn configure(&self, host_ip: IpAddr) -> Result<()> {
        let prefix_len = match host_ip {
            IpAddr::V4(_) => 16,
            IpAddr::V6(_) => 10,
        };
        Ok(self
            .interface
            .add_address(IpNet::new_assert(host_ip, prefix_len))
            .map_err(|err| err.to_string())?)
    }

    pub fn addresses(&self) -> Result<Vec<IpNet>> {
        Ok(self.interface.addresses().map_err(|err| err.to_string())?)
    }
}
