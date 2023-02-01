use std::collections::HashMap;

use crate::logic::lson::Lson;

pub trait LibrettoDataManager {
    fn on_mutate(&self, identifier : &str, old : Lson, new : Lson) -> Lson;

    fn on_declare(&self, identifier : &str, value : Lson);

    fn on_request(&self, identifier : &str) -> Lson;
}

pub struct DefaultDataManager {
    global_data : HashMap<String, Lson>
}

impl LibrettoDataManager for DefaultDataManager {
    fn on_mutate(&self, identifier : &str, old : Lson, new : Lson) -> Lson {
        todo!()
    }

    fn on_declare(&self, identifier : &str, value : Lson) {
        todo!()
    }

    fn on_request(&self, identifier : &str) -> Lson {
        todo!()
    }
}