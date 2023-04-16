use std::collections::HashMap;

use crate::scope::LibrettoScope;
use crate::lson::{Lson, LsonType};

pub struct LibrettoCompiletime {
    current_scope: LibrettoScope<LsonType>,
    errors : Vec<LibrettoCompileError>
}

impl Default for LibrettoCompiletime {
    fn default() -> Self {
        LibrettoCompiletime {
            current_scope : LibrettoScope {data : HashMap::new(), parrent: None},
            errors : Vec::new()
        }
    }
}

impl LibrettoCompiletime {

    pub fn with_data(data : impl Into<HashMap<String, LsonType>>) -> Self {
        LibrettoCompiletime {
            current_scope: LibrettoScope { data: data.into(), parrent: None },
            errors : Vec::new()
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

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn push_error(&mut self, value: LibrettoCompileError) {
        self.errors.push(value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LibrettoCompileError {
    #[error("Values are not allowed to be set to null.")]
    NullValueError,

    #[error("The operator {0} is not supported for type {1}")]
    OperationNotSupportedError(String, String),

    #[error("The operation {0} is not supported for types {1} and {2}")]
    InvalidOperationError(String, String, String),

    #[error("When parsing '{0}', the pre parse check passed event though the pattern doesn't match.")]
    ParseCheckNotThoroughError(String),

    #[error("When parsing an expression with type {0}, there was a default supplied with type {1}. These types must be the same.")]
    ExprDefaultTypeMissmatch(String, String),

    #[error("Cannot assign value to variable {0} because the variable is not the same type.")]
    AssignmentWithInvalidType(String),
    
    #[error("Cannot assign value to undeclared variable '{0}'.")]
    AssignmentWithUndeclaredVariable(String),
}