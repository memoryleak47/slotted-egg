use crate::*;

struct LambdaAnalysis {
    todo_unions: Vec<((SlotMap, Id), (SlotMap, Id))>,
}

#[derive(Debug, Clone, Default)]
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
        DidMerge(false, true) // TODO correct?
    }

    fn modify(eg: &mut EGraph<Lambda, Self>, eclass: Id) {
        // TODO add corresponding Rename nodes.

        for (l, r) in std::mem::take(&mut eg.analysis.todo_unions) {
            let (m1, l) = find(l, eg);
            let (m2, r) = find(r, eg);
            if l == r {
                // m1*l = m2*l
                // l = m1⁻¹*m2*l
                let m = m1.inverse().compose(&m2);
                if !eg[l].data.group.contains(&m) {
                    let mut data = std::mem::take(&mut eg[l].data);
                    data.group.insert(m);
                    complete_group(&mut data.group);
                    eg.set_analysis_data(l, data);
                }
            } else {
                // m1*l = m2*r
                // l -> m1⁻¹*m2*r
                let m = m1.inverse().compose(&m2);
                let mut data = std::mem::take(&mut eg[l].data);
                data.leader = (m.clone(), r);
                eg.set_analysis_data(l, data);

                // TODO move over symmetries & redundancies.
            }
        }
    }
}

fn find((mut m, mut x): (SlotMap, Id), eg: &EGraph<Lambda, LambdaAnalysis>) -> (SlotMap, Id) {
    loop {
        let (m2, y) = &eg[x].data.leader;
        if x == *y { return (m, x) }
        (m, x) = (m.compose(m2), *y);
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
