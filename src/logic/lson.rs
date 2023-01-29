use std::{hash::Hash, collections::HashMap};

pub enum Lson {
  Int(i64),
  Float(f64),
  String(String),
  Bool(bool),
  Array(Vec<Lson>),
  Struct(HashMap<String, Lson>)
}

impl From<i64> for Lson {
    fn from(value: i64) -> Self {
        Lson::Int(value)
    }
}

impl From<i32> for Lson {
  fn from(value: i32) -> Self {
      Lson::Int(value as i64)
  }
}

impl From<i16> for Lson {
  fn from(value: i16) -> Self {
      Lson::Int(value as i64)
  }
}

impl From<i8> for Lson {
  fn from(value: i8) -> Self {
      Lson::Int(value as i64)
  }
}

impl From<f64> for Lson {
  fn from(value: f64) -> Self {
      Lson::Float(value)
  }
}

impl From<f32> for Lson {
  fn from(value: f32) -> Self {
      Lson::Float(value as f64)
  }
}

impl From<String> for Lson {
    fn from(value: String) -> Self {
        Lson::String(value)
    }
}

impl From<&str> for Lson {
  fn from(value: &str) -> Self {
      Lson::String(value.to_string())
  }
}

impl From<bool> for Lson {
    fn from(value: bool) -> Self {
        Lson::Bool(value)
    }
}

impl <T : Into<Lson>> From<Vec<T>> for Lson {
    fn from(value: Vec<T>) -> Self {
        let res : Vec<Lson> = value.into_iter().map(|value| value.into()).collect();
        Lson::Array(res)
    }
}

impl <T: Into<Lson>, const COUNT : usize> From<[T; COUNT]> for Lson {
    fn from(value: [T; COUNT]) -> Self {
        let res : Vec<Lson> = value.into_iter().map(|value| value.into()).collect();
        Lson::Array(res)
    }
}

impl <T: Into<Lson>> From<HashMap<String, T>> for Lson {
    fn from(value: HashMap<String, T>) -> Self {
        let res : HashMap<String, Lson> = value.into_iter().map(|(key, value)| (key, value.into())).collect();
        Lson::Struct(res)
    }
}