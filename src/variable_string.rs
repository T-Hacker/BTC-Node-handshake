use crate::{Deserialize, Serialize};
use std::io;

/// A string representation of variable length.
#[derive(Debug, Default)]
pub struct VarStr {
    data: String,
}

impl VarStr {
    /// Text stored inside the string.
    pub fn text(&self) -> &str {
        &self.data
    }
}

impl Deserialize for VarStr {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.data.len() + 9);

        let data_size = self.data.len();
        if data_size < 0xFD {
            buffer.push(data_size as u8);
        } else if data_size <= 0xFFFF {
            let data_size = data_size as u16;
            buffer.extend(data_size.to_le_bytes());
        } else if data_size <= 0xFFFFFFFF {
            let data_size = data_size as u32;
            buffer.extend(data_size.to_le_bytes());
        } else {
            let data_size = data_size as u64;
            buffer.extend(data_size.to_le_bytes());
        }

        // Add string characters.
        buffer.extend(self.data.as_bytes());

        buffer
    }

    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
        let buffer = self.to_bytes();

        writer.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl Serialize for VarStr {
    fn from_reader<R: io::Read>(reader: &mut R) -> Result<Self, io::Error> {
        // Read the first byte to determine the storage length of the string.
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let flag = u8::from_le_bytes(buffer);

        // Read the size of the string.
        let size = match flag {
            // The string size is represented using 16 bits.
            0xFD => {
                let mut buffer = [0u8; std::mem::size_of::<u16>()];
                reader.read_exact(&mut buffer)?;

                u16::from_le_bytes(buffer) as usize
            }

            // The string size is represented using 32 bits.
            0xFE => {
                let mut buffer = [0u8; std::mem::size_of::<u32>()];
                reader.read_exact(&mut buffer)?;

                u32::from_le_bytes(buffer) as usize
            }

            // The string size is represented using 64 bits.
            0xFF => {
                let mut buffer = [0u8; std::mem::size_of::<u64>()];
                reader.read_exact(&mut buffer)?;

                u64::from_le_bytes(buffer) as usize
            }

            // The size is specified using only 8 bits.
            _ => flag as usize,
        };

        // Read the string bytes.
        let mut buffer = vec![0u8; size];
        reader.read_exact(&mut buffer)?;

        // Return object.
        let data =
            String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(Self { data })
    }
}
