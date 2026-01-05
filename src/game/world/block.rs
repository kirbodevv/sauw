#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u16);

#[derive(Debug, Clone, Copy)]
pub enum BlockBehavior {
    None,
    Solid,
}
