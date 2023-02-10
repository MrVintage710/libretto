mod data;
mod event;
pub mod function;

use crate::logic::lson::Lson;
use std::collections::HashMap;

use self::{data::LibrettoDataManager, event::LibrettoEventListener};

pub struct LibrettoRuntime {
    data: Box<dyn LibrettoDataManager>,
    event_listeners: Vec<Box<dyn LibrettoEventListener>>,
}
