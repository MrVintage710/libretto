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

    pub fn depth(&self) -> u32 {
        if let Some(parrent) = &self.parrent {
            parrent.depth() + 1
        } else {
            1
        }
    }
}