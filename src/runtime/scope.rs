use std::collections::HashMap;
use crate::lson::Lson;

pub struct LibrettoScope {
    parrent_scope : Option<Box<LibrettoScope>>,
    scope_data : HashMap<String, Lson>
}

impl LibrettoScope {
    pub fn push_scope(self, data : impl Into<HashMap<String, Lson>>) -> LibrettoScope {
        LibrettoScope { parrent_scope: Some(Box::new(self)), scope_data: data.into() }
    }

    pub fn pop_scope(self) -> LibrettoScope {
        if let Some(parent) = self.parrent_scope {
            *parent
        } else {
            self
        }
    }

    pub fn get_data(&self, key : &str) -> &Lson {
        if self.scope_data.contains_key(key) {
            self.scope_data.get(key).unwrap()
        } else {
            if let Some(parent) = self.parrent_scope {
                parent.get_data(key)
            } else {
                &Lson::None
            }
        }
    }

    pub fn get_data_mut(&mut self, key : &str) -> &mut Lson {
        if self.scope_data.contains_key(key) {
            self.scope_data.get_mut(key).unwrap()
        } else {
            if let Some(parent) = self.parrent_scope {
                parent.get_data_mut(key)
            } else {
                &mut Lson::None
            }
        }
    }

    pub fn insert(&mut self, indent : &str, value : Lson) {
        self.scope_data.insert(indent.to_string(), value);
    }
}
