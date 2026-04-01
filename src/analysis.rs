use crate::*;

struct LambdaAnalysis;
#[derive(Debug)]
struct LambdaData;

impl Analysis<Lambda> for LambdaAnalysis {
    type Data = LambdaData;
    fn make(eg: &mut EGraph<Lambda, Self>, n: &Lambda, x: Id) -> Self::Data { todo!() }
    fn merge(&mut self, a: &mut LambdaData, b: LambdaData) -> DidMerge { todo!() }
}



