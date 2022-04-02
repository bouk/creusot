use crate as creusot_contracts;
use creusot_contracts_proc::*;

use crate::logic::*;

pub trait Eq: EqLogic {
    #[ensures(result === self.log_eq(*o))]
    fn eq(&self, o: &Self) -> bool;
}


impl<T : Eq> Eq for Option<T> {
    #[trusted]
    #[ensures(*self == *rhs)]
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Some(x), Some(y)) => x.eq(y),
            (None, None) => true,
            _ => false
        }
    }   
}