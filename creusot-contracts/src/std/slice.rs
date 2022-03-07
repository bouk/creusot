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

use std::slice::SliceIndex;

extern_spec! {
    #[ensures(match result {
        Some(t) => *t === (@self_)[ix],
        None => true, // should learn something about the index being out of bounds
    })]
    fn <[T]>::get<T, I : SliceIndex<[T]>>(self_: &[T], ix: I) -> Option<&<I as SliceIndex<[T]>>::Output>
      where crate::Seq<T> : Index<I, Output = <I as SliceIndex<[T]>>::Output>

}

extern_spec! {
  fn <[T]>::index<T, I : SliceIndex<[T]>>(self_: &[T], ix: I) -> &<I as SliceIndex<[T]>>::Output
    where crate::Seq<T> : Index<I, Output = <I as SliceIndex<[T]>>::Output>
}

use std::ops::Index;

// // A hack which allows us to use [..] notation for sequences.
// // Relies on the fact we don't enforce that implementations of traits are of
// // the same function type as the trait signature.. When this is addressed
// // the following instance will error.
// impl<T> std::ops::Index<Int> for Seq<T> {
//     type Output = T;

//     #[trusted]
//     #[logic]
//     #[creusot::builtins = "seq.Seq.get"]
//     fn index(&self, _: Int) -> &T {
//         std::process::abort()
//     }
// }



// trait SeqIndex<T:?Sized> {
//   type Output = T;

//   #[logic]
//   fn index(self, seq: Seq<T>) -> Self::Output;
// }

// impl SeqIndex<T> for Int {
//   type Output = T;

//     #[trusted]
//     #[logic]
//     #[creusot::builtins = "seq.Seq.get"]
//     fn index(self, seq: Seq<T>) -> Output {
//       // flip get arguments around...
//         std::process::abort()
//     }
// }

// impl SeqIndex<T> for Range<Int> {
//     type Output = Seq<T>;

//     #[trusted]
//     #[logic]
//     #[creusot::builtins = "seq.Seq.get"]
//     fn index(self, seq: Seq<T>) -> Self::Output {
//       // flip get arguments around...
//         std::process::abort()
//     }
// }