use strum::EnumDiscriminants;

use crate::runtime::LibrettoRuntime;
use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::{self},
    rc::Rc,
};

pub type LibrettoFunction = Rc<dyn Fn(Vec<Lson>, &mut LibrettoRuntime) -> Lson>;

#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(name(LsonType))]
pub enum Lson {
    None,
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Lson>),
    Struct(HashMap<String, Lson>),
    Function(LibrettoFunction),
}

impl Lson {
    pub fn is_i64(&self) -> bool {
        if let Lson::Int(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_f64(&self) -> bool {
        if let Lson::Float(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_bool(&self) -> bool {
        if let Lson::Bool(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let Lson::String(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_array(&self) -> bool {
        if let Lson::Array(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_struct(&self) -> bool {
        if let Lson::Struct(_) = self {
            true
        } else {
            false
        }
    }

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

    pub fn matches_type(&self, t : LsonType) -> bool {
        match self {
            Lson::None => LsonType::None == t,
            Lson::Int(_) => LsonType::Int == t,
            Lson::Float(_) => LsonType::Float == t,
            Lson::String(_) => LsonType::String == t,
            Lson::Bool(_) => LsonType::Bool == t,
            Lson::Array(_) => LsonType::Array == t,
            Lson::Struct(_) => LsonType::Struct == t,
            Lson::Function(_) => LsonType::Function == t,
        }
    }

    pub fn get_type(&self) -> LsonType {
        self.into()
    }
}

impl ToString for LsonType {
    fn to_string(&self) -> String {
        match self {
            LsonType::None => String::from("none"),
            LsonType::Int => String::from("int"),
            LsonType::Float => String::from("float"),
            LsonType::String => String::from("string"),
            LsonType::Bool => String::from("bool"),
            LsonType::Array => String::from("array"),
            LsonType::Struct => String::from("struct"),
            LsonType::Function => String::from("function"),
        }
    }
}

impl LsonType {
    pub fn get_sum_type(&self, other : LsonType) -> LsonType {
        match (self, other) {
            (LsonType::Float, LsonType::Float) | 
            (LsonType::Float, LsonType::Int) |
            (LsonType::Int, LsonType::Float) => LsonType::Float,
            (_, LsonType::String) |
            (LsonType::String, _) => LsonType::String,
            (LsonType::Int, LsonType::Int) => LsonType::Int,
            _ => LsonType::None
        }
    }

    pub fn get_difference_type(&self, other : LsonType) -> LsonType {
        match (self, other) {
            (LsonType::Float, LsonType::Float) | 
            (LsonType::Float, LsonType::Int) |
            (LsonType::Int, LsonType::Float) => LsonType::Float,
            (LsonType::Int, LsonType::Int) => LsonType::Int,
            _ => LsonType::None
        }
    }

    pub fn get_product_type(&self, other : LsonType) -> LsonType {
        match (self, other) {
            (LsonType::Float, LsonType::Float) | 
            (LsonType::Float, LsonType::Int) |
            (LsonType::Int, LsonType::Float) => LsonType::Float,
            (LsonType::Int, LsonType::Int) => LsonType::Int,
            _ => LsonType::None
        }
    }

    pub fn get_quotient_type(&self, other : LsonType) -> LsonType {
        match (self, other) {
            (LsonType::Float, LsonType::Float) | 
            (LsonType::Float, LsonType::Int) |
            (LsonType::Int, LsonType::Float) => LsonType::Float,
            (LsonType::Int, LsonType::Int) => LsonType::Int,
            _ => LsonType::None
        }
    }

    pub fn get_comparison_type(&self, other : LsonType) -> LsonType {
        match (self, other) {
            (LsonType::Int, LsonType::Float) |
            (LsonType::Float, LsonType::Int) => LsonType::Bool,
            _ => LsonType::None
        }
    }

    pub fn get_equality_type(&self, other : LsonType) -> LsonType {
        match (self, other) {
            (LsonType::Int, LsonType::Float) |
            (LsonType::Float, LsonType::Int) |
            (LsonType::Int, LsonType::Int) |
            (LsonType::Float, LsonType::Float) |
            (LsonType::String, LsonType::String) |
            (LsonType::Bool, LsonType::Bool) |
            (LsonType::Array, LsonType::Array) |
            (LsonType::Struct, LsonType::Struct) |
            (LsonType::Function, LsonType::Function) => LsonType::Bool,
            _ => LsonType::None
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

impl<T: Into<Lson>> From<Vec<T>> for Lson {
    fn from(value: Vec<T>) -> Self {
        let res: Vec<Lson> = value.into_iter().map(|value| value.into()).collect();
        Lson::Array(res)
    }
}

impl<T: Into<Lson>, const COUNT: usize> From<[T; COUNT]> for Lson {
    fn from(value: [T; COUNT]) -> Self {
        let res: Vec<Lson> = value.into_iter().map(|value| value.into()).collect();
        Lson::Array(res)
    }
}

impl<T: Into<Lson>> From<HashMap<String, T>> for Lson {
    fn from(value: HashMap<String, T>) -> Self {
        let res: HashMap<String, Lson> = value
            .into_iter()
            .map(|(key, value)| (key, value.into()))
            .collect();
        Lson::Struct(res)
    }
}

impl<T: Into<Lson>> From<HashMap<&str, T>> for Lson {
    fn from(value: HashMap<&str, T>) -> Self {
        let res: HashMap<String, Lson> = value
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.into()))
            .collect();
        Lson::Struct(res)
    }
}

impl PartialEq<&str> for Lson {
    fn eq(&self, other: &&str) -> bool {
        self.as_str().map_or(false, |s| s == *other)
    }
}

impl PartialEq<String> for Lson {
    fn eq(&self, other: &String) -> bool {
        self.as_string().map_or(false, |s| s == *other)
    }
}

impl PartialEq<i64> for Lson {
    fn eq(&self, other: &i64) -> bool {
        self.as_i64().map_or(false, |i| i == *other)
    }
}

impl PartialEq<i32> for Lson {
    fn eq(&self, other: &i32) -> bool {
        self.as_i64().map_or(false, |i| i == (*other) as i64)
    }
}

impl PartialEq<i16> for Lson {
    fn eq(&self, other: &i16) -> bool {
        self.as_i64().map_or(false, |i| i == (*other) as i64)
    }
}

impl PartialEq<i8> for Lson {
    fn eq(&self, other: &i8) -> bool {
        self.as_i64().map_or(false, |i| i == (*other) as i64)
    }
}

impl PartialEq<u64> for Lson {
    fn eq(&self, other: &u64) -> bool {
        self.as_i64().map_or(false, |i| i == (*other) as i64)
    }
}

impl PartialEq<u32> for Lson {
    fn eq(&self, other: &u32) -> bool {
        self.as_i64().map_or(false, |i| i == (*other) as i64)
    }
}

impl PartialEq<u16> for Lson {
    fn eq(&self, other: &u16) -> bool {
        self.as_i64().map_or(false, |i| i == (*other) as i64)
    }
}

impl PartialEq<u8> for Lson {
    fn eq(&self, other: &u8) -> bool {
        self.as_i64().map_or(false, |i| i == (*other) as i64)
    }
}

impl PartialEq<f64> for Lson {
    fn eq(&self, other: &f64) -> bool {
        self.as_f64().map_or(false, |f| f == (*other))
    }
}

impl PartialEq<f32> for Lson {
    fn eq(&self, other: &f32) -> bool {
        self.as_f64().map_or(false, |f| f == (*other) as f64)
    }
}

impl<T> ops::Index<T> for Lson
where
    T: LsonIndex,
{
    type Output = Lson;

    fn index(&self, index: T) -> &Lson {
        index.index_into(self).unwrap_or(&Lson::None)
    }
}

impl<T> ops::IndexMut<T> for Lson
where
    T: LsonIndex,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        index.index_or_insert(self)
    }
}

impl Debug for Lson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Int(arg0) => f.debug_tuple("Int").field(arg0).finish(),
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Bool(arg0) => f.debug_tuple("Bool").field(arg0).finish(),
            Self::Array(arg0) => f.debug_tuple("Array").field(arg0).finish(),
            Self::Struct(arg0) => f.debug_tuple("Struct").field(arg0).finish(),
            Self::Function(arg0) => f.debug_tuple("Function").field(&"()").finish(),
        }
    }
}

impl PartialEq for Lson {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Struct(l0), Self::Struct(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => false,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

//================================================================================================
//          Index
//================================================================================================

pub trait LsonIndex {
    fn index_into<'l>(&self, value: &'l Lson) -> Option<&'l Lson>;

    fn index_into_mut<'l>(&self, value: &'l mut Lson) -> Option<&'l mut Lson>;

    fn index_or_insert<'l>(&self, value: &'l mut Lson) -> &'l mut Lson;
}

impl LsonIndex for usize {
    fn index_into<'l>(&self, value: &'l Lson) -> Option<&'l Lson> {
        match value {
            Lson::Array(vec) => vec.get(*self),
            _ => None,
        }
    }

    fn index_into_mut<'l>(&self, value: &'l mut Lson) -> Option<&'l mut Lson> {
        match value {
            Lson::Array(vec) => vec.get_mut(*self),
            _ => None,
        }
    }

