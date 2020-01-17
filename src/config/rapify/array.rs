use std::io::{Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::ArrayElement;
use crate::io::{ReadExt, WriteExt};
use crate::ArmaLintError;

#[derive(Debug)]
pub struct Array {
    pub expand: bool,
    pub elements: Vec<ArrayElement>,
}

impl Array {
    pub fn write_rapified<O: Write>(&self, output: &mut O) -> Result<usize, ArmaLintError> {
        let mut written = output.write_compressed_int(self.elements.len() as u32)?;

        for element in &self.elements {
            match element {
                ArrayElement::Str(s) => {
                    output.write_all(&[0])?;
                    output.write_cstring(s)?;
                    written += s.len() + 2;
                }
                ArrayElement::Float(f) => {
                    output.write_all(&[1])?;
                    output.write_f32::<LittleEndian>(*f)?;
                    written += 5;
                }
                ArrayElement::Int(i) => {
                    output.write_all(&[2])?;
                    output.write_i32::<LittleEndian>(*i)?;
                    written += 5;
                }
                ArrayElement::Array(a) => {
                    output.write_all(&[3])?;
                    written += 1 + a.write_rapified(output)?;
                }
            }
        }

        Ok(written)
    }

    pub fn read_rapified<I: Read + Seek>(input: &mut I) -> Result<Array, ArmaLintError> {
        let num_elements: u32 = input.read_compressed_int()?;
        let mut elements: Vec<ArrayElement> = Vec::with_capacity(num_elements as usize);

        for _i in 0..num_elements {
            let element_type: u8 = input.bytes().next().unwrap()?;

            if element_type == 0 {
                elements.push(ArrayElement::Str(input.read_cstring()?));
            } else if element_type == 1 {
                elements.push(ArrayElement::Float(input.read_f32::<LittleEndian>()?));
            } else if element_type == 2 {
                elements.push(ArrayElement::Int(input.read_i32::<LittleEndian>()?));
            } else if element_type == 3 {
                elements.push(ArrayElement::Array(Array::read_rapified(input)?));
            } else {
                return Err(ArmaLintError::InvalidInput(format!(
                    "Unrecognized array element type: {}",
                    element_type
                )));
            }
        }

        Ok(Array { expand: false, elements })
    }
}
