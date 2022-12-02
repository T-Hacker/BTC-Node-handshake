#![warn(missing_docs)]

//! This crate contains basic types and serialization/deserialization functionality
//! to be able to do a handshake with a Bitcoin Core Node.

mod message;
mod network_address;
mod variable_string;
mod version;

pub use message::Message;
pub use message::MAGIC_NUMBER;
pub use network_address::NetworkAddress;
pub use variable_string::VarStr;
pub use version::Version;

/// Used to deserialize objects to byte representation or to a stream writer.
pub trait Deserialize {
    /// Get a binary representation of the object.
    fn to_bytes(&self) -> Vec<u8>;

    /// Write to a stream the binary representation of the object.
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<usize, std::io::Error>;
}

/// Used to create objects from binary representation.
pub trait Serialize: Sized {
    /// Create the object from a stream reader.
    fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error>;
}

#[cfg(test)]
mod tests {
    use crate::{message::MAGIC_NUMBER, Deserialize, Message, NetworkAddress, Serialize, Version};
    use std::{io::Write, net::IpAddr, time::UNIX_EPOCH};

    #[test]
    fn handshake() {
        let address = std::env::var("BTC_NODE_ADDRESS")
            .expect("Environment variable BTC_NODE_ADDRESS is not set.");
        let address = address.trim();
        println!("Connecting to {}...", address);

        let mut stream = std::net::TcpStream::connect(address).unwrap();

        // Create payload message.
        let timestamp = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let address: IpAddr = "127.0.0.1".parse().unwrap();
        let address = NetworkAddress::new(0, address, 8333);

        let version_message = Version::new(
            60002,
            0,
            timestamp,
            address.clone(),
            address,
            Default::default(),
            0,
        );

        let payload = version_message.to_bytes();

        // Create message header.
        let command = *b"version\0\0\0\0\0";
        let checksum = version_message.calculate_checksum();
        let message = Message::new(command, payload.len() as u32, checksum);

        // Send header message.
        message.to_writer(&mut stream).unwrap();

        // Send version message.
        stream.write_all(&payload).unwrap();

        // Flush socket to make sure the messages arrive to the destination.
        stream.flush().unwrap();

        // Receive response header.
        println!("Waiting for response from node...");
        let message = Message::from_reader(&mut stream).unwrap();

        // Do basic validation on response.
        assert_eq!(
            message.magic(),
            MAGIC_NUMBER,
            "Magic number is not correct."
        );

        assert_eq!(
            message.command().unwrap(),
            "version",
            "Command is not correct."
        );

        // Receive version payload.
        println!("Waiting for version payload...");
        let version = Version::from_reader(&mut stream).unwrap();

        println!("User agent: {}", version.user_agent());
    }
}
