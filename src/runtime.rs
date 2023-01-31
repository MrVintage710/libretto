mod data;
mod event;

use std::collections::HashMap;
use crate::logic::lson::Lson;

use self::{data::LibrettoDataManager, event::LibrettoEventListener};

pub struct LibrettoRuntime<D> where D : LibrettoDataManager {
    data : D,
    event_listeners : Vec<Box<dyn LibrettoEventListener>>
}