    fn index_or_insert<'l>(&self, value: &'l mut Lson) -> &'l mut Lson {
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
            _ => panic!("cannot access index {} of LSON {:?}", self, value.get_type()),
        }
    }
}

impl LsonIndex for str {
    fn index_into<'l>(&self, value: &'l Lson) -> Option<&'l Lson> {
        match value {
            Lson::Struct(vec) => vec.get(self),
            _ => None,
        }
    }

    fn index_into_mut<'l>(&self, value: &'l mut Lson) -> Option<&'l mut Lson> {
        match value {
            Lson::Struct(vec) => vec.get_mut(self),
            _ => None,
        }
    }

    fn index_or_insert<'l>(&self, value: &'l mut Lson) -> &'l mut Lson {
        if let Lson::None = value {
            *value = Lson::Struct(HashMap::new());
        }
        match value {
            Lson::Struct(map) => map.entry(self.to_owned()).or_insert(Lson::None),
            _ => panic!("cannot access key {:?} in JSON {:?}", self, value.get_type()),
        }
    }
}

impl LsonIndex for String {
    fn index_into<'l>(&self, value: &'l Lson) -> Option<&'l Lson> {
        self[..].index_into(value)
    }

    fn index_into_mut<'l>(&self, value: &'l mut Lson) -> Option<&'l mut Lson> {
        self[..].index_into_mut(value)
    }

    fn index_or_insert<'l>(&self, value: &'l mut Lson) -> &'l mut Lson {
        self[..].index_or_insert(value)
    }
}

