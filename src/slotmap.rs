pub type Slot = usize;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone)]
pub struct SlotMap(Vec<(Slot, Slot)>);
