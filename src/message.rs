use crate::{Deserialize, Serialize};
use std::{io, string::FromUtf8Error};

/// Magic number that must be present in the message header to be considered valid.
pub const MAGIC_NUMBER: u32 = 0xD9B4BEF9;

/// Message header used by all communication with the nodes.
#[derive(Debug)]
pub struct Message {
    magic: u32,
    command: [u8; 12],
    length: u32,
    checksum: u32,
}

impl Message {
    /// Create a new message.
    pub fn new(command: [u8; 12], length: u32, checksum: u32) -> Self {
        Self {
            magic: MAGIC_NUMBER,
            command,
            length,
            checksum,
        }
    }

    /// Returns the magic number. Useful to verify the validity of the message.
    pub fn magic(&self) -> u32 {
        self.magic
    }

    /// Command name that identifies the type of payload coming down the line.
    pub fn command(&self) -> Result<String, FromUtf8Error> {
        let command = String::from_utf8(self.command.into())?;

        Ok(command.trim_end_matches(char::from(0)).to_owned())
    }

    /// Total length of the payload in bytes.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Checksum of the incoming payload.
    pub fn checksum(&self) -> u32 {
        self.checksum
    }
}

impl Deserialize for Message {
    fn to_bytes(&self) -> Vec<u8> {
        const SIZE: usize = std::mem::size_of::<Message>();

        let mut buffer = Vec::with_capacity(SIZE);

        buffer.extend(self.magic.to_le_bytes());
        buffer.extend(self.command);
        buffer.extend(self.length.to_le_bytes());
        buffer.extend(self.checksum.to_le_bytes());

        buffer
    }

    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
        let buffer = self.to_bytes();

        writer.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl Serialize for Message {
    fn from_reader<R: io::Read>(reader: &mut R) -> Result<Self, io::Error> {
        // Read magic number.
        let mut buffer = [0u8; std::mem::size_of::<u32>()];
        reader.read_exact(&mut buffer)?;
        let magic = u32::from_le_bytes(buffer);

        // Read command.
        let mut command = [0u8; 12];
        reader.read_exact(&mut command)?;

        // Read length.
        let mut buffer = [0u8; std::mem::size_of::<u32>()];
        reader.read_exact(&mut buffer)?;
        let length = u32::from_le_bytes(buffer);

        // Read checksum.
        let mut buffer = [0u8; std::mem::size_of::<u32>()];
        reader.read_exact(&mut buffer)?;
        let checksum = u32::from_le_bytes(buffer);

        // Return object.
        Ok(Self {
            magic,
            command,
            length,
            checksum,
        })
    }
}
