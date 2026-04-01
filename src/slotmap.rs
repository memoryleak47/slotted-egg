use std::ops::Index;
pub use std::collections::BTreeSet;

pub type Slot = usize;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone)]
pub struct SlotMap(Vec<(Slot, Slot)>);

impl SlotMap {
    pub fn identity(slots: &BTreeSet<Slot>) -> Self {
        SlotMap(slots.iter().map(|x| (*x, *x)).collect())
    }

    pub fn compose(&self, other: &SlotMap) -> SlotMap {
        todo!()
    }
}

impl Index<Slot> for SlotMap {
    type Output = Slot;

    fn index(&self, i: Slot) -> &Self::Output {
        &self.0.iter().find(|(x, y)| *x == i).unwrap().1
    }
}
