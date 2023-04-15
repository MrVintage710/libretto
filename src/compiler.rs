use std::collections::HashMap;

use crate::scope::LibrettoScope;
use crate::lson::{Lson, LsonType};

pub struct LibrettoCompiletime {
    current_scope: LibrettoScope<LsonType>
}

impl Default for LibrettoCompiletime {
    fn default() -> Self {
        LibrettoCompiletime {
            current_scope : LibrettoScope {data : HashMap::new(), parrent: None},
        }
    }
}

impl LibrettoCompiletime {

    pub fn with_data(data : impl Into<HashMap<String, LsonType>>) -> Self {
        LibrettoCompiletime {
            current_scope: LibrettoScope { data: data.into(), parrent: None },
        }
    }

    pub fn get_data(&self, key : &str) -> LsonType {
        self.current_scope.get_data(key)
    }
    
    pub fn push_scope(&mut self, data : impl Into<HashMap<String, LsonType>>) {
        let next_scope = LibrettoScope::new(data);
        let last_scope = std::mem::replace(&mut self.current_scope, next_scope);
        self.current_scope.parrent = Some(Box::new(last_scope))
    }

    pub fn pop_scope(&mut self) {
        if let Some(parrent) = std::mem::replace(&mut self.current_scope.parrent, None) {
            self.current_scope = *parrent;
        }
    }

    pub fn insert_data(&mut self, ident : &str, value : LsonType) {
        self.current_scope.data.insert(ident.to_string(), value);
    }
}