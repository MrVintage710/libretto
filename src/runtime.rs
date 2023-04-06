mod scope;
mod event;
pub mod function;

use crate::{lson::Lson, parse::LibrettoEvaluator};
use std::collections::HashMap;

use self::{event::LibrettoEventListener, scope::LibrettoScope};

pub struct LibrettoRuntime {
    ast : Box<dyn LibrettoEvaluator>,
    current_scope: LibrettoScope,
    event_listeners: Vec<Box<dyn LibrettoEventListener>>,
}

impl LibrettoRuntime {
    pub fn get_data(&self, key : &str) -> &Lson {
        self.current_scope.get_data(key)
    }

    pub fn get_data_mut(&mut self, key : &str) -> &mut Lson {
        self.current_scope.get_data_mut(key)
    }

    pub fn push_scope(&mut self, data : impl Into<HashMap<String, Lson>>) {
        self.current_scope = self.current_scope.push_scope(data);
    }

    pub fn pop_scope(&mut self) {
        self.current_scope = self.current_scope.pop_scope()
    }

    pub fn insert_data(&mut self, indent : &str, value : Lson) {
        self.current_scope.insert(indent, value)
    }
}