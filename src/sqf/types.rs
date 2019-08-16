use crate::ast::Node;

pub enum Types {
    _Array(Vec<Node>),
    _String(String),
    _Number(f32),
    _Boolean(bool),
    _Code(Node),
    _Side(ArmaSide)
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
