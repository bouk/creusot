// WHY3PROVE NOSPLIT CVC4
extern crate creusot_contracts;
use creusot_contracts::std::*;
use creusot_contracts::*;

#[predicate]
fn sorted_range(s: Seq<u32>, l: Int, u: Int) -> bool {
    pearlite! {
        forall<i : Int, j : Int> l <= i && i < j && j < u ==> s[i] <= s[j]
    }
}

#[predicate]
fn sorted(s: Seq<u32>) -> bool {
    sorted_range(s, 0, s.len())
}

#[requires(sorted(@arr))]
#[requires(@elem < (@arr).len())]
fn binary_search(arr: &Vec<u32>, elem: u32) -> Result<usize, usize> {
    if arr.len() == 0 {
        return Err(0);
    }
    let mut size = arr.len();
    let mut base = 0;

    #[invariant(size_valid, 0 < @size && @size + @base <= (@arr).len())]
    while size > 1 {
        let half = size / 2;
        let mid = base + half;

        base = if arr[mid] > elem { base } else { mid };
        size -= half;
    }

    let cmp = arr[base];
    if cmp == elem {
        Ok(base)
    } else if cmp < elem {
        Err(base + 1)
    } else {
        Err(base)
    }
}
