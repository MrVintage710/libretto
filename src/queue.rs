use std::collections::VecDeque;
use std::collections::vec_deque::Iter;
use std::marker::PhantomData;
use std::ops::RangeBounds;
use std::{
    fmt::{Debug, Display},
};
use logos::Logos;

use crate::lexer::{Ordinal, OrdinalGroup};
use crate::parse::{Checkable, NullParsable, Parsable, CheckResult};

pub struct TokenQueue<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug
{
    tokens : VecDeque<T>,
    cursor : usize,
    mark : usize,
    removed : Vec<(T, usize)>,
    _phantom: &'a PhantomData<T>
}

impl <'a, T> TokenQueue<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug
{
    pub fn is<C>(&mut self) -> CheckResult where C: Checkable<'a, T>{
        if let Some(result) = C::check(self) {
            
        } else {
            Box::new(NullParsable{})
        }
    }
    
    pub fn zero_or_one(&mut self, ) -> CheckResult<'a, T> {
        
    }
    
    pub fn chunk<R>(&mut self, range: R) -> TokenQueue<'a, T>
    where
        R: RangeBounds<usize>,
    {
        let tokens = VecDeque::new();
        for i in range {
            tokens.push_back(self.tokens.remove(i));
        }
        TokenQueue { tokens, _phantom: &PhantomData, cursor: 0 }
    }
    
    pub fn is_token<D: From<T> + PartialEq + Copy>(&mut self, token_group : impl Into<OrdinalGroup<'a, T, D>>) -> CheckResult<'a, T> {
        self.start();
        
    }
    
    pub fn peek(&self) -> T {
        
    }
    
    pub fn start(&mut self) {
        self.removed.clear()
        self.mark = self.cursor;
    }
    
    pub fn rew
}