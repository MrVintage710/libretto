use crate::logic::lson::Lson;
use super::{data::LibrettoDataManager, LibrettoRuntime};

pub type LibrettoFunction = &'static dyn Fn(Vec<Lson>, &mut LibrettoRuntime) -> Lson;