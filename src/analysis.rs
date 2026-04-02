use crate::*;

#[derive(Default)]
pub struct LambdaAnalysis {
    todo_unions: Vec<((SlotMap, Id), (SlotMap, Id))>,
}

#[derive(Debug, Clone, Default)]
pub struct LambdaData {
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
            Lambda::Sym(_) => std::iter::empty().collect(),
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
            let m1_inv = m1.inverse();
            let m2_inv = m2.inverse();

            let common_slots = &m1.value_set() & &m2.value_set();
            let l_slots = common_slots.iter().filter_map(|a| m1_inv.get(*a)).collect();
            let r_slots = common_slots.iter().filter_map(|a| m2_inv.get(*a)).collect();
            mark_slot_redundant(eg, l, l_slots);
            mark_slot_redundant(eg, r, r_slots);

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

                // update l data, to point to r.
                let mut data = std::mem::take(&mut eg[l].data);
                data.leader = (m.clone(), r);
                let l_group = data.group.clone();
                eg.set_analysis_data(l, data);

                // update r data, the obtained symmetries.
                let mut data = std::mem::take(&mut eg[r].data);

                for g in l_group {
                    // g*l = l  /\ l = m*r
                    // g*m*r = m*r
                    // -> m⁻¹*g*m*r = r
                    let mm = m.inverse().compose(&g).compose(&m);
                    data.group.insert(mm);
                }
                complete_group(&mut data.group);
                eg.set_analysis_data(r, data);
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

fn mark_slot_redundant(eg: &mut EGraph<Lambda, LambdaAnalysis>, i: Id, new_set: BTreeSet<Slot>) {
    let mut data = std::mem::take(&mut eg[i].data);

    // TODO respect orbits.

    assert!(new_set.is_subset(&data.slots));
    data.slots = new_set;

    // TODO shrink group.

    eg.set_analysis_data(i, data);
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
