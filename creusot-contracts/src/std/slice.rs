use crate as creusot_contracts;
use creusot_contracts_proc::*;

use crate::OrdLogic;

extern_spec! {
  #[ensures((@s).len() === @result)]
  fn <[T]>::len<T>(s: &[T]) -> usize
}

extern_spec! {
    #[requires(@i < (@s).len())]
    #[requires(@j < (@s).len())]
    #[ensures((@^s).exchange(@*s, @i, @j))]
    fn <[T]>::swap<T>(s: &mut [T], i: usize, j: usize)
}