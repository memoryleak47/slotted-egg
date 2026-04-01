use std::ops::Index;
pub use std::collections::BTreeSet;

pub type Slot = usize;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone, Default)]
pub struct SlotMap(Vec<(Slot, Slot)>);

impl SlotMap {
    pub fn identity(slots: &BTreeSet<Slot>) -> Self {
        SlotMap(slots.iter().map(|x| (*x, *x)).collect())
    }

    pub fn mk(iter: impl Iterator<Item=(Slot, Slot)>) -> SlotMap {
        let mut v: Vec<_> = iter.collect();
        v.sort_by_key(|(x, _)| *x);

        SlotMap(v)
    }

    // m1*m2[x] = m1[m2[x]]
    pub fn compose(&self, m2: &SlotMap) -> SlotMap {
        let m1 = self;

        SlotMap::mk(m2.0.iter().map(|(k, v)| (*k, m1[*v])))
    }

    pub fn inverse(&self) -> SlotMap {
        SlotMap::mk(self.0.iter().map(|(x, y)| (*y, *x)))
    }
}

impl Index<Slot> for SlotMap {
    type Output = Slot;

    fn index(&self, i: Slot) -> &Self::Output {
        // TODO binary search.
        &self.0.iter().find(|(x, y)| *x == i).unwrap().1
    }
}
