mod slotmap;
pub use slotmap::*;

mod lang;
pub use lang::*;

mod analysis;
pub use analysis::*;

use egg::*;

fn var(s: Slot, eg: &mut EGraph<Lambda, LambdaAnalysis>) -> Id { eg.add(Lambda::Var(s)) }
fn app(x: Id, y: Id, eg: &mut EGraph<Lambda, LambdaAnalysis>) -> Id { eg.add(Lambda::App([x, y])) }
fn lam(s: Slot, b: Id, eg: &mut EGraph<Lambda, LambdaAnalysis>) -> Id { eg.add(Lambda::Lam(s, b)) }
#[allow(unused)] fn rename(m: SlotMap, b: Id, eg: &mut EGraph<Lambda, LambdaAnalysis>) -> Id { eg.add(Lambda::Rename(m, b)) }
fn sym(s: &str, eg: &mut EGraph<Lambda, LambdaAnalysis>) -> Id { eg.add(Lambda::Sym(Symbol::new(s))) }

fn main() {
}

#[test]
fn test1() {
    let eg = &mut EGraph::new(LambdaAnalysis::default());

    let v1 = var(1, eg);
    let v2 = var(2, eg);

    let fsym = sym("f", eg);
    let gsym = sym("g", eg);

    let f1 = app(fsym, v1, eg);
    let f2 = app(fsym, v2, eg);
    let g1 = app(gsym, v1, eg);
    let g2 = app(gsym, v2, eg);

    eg.union(f1, g1);
    eg.rebuild();

    // f(1) = g(1) -> f(2) = g(2)
    assert_eq!(eg.find(f2), eg.find(g2));
}

#[test]
fn test2() {
    let eg = &mut EGraph::new(LambdaAnalysis::default());

    let v1 = var(1, eg);
    let v2 = var(2, eg);

    let l1v1 = lam(1, v1, eg);
    let l2v2 = lam(2, v2, eg);

    assert_eq!(eg.find(l1v1), eg.find(l2v2));
}
