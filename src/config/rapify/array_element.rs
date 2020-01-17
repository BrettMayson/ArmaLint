use std::iter::Sum;

use super::Array;
use crate::io::compressed_int_len;

#[derive(Debug)]
pub enum ArrayElement {
    /// String element
    Str(String),
    /// Float element
    Float(f32),
    /// Int element
    Int(i32),
    /// Array element
    Array(Array),
}

impl ArrayElement {
    pub fn rapified_length(&self) -> usize {
        match self {
            ArrayElement::Str(s) => s.len() + 2,
            ArrayElement::Float(_f) => 5,
            ArrayElement::Int(_i) => 5,
            ArrayElement::Array(a) => {
                1 + compressed_int_len(a.elements.len() as u32) + usize::sum(a.elements.iter().map(|e| e.rapified_length()))
            }
        }
    }
}
