#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    Air,
    Grass,
    Dirt,
    Stone,
}

#[derive(Debug, Clone, Copy)]
pub enum Layer {
    Ground,
    Object,
}

#[derive(Debug, Clone, Copy)]
pub enum BlockBehavior {
    None,
    Solid,
    Chest,
}
