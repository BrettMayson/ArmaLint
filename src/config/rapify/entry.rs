use std::iter::Sum;

use super::{Array, Class};
use crate::io::compressed_int_len;

#[derive(Debug)]
pub enum Entry {
    Str(String),
    Float(f32),
    Int(i32),
    Array(Array),
    Class(Class),
}

impl Entry {
    pub fn rapified_length(&self) -> usize {
        match self {
            Self::Str(s) => s.len() + 3,
            Self::Float(_f) => 6,
            Self::Int(_i) => 6,
            Self::Array(a) => {
                let len = 1
                    + compressed_int_len(a.elements.len() as u32)
                    + usize::sum(a.elements.iter().map(|e| e.rapified_length()));
                if a.expand {
                    len + 4
                } else {
                    len
                }
            }
            Self::Class(c) => {
                if c.external || c.deletion {
                    1
                } else {
                    5
                }
            }
        }
    }
}
