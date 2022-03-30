extern crate creusot_contracts;
use creusot_contracts::*;
use creusot_contracts::std::*;

#[predicate]
fn match_at(needle: &Vec<u8>, haystack: &Vec<u8>, pos: Int, len: Int) -> bool {
  pearlite! { len <= (@needle).len()
    && pos <= (@haystack).len() - len
    && forall<i: Int>
        0 <= i && i < len ==> (@needle)[i] === (@haystack)[pos + i]
  }
}

#[requires((@needle).len() >= 1 && (@needle).len() <= (@haystack).len())]
#[ensures(@result == (@haystack).len() || @result < (@haystack).len() - (@needle).len() + 1)]
#[ensures(@result < (@haystack).len() ==>
            (match_at(needle, haystack, @result, (@needle).len())
             && (forall <i: Int> 0 <= i && i < @result ==> ! match_at(needle, haystack, i, (@needle).len()))))]
#[ensures(@result == (@haystack).len() ==> (forall <i: Int> 0 <= i && i < (@haystack).len() ==> ! match_at(needle, haystack, i, (@needle).len())))]
fn search(needle: &Vec<u8>, haystack: &Vec<u8>) -> usize {
   let mut i:usize = 0;
   // #[invariant(range_i,@i <= (@haystack).len() - (@needle).len() + 1)]
   #[invariant(no_match,forall<k: Int> 0 <= k && k < @i ==> ! match_at(needle, haystack, k, (@needle).len()))]
   while i < haystack.len() - needle.len() + 1 {
      let mut j:usize = 0;
      // #[invariant(range_j,@j <= (@needle).len())]
      #[invariant(partial_match,match_at(needle, haystack, @i, @j))]
      while j < needle.len() {
         proof_assert!(@j <= (@needle).len());
         if needle[j] != haystack[i + j] {
            break
         }
         j += 1;
      };
      proof_assert! { false};
      if j == needle.len() {
         return i
      };

      i += 1
   };
   return haystack.len()
}
