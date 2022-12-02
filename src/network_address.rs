use crate::{Deserialize, Serialize};
use std::{borrow::Borrow, io, net::IpAddr};

/// Network address of a node, including the listening port and services.
#[derive(Debug, Clone)]
pub struct NetworkAddress {
    services: u64,
    ip_v6: [u8; 16],
    port: u16,
}

impl NetworkAddress {
    /// Create a new `NetworkAddress` object.
    pub fn new<IP>(services: u64, ip_address: IP, port: u16) -> Self
    where
        IP: Borrow<IpAddr>,
    {
        let ip_address = ip_address.borrow();
        let ip_v6 = match ip_address {
            IpAddr::V4(ip_address) => ip_address.to_ipv6_mapped().octets(),
            IpAddr::V6(ip_address) => ip_address.octets(),
        };

        Self {
            services,
            ip_v6,
            port,
        }
    }
}

impl Deserialize for NetworkAddress {
    fn to_bytes(&self) -> Vec<u8> {
        const SIZE: usize = std::mem::size_of::<NetworkAddress>();

        let mut buffer = Vec::with_capacity(SIZE);

        buffer.extend(self.services.to_be_bytes());
        buffer.extend(self.ip_v6);
        buffer.extend(self.port.to_be_bytes());

        buffer
    }

    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
        let buffer = self.to_bytes();

        writer.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl Serialize for NetworkAddress {
    fn from_reader<R: io::Read>(reader: &mut R) -> Result<Self, io::Error> {
        // Read services.
        let mut buffer = [0u8; std::mem::size_of::<u64>()];
        reader.read_exact(&mut buffer)?;
        let services = u64::from_le_bytes(buffer);

        // Read Internet Protocol Address version 6.
        let mut ip_v6 = [0u8; 16];
        reader.read_exact(&mut ip_v6)?;

        // Read port.
        let mut buffer = [0u8; std::mem::size_of::<u16>()];
        reader.read_exact(&mut buffer)?;
        let port = u16::from_le_bytes(buffer);

        // Return object.
        Ok(Self {
            services,
            ip_v6,
            port,
        })
    }
}
