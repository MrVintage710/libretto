use core::fmt;
use std::{collections::HashMap, ops::{Index, self}, fmt::Display};

#[derive(Debug)]
pub enum Lson {
  None,
  Int(i64),
  Float(f64),
  String(String),
  Bool(bool),
  Array(Vec<Lson>),
  Struct(HashMap<String, Lson>)
}

impl Lson {

  pub fn as_i64(&self) -> Option<i64> {
    if let Lson::Int(value) = self {
      Some(value.clone())
    } else {
      None
    }
  }

  pub fn as_f64(&self) -> Option<f64> {
    if let Lson::Float(value) = self {
      Some(value.clone())
    } else {
      None
    }
  }

  pub fn as_bool(&self) -> Option<bool> {
    if let Lson::Bool(value) = self {
      Some(value.clone())
    } else {
      None
    }
  }

  pub fn as_string(&self) -> Option<String> {
    if let Lson::String(value) = self {
      Some(value.clone())
    } else {
      None
    }
  }

  pub fn as_str(&self) -> Option<&str> {
    if let Lson::String(value) = self {
      Some(value.as_str())
    } else {
      None
    }
  }
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

impl <T: Into<Lson>> From<HashMap<&str, T>> for Lson {
  fn from(value: HashMap<&str, T>) -> Self {
      let res : HashMap<String, Lson> = value.into_iter().map(|(key, value)| (key.to_string(), value.into())).collect();
      Lson::Struct(res)
  }
}

impl PartialEq<&str> for Lson {
    fn eq(&self, other: &&str) -> bool {
        self.as_str().map_or(false, |i| i == *other)
    }
}

impl <T> ops::Index<T> for Lson where T : LsonIndex {
  type Output = Lson;

  fn index(&self, index: T) -> &Lson {
    static NULL: Lson = Lson::None;
    index.index_into(self).unwrap_or(&NULL)
  }
}

impl <T> ops::IndexMut<T> for Lson where T : LsonIndex {
  fn index_mut(&mut self, index: T) -> &mut Self::Output {
    index.index_or_insert(self)
  }
}

//================================================================================================
//          Index
//================================================================================================

pub trait LsonIndex {
  fn index_into<'l>(&self, value : &'l Lson) -> Option<&'l Lson>;
  
  fn index_into_mut<'l>(&self, value : &'l mut Lson) -> Option<&'l mut Lson>;

  fn index_or_insert<'l>(&self, value : &'l mut Lson) -> &'l mut Lson;
}

impl LsonIndex for usize {
    fn index_into<'l>(&self, value : &'l Lson) -> Option<&'l Lson> {
        match value {
          Lson::Array(vec) => vec.get(*self),
          _ => None
        }
    }

    fn index_into_mut<'l>(&self, value : &'l mut Lson) -> Option<&'l mut Lson> {
      match value {
        Lson::Array(vec) => vec.get_mut(*self),
        _ => None
      }
    }

    fn index_or_insert<'l>(&self, value : &'l mut Lson) -> &'l mut Lson {
      match value {
        Lson::Array(vec) => {
          let len = vec.len();
          vec.get_mut(*self).unwrap_or_else(|| {
            panic!(
              "cannot access index {} of LSON array of length {}",
              self, len
            )
          })
        }
        _ => panic!("cannot access index {} of LSON {}", self, Type(value))
      }
    }
}

/// Used in panic messages.
struct Type<'a>(&'a Lson);

impl<'a> Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
          Lson::None => formatter.write_str("undefined"),
          Lson::Bool(_) => formatter.write_str("boolean"),
          Lson::Int(_) => formatter.write_str("int"),
          Lson::Float(_) => formatter.write_str("float"),
          Lson::String(_) => formatter.write_str("string"),
          Lson::Array(_) => formatter.write_str("array"),
          Lson::Struct(_) => formatter.write_str("struct"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Lson;

    #[test]
    fn from_int() {
        let i8_var : Lson = 1i8.into();
        let i16_var : Lson = 2i16.into();
        let i32_var : Lson = 3i32.into();
        let i64_var : Lson = 4i64.into();
        assert_eq!(i8_var.as_i64().unwrap() , 1);
        assert_eq!(i16_var.as_i64().unwrap() , 2);
        assert_eq!(i32_var.as_i64().unwrap() , 3);
        assert_eq!(i64_var.as_i64().unwrap() , 4);
    }

    #[test]
    fn from_array() {
      let array : Lson = ["This", "is", "a", "test"].into();
      println!("{:?}", array)
    }

    #[test]
    fn from_hash() {
      let array : Lson = HashMap::from([
        ("test1", 30),
        ("test2", 30),
        ("test3", 30),
      ]).into();
      println!("{:?}", array)
    }

    #[test]
    fn index_array() {
      let array : Lson = ["This", "is", "a", "test"].into();
      assert_eq!(array[2], "a");
    }
}