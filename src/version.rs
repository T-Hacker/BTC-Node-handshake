use crate::{Deserialize, NetworkAddress, Serialize, VarStr};
use rand::Rng;
use sha2::Digest;
use std::io;

/// Version message exchanged between nodes when initiating a new connection.
#[derive(Debug)]
pub struct Version {
    version: i32,
    services: u64,
    timestamp: i64,
    addr_recv: NetworkAddress,
    addr_from: NetworkAddress,
    nonce: u64,
    user_agent: VarStr,
    start_height: i32,
}

impl Version {
    /// Create a new version message to do the handshake between two nodes.
    pub fn new(
        version: i32,
        services: u64,
        timestamp: i64,
        addr_recv: NetworkAddress,
        addr_from: NetworkAddress,
        user_agent: VarStr,
        start_height: i32,
    ) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            version,
            services,
            timestamp,
            addr_recv,
            addr_from,
            nonce: rng.gen(),
            user_agent,
            start_height,
        }
    }

    /// User agent used by the node.
    pub fn user_agent(&self) -> &str {
        self.user_agent.text()
    }

    /// Generate the checksum to verify the integrity of this message.
    pub fn calculate_checksum(&self) -> u32 {
        let payload = self.to_bytes();

        let mut hasher = sha2::Sha256::default();
        hasher.update(payload);
        let checksum = hasher.finalize();

        let mut hasher = sha2::Sha256::default();
        hasher.update(checksum);
        let checksum = hasher.finalize();

        let checksum: [u8; 4] = checksum[..4].try_into().unwrap();
        u32::from_le_bytes(checksum)
    }
}

impl Deserialize for Version {
    fn to_bytes(&self) -> Vec<u8> {
        const SIZE: usize = std::mem::size_of::<Version>();

        let mut buffer = Vec::with_capacity(SIZE);

        buffer.extend(self.version.to_le_bytes());
        buffer.extend(self.services.to_le_bytes());
        buffer.extend(self.timestamp.to_le_bytes());
        buffer.extend(self.addr_recv.to_bytes());
        buffer.extend(self.addr_from.to_bytes());
        buffer.extend(self.nonce.to_le_bytes());
        buffer.extend(self.user_agent.to_bytes());
        buffer.extend(self.start_height.to_le_bytes());

        buffer
    }

    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
        let buffer = self.to_bytes();

        writer.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl Serialize for Version {
    fn from_reader<R: io::Read>(reader: &mut R) -> Result<Self, io::Error> {
        // Read version.
        let mut buffer = [0u8; std::mem::size_of::<i32>()];
        reader.read_exact(&mut buffer)?;
        let version = i32::from_le_bytes(buffer);

        // Read services.
        let mut buffer = [0u8; std::mem::size_of::<u64>()];
        reader.read_exact(&mut buffer)?;
        let services = u64::from_le_bytes(buffer);

        // Read timestamp.
        let mut buffer = [0u8; std::mem::size_of::<i64>()];
        reader.read_exact(&mut buffer)?;
        let timestamp = i64::from_le_bytes(buffer);

        // Read the receiver network address.
        let addr_recv = NetworkAddress::from_reader(reader)?;

        // Read the from network address.
        let addr_from = NetworkAddress::from_reader(reader)?;

        // Read nonce.
        let mut buffer = [0u8; std::mem::size_of::<u64>()];
        reader.read_exact(&mut buffer)?;
        let nonce = u64::from_le_bytes(buffer);

        // Read user agent string.
        let user_agent = VarStr::from_reader(reader)?;

        // Read start height.
        let mut buffer = [0u8; std::mem::size_of::<i32>()];
        reader.read_exact(&mut buffer)?;
        let start_height = i32::from_le_bytes(buffer);

        // Return object.
        Ok(Self {
            version,
            services,
            timestamp,
            addr_recv,
            addr_from,
            nonce,
            user_agent,
            start_height,
        })
    }
}
