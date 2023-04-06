use crate::lson::Lson;

pub trait LibrettoEventListener {
    fn on_event(&mut self, event_id: &str, data: Vec<Lson>);
}
