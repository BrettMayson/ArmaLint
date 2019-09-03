use crate::ast::Node;

pub enum Type {
    _Array(Vec<Type>),
    _String(String),
    _Number(f32),
    _Boolean(bool),
    _Code(Node),
    _Side(ArmaSide),
}

pub enum ArmaSide {
    Blue,
    Red,
    Green,
    Purple,
    Unknown,
    Logic,
    Empty,
    AmbientLife,
}
