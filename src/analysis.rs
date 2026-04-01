use crate::*;

struct LambdaAnalysis {
    todo_unions: Vec<((SlotMap, Id), (SlotMap, Id))>,
}

#[derive(Debug, Clone)]
struct LambdaData {
    slots: BTreeSet<Slot>,
    leader: (SlotMap, Id),
    group: BTreeSet<SlotMap>,
}

impl Analysis<Lambda> for LambdaAnalysis {
    type Data = LambdaData;

    fn make(eg: &mut EGraph<Lambda, Self>, n: &Lambda, x: Id) -> Self::Data {
        let slots = match n {
            Lambda::Lam(s, b) => eg[*b].data.slots.iter().copied().filter(|x| x != s).collect(),
            Lambda::App([a, b]) => &eg[*a].data.slots | &eg[*b].data.slots,
            Lambda::Var(s) => std::iter::once(*s).collect(),
            Lambda::Rename(m, a) => eg[*a].data.slots.iter().map(|x| m[*x]).collect(),
        };
        let identity = SlotMap::identity(&slots);
        LambdaData {
            slots,
            leader: (identity.clone(), x),
            group: std::iter::once(identity).collect(),
        }
    }

    fn merge(&mut self, a: &mut LambdaData, b: LambdaData) -> DidMerge {
        self.todo_unions.push((a.leader.clone(), b.leader));
        DidMerge(false, true) // TODD correct?
    }

    fn modify(egraph: &mut EGraph<Lambda, Self>, eclass: Id) {
        // TODO add corresponding Rename nodes.

        for (l, r) in std::mem::take(&mut egraph.analysis.todo_unions) {
            todo!();
        }
    }
}

fn complete_group(group: &mut BTreeSet<SlotMap>) {
    loop {
        let n = group.len();

        let mut extra = BTreeSet::new();

        for x in group.iter() {
            for y in group.iter() {
                extra.insert(x.compose(y));
            }
        }
        group.extend(extra);

        if n == group.len() { break }
    }
}
