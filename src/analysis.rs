use crate::*;

struct LambdaAnalysis;

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
        let (ma, la) = &a.leader;
        let (mb, lb) = &b.leader;
        if la == lb { // symmetries
            let l = la;
            // ma * l = mb * l
            // l = ma⁻¹ * mb * l

            // TODO how to do that? I have no e-graph access.
            // let mut data = [l].data.clone();
            // data.group.push(ma.inverse().compose(mb));
            // complete_group(&mut data.group);
            // egraph.set_analysis_data(l, data);
        } else {
            todo!()
        }
    }
}

fn complete_group(group: &mut BTreeSet<SlotMap>) {
    loop {
        let n = group.len();

        let mut extra = BTreeSet::new();

        for x in group {
            for y in group {
                extra.push(x.compose(y));
            }
        }
        group.extend(extra);

        if n == group.len() { break }
    }
}
