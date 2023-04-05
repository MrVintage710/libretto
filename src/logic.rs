use crate::runtime::LibrettoRuntime;
use self::lson::Lson;

pub mod lson;

//==================================================================================================
//          Evaluator
//==================================================================================================

pub trait LibrettoEvaluator {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> Lson;
}