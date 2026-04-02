use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lambda {
    Lam(Slot, Id),
    App([Id; 2]),
    Var(Slot),
    Rename(SlotMap, Id),
    Sym(Symbol),
}

impl Language for Lambda {
    type Discriminant = std::mem::Discriminant<Lambda>;

    fn discriminant(&self) -> <Self as egg::Language>::Discriminant { std::mem::discriminant(self) }

    fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Lambda::Lam(x, _), Lambda::Lam(y, _)) => x == y,
            (Lambda::App(_), Lambda::App(_)) => true,
            (Lambda::Var(x), Lambda::Var(y)) => x == y,
            (Lambda::Rename(m1, _), Lambda::Rename(m2, _)) => m1 == m2,
            (Lambda::Sym(s1), Lambda::Sym(s2)) => s1 == s2,
            _ => false,
        }
    }
     
    fn children(&self) -> &[egg::Id] {
        match self {
            Lambda::Lam(_, b) => std::slice::from_ref(b),
            Lambda::App(ab) => ab,
            Lambda::Var(_) => &[],
            Lambda::Rename(_, a) => std::slice::from_ref(a),
            Lambda::Sym(_) => &[],
        }
    }

    fn children_mut(&mut self) -> &mut [egg::Id] {
        match self {
            Lambda::Lam(_, b) => std::slice::from_mut(b),
            Lambda::App(ab) => ab,
            Lambda::Var(_) => &mut [],
            Lambda::Rename(_, a) => std::slice::from_mut(a),
            Lambda::Sym(_) => &mut [],
        }
    }
}
