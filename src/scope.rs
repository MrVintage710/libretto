use std::collections::HashMap;

pub(crate) struct LibrettoScope<T> {
    pub data : HashMap<String, T>,
    pub parrent : Option<Box<LibrettoScope<T>>>
}

impl <T> LibrettoScope<T> where T: Clone + Default {
    pub fn new(data : impl Into<HashMap<String, T>> ) -> Self {
        LibrettoScope { data: data.into(), parrent: None }
    }

    pub fn get_data(&self, ident : &str) -> T {
        if self.data.contains_key(ident) {
            return self.data.get(ident).unwrap().clone();
        }

        if self.parrent.is_some() {
            return self.parrent.as_ref().unwrap().get_data(ident);
        }

        T::default()
    }

    pub fn replace_data(&mut self, ident : &str, value : T) -> bool {
        if self.data.contains_key(ident) {
            self.data.insert(ident.to_string(), value);
            return true;
        }

        if self.parrent.is_some() {
            return self.parrent.as_mut().unwrap().replace_data(ident, value);
        }

        false
    }

    pub fn has_data(&self, ident : &str) -> bool {
        if self.data.contains_key(ident) {
            return true;
        }

        if self.parrent.is_some() {
            return self.parrent.as_ref().unwrap().has_data(ident);
        }

        false
    }

    pub fn data_depth(&self, ident : &str) -> i32 {
        self.check_depth(ident, 0)
    }

    fn check_depth(&self, ident : &str, current_depth : i32) -> i32 {
        if self.data.contains_key(ident) {
            return current_depth;
        }

        if self.parrent.is_some() {
            return self.parrent.as_ref().unwrap().check_depth(ident, current_depth + 1);
        }

        -1
    }

    pub fn depth(&self) -> u32 {
        if let Some(parrent) = &self.parrent {
            parrent.depth() + 1
        } else {
            1
        }
    }
}