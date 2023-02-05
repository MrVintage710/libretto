use super::{data::LibrettoDataManager, LibrettoRuntime};
use crate::logic::lson::Lson;

pub type LibrettoFunction = &'static dyn Fn(Vec<Lson>, &mut LibrettoRuntime) -> Lson;
