mod event;
pub mod function;

use crate::lson::Lson;
use crate::scope::LibrettoScope;
use std::collections::HashMap;

use self::{event::LibrettoEventListener};

pub struct LibrettoRuntime {
    current_scope: LibrettoScope<Lson>,
    event_listeners: Vec<Box<dyn LibrettoEventListener>>,
}

impl Default for LibrettoRuntime {
    fn default() -> Self {
        LibrettoRuntime {
            current_scope : LibrettoScope {data : HashMap::new(), parrent: None},
            event_listeners : Vec::new()
        }
    }
}

impl LibrettoRuntime {

    pub fn with_data(data : impl Into<HashMap<String, Lson>>) -> Self {
        LibrettoRuntime {
            current_scope: LibrettoScope { data: data.into(), parrent: None },
            event_listeners: Vec::new()
        }
    }

    pub fn get_data(&self, key : &str) -> Lson {
        self.current_scope.get_data(key)
    }

    pub fn push_scope(&mut self, data : impl Into<HashMap<String, Lson>>) {
        let next_scope = LibrettoScope::new(data);
        let last_scope = std::mem::replace(&mut self.current_scope, next_scope);
        self.current_scope.parrent = Some(Box::new(last_scope))
    }

    pub fn pop_scope(&mut self) {
        if let Some(parrent) = std::mem::replace(&mut self.current_scope.parrent, None) {
            self.current_scope = *parrent;
        }
    }

    pub fn insert_data(&mut self, ident : &str, value : Lson) {
        self.current_scope.data.insert(ident.to_string(), value);
    }
}

pub trait LibrettoEvaluator {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> LibrettoRuntimeResult;
}

#[derive(thiserror::Error, Debug)]
pub enum LibrettoRuntimeError {
    #[error("There is no assignable variable with the identifier '{0}'")]
    VariableNotDefined(String),
}

pub type LibrettoRuntimeResult = Result<Lson, LibrettoRuntimeError>;

#[cfg(test)]
mod tests {

    use crate::{
        lson::{Lson, LsonType},
    };

    use super::*;

    #[test]
    fn add_scope_to_runtime() {
        let mut runtime = LibrettoRuntime::default();
        assert_eq!(runtime.current_scope.depth(), 1);
        runtime.push_scope([]);
        assert_eq!(runtime.current_scope.depth(), 2);
        runtime.pop_scope();
        assert_eq!(runtime.current_scope.depth(), 1);
    }

    #[test]
    fn get_data_from_runtime() {
        let mut runtime = LibrettoRuntime::with_data([("foo".to_string(), Lson::Bool(true))]);
        assert!(runtime.get_data("foo").as_bool().unwrap());
        runtime.push_scope([]);
        assert!(runtime.get_data("foo").as_bool().unwrap());
    }
}