impl<'a, T> LsonIndex for &'a T
where
    T: ?Sized + LsonIndex,
{
    fn index_into<'l>(&self, value: &'l Lson) -> Option<&'l Lson> {
        (**self).index_into(value)
    }

    fn index_into_mut<'l>(&self, value: &'l mut Lson) -> Option<&'l mut Lson> {
        (**self).index_into_mut(value)
    }

    fn index_or_insert<'l>(&self, value: &'l mut Lson) -> &'l mut Lson {
        (**self).index_or_insert(value)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Lson;

    #[test]
    fn from_int() {
        let i8_var: Lson = 1i8.into();
        let i16_var: Lson = 2i16.into();
        let i32_var: Lson = 3i32.into();
        let i64_var: Lson = 4i64.into();
        assert_eq!(i8_var.as_i64().unwrap(), 1);
        assert_eq!(i16_var.as_i64().unwrap(), 2);
        assert_eq!(i32_var.as_i64().unwrap(), 3);
        assert_eq!(i64_var.as_i64().unwrap(), 4);
    }

    #[test]
    fn from_array() {
        let array: Lson = ["This", "is", "a", "test"].into();
        println!("{:?}", array)
    }

    #[test]
    fn from_hash() {
        let array: Lson = HashMap::from([("test1", 30), ("test2", 30), ("test3", 30)]).into();
        println!("{:?}", array)
    }

    #[test]
    fn index_array() {
        let array: Lson = ["This", "is", "a", "test"].into();
        assert_eq!(array[2], "a");
    }

    #[test]
    fn index_struct() {
        let array: Lson = HashMap::from([("test1", 10), ("test2", 20), ("test3", 30)]).into();
        assert_eq!(array["test1"], 10);
    }
}
