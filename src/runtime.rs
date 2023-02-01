mod data;
mod event;
mod function;

use std::collections::HashMap;
use crate::logic::lson::Lson;

use self::{data::LibrettoDataManager, event::LibrettoEventListener};

pub struct LibrettoRuntime {
    data : Box<dyn LibrettoDataManager>,
    event_listeners : Vec<Box<dyn LibrettoEventListener>>
}