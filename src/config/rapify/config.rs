use std::io::{BufReader, BufWriter, Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use super::Class;
use crate::ArmaLintError;

#[derive(Debug)]
pub struct Config {
    pub root: Class,
}

impl Config {
    /// Writes the rapified config to the output.
    pub fn write_rapified<O: Write>(&self, output: &mut O) -> Result<(), ArmaLintError> {
        let mut writer = BufWriter::new(output);

        writer.write_all(b"\0raP")?;
        writer.write_all(b"\0\0\0\0\x08\0\0\0")?; // always_0, always_8

        let buffer: Box<[u8]> = vec![0; self.root.rapified_length()].into_boxed_slice();
        let mut cursor: Cursor<Box<[u8]>> = Cursor::new(buffer);
        self.root.write_rapified(&mut cursor, 16)?;

        let enum_offset: u32 = 16 + cursor.get_ref().len() as u32;
        writer.write_u32::<LittleEndian>(enum_offset)?;

        writer.write_all(cursor.get_ref())?;

        writer.write_all(b"\0\0\0\0")?;

        Ok(())
    }

    /// Returns the rapified config as a `Cursor`.
    pub fn to_cursor(&self) -> Result<Cursor<Box<[u8]>>, ArmaLintError> {
        let len = self.root.rapified_length() + 20;

        let buffer: Box<[u8]> = vec![0; len].into_boxed_slice();
        let mut cursor: Cursor<Box<[u8]>> = Cursor::new(buffer);
        self.write_rapified(&mut cursor)?;

        Ok(cursor)
    }

    /// Reads the rapified config from input.
    pub fn read_rapified<I: Read + Seek>(input: &mut I) -> Result<Config, ArmaLintError> {
        let mut reader = BufReader::new(input);

        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;

        if &buffer != b"\0raP" {
            return Err(ArmaLintError::InvalidInput(
                "File doesn't seem to be a rapified config.".to_string(),
            ));
        }

        Ok(Config {
            root: Class::read_rapified(&mut reader, 0)?,
        })
    }
}
