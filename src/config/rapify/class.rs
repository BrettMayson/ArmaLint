use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::iter::Sum;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::{Array, Entry};
use crate::io::*;
use crate::ArmaLintError;

#[derive(Debug)]
pub struct Class {
    pub parent: String,
    pub external: bool,
    pub deletion: bool,
    pub entries: Vec<(String, Entry)>,
}

impl Class {
    pub fn rapified_length(&self) -> usize {
        if !self.entries.is_empty() {
            self.parent.len()
                + 1
                + compressed_int_len(self.entries.len() as u32)
                + usize::sum(self.entries.iter().map(|(k, v)| {
                    k.len()
                        + 1
                        + v.rapified_length()
                        + match v {
                            Entry::Class(c) => c.rapified_length(),
                            _ => 0,
                        }
                }))
        } else {
            0
        }
    }

    pub fn write_rapified<O: Write>(&self, output: &mut O, offset: usize) -> Result<usize, ArmaLintError> {
        let mut written = 0;

        if !self.entries.is_empty() {
            output.write_cstring(&self.parent)?;
            written += self.parent.len() + 1;

            written += output.write_compressed_int(self.entries.len() as u32)?;

            let entries_len = usize::sum(self.entries.iter().map(|(k, v)| k.len() + 1 + v.rapified_length()));
            let mut class_offset = offset + written + entries_len;
            let mut class_bodies: Vec<Cursor<Box<[u8]>>> = Vec::new();
            let pre_entries = written;

            for (name, entry) in &self.entries {
                let pre_write = written;
                match entry {
                    Entry::Str(s) => {
                        output.write_all(&[1, 0])?;
                        output.write_cstring(name)?;
                        output.write_cstring(s)?;
                        written += name.len() + s.len() + 4;
                    }
                    Entry::Float(f) => {
                        output.write_all(&[1, 1])?;
                        output.write_cstring(name)?;
                        output.write_f32::<LittleEndian>(*f)?;
                        written += name.len() + 7;
                    }
                    Entry::Int(i) => {
                        output.write_all(&[1, 2])?;
                        output.write_cstring(name)?;
                        output.write_i32::<LittleEndian>(*i)?;
                        written += name.len() + 7;
                    }
                    Entry::Array(a) => {
                        output.write_all(if a.expand { &[5] } else { &[2] })?;
                        if a.expand {
                            output.write_all(&[1, 0, 0, 0])?;
                            written += 4;
                        }
                        output.write_cstring(name)?;
                        written += name.len() + 2 + a.write_rapified(output)?;
                    }
                    Entry::Class(c) => {
                        if c.external || c.deletion {
                            output.write_all(if c.deletion { &[4] } else { &[3] })?;
                            output.write_cstring(name)?;
                            written += name.len() + 2;
                        } else {
                            output.write_all(&[0])?;
                            output.write_cstring(name)?;
                            output.write_u32::<LittleEndian>(class_offset as u32)?;
                            written += name.len() + 6;

                            let buffer: Box<[u8]> = vec![0; c.rapified_length()].into_boxed_slice();
                            let mut cursor: Cursor<Box<[u8]>> = Cursor::new(buffer);
                            class_offset += c.write_rapified(&mut cursor, class_offset)?;

                            class_bodies.push(cursor);
                        }
                    }
                }
                assert_eq!(written - pre_write, entry.rapified_length() + name.len() + 1);
            }

            assert_eq!(written - pre_entries, entries_len);

            for cursor in class_bodies {
                output.write_all(cursor.get_ref())?;
                written += cursor.get_ref().len();
            }
        }

        Ok(written)
    }

    pub fn read_rapified<I: Read + Seek>(input: &mut I, level: u32) -> Result<Class, ArmaLintError> {
        let mut fp = 0;
        if level == 0 {
            input.seek(SeekFrom::Start(16))?;
        } else {
            let classbody_fp: u32 = input.read_u32::<LittleEndian>()?;

            fp = input.seek(SeekFrom::Current(0))?;
            input.seek(SeekFrom::Start(classbody_fp.into()))?;
        }

        let parent = input.read_cstring()?;
        let num_entries: u32 = input.read_compressed_int()?;
        let mut entries: Vec<(String, Entry)> = Vec::with_capacity(num_entries as usize);

        for _i in 0..num_entries {
            let entry_type: u8 = input.bytes().next().unwrap()?;

            if entry_type == 0 {
                let name = input.read_cstring()?;

                let class_entry = Class::read_rapified(input, level + 1)?;
                entries.push((name, Entry::Class(class_entry)));
            } else if entry_type == 1 {
                let subtype: u8 = input.bytes().next().unwrap()?;
                let name = input.read_cstring()?;

                if subtype == 0 {
                    entries.push((name, Entry::Str(input.read_cstring()?)));
                } else if subtype == 1 {
                    entries.push((name, Entry::Float(input.read_f32::<LittleEndian>()?)));
                } else if subtype == 2 {
                    entries.push((name, Entry::Int(input.read_i32::<LittleEndian>()?)));
                } else {
                    return Err(ArmaLintError::InvalidInput(format!(
                        "Unrecognized variable entry subtype: {}.",
                        subtype
                    )));
                }
            } else if entry_type == 2 || entry_type == 5 {
                if entry_type == 5 {
                    input.seek(SeekFrom::Current(4))?;
                }

                let name = input.read_cstring()?;
                let mut array = Array::read_rapified(input)?;
                array.expand = entry_type == 5;

                entries.push((name.clone(), Entry::Array(array)));
            } else if entry_type == 3 || entry_type == 4 {
                let name = input.read_cstring()?;
                let class_entry = Class {
                    parent: String::from(""),
                    external: entry_type == 3,
                    deletion: entry_type == 5,
                    entries: Vec::new(),
                };

                entries.push((name.clone(), Entry::Class(class_entry)));
            } else {
                return Err(ArmaLintError::InvalidInput(format!(
                    "Unrecognized class entry type: {}.",
                    entry_type
                )));
            }
        }

        if level > 0 {
            input.seek(SeekFrom::Start(fp))?;
        }

        Ok(Class {
            parent,
            external: false,
            deletion: false,
            entries,
        })
    }
}
