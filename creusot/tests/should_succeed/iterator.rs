#![feature(slice_take)]
extern crate creusot_contracts;
use creusot_contracts::*;
use creusot_contracts::std::*;

trait Iterator: Sized {
    type Item;

    #[predicate]
    fn completed(self) -> bool;

    #[predicate]
    fn produces(self, visited: Seq<Self::Item>, _: Self) -> bool;

    #[law]
    #[ensures(a.produces(Seq::EMPTY, a))]
    fn produces_refl(a: Self);

    #[law]
    #[requires(a.produces(ab, b))]
    #[requires(b.produces(bc, c))]
    #[ensures(a.produces(ab.concat(bc), c))]
    fn produces_trans(a: Self, ab: Seq<Self::Item>, b: Self, bc: Seq<Self::Item>, c: Self);

    #[ensures(match result {
      None => (*self).completed(),
      Some(v) => (*self).produces(Seq::singleton(v), ^self) && !(*self).completed()
    })]
    fn next(&mut self) -> Option<Self::Item>;
}

struct Map<I, F> {
    iter: I,
    func: F,
}

// impl<I : Iterator, B, F: FnMut(I::Item) -> B> Iterator for Map<I, F> {
//     type Item = B;

//     #[predicate]
//     fn completed(self) -> bool {
//         self.iter.completed()
//     }

//     #[predicate]
//     fn produces(self, visited: Seq<Self::Item>, succ: Self) -> bool {
//         pearlite! {
//             exists<is : Seq<I::Item>, fs : Seq<&mut F>>
//                    self.iter.produces(is, succ.iter )
//                 && is.len() === fs.len()
//                 && fs.len() === visited.len()
//                 && (forall<i : Int> 1 <= i && i < fs.len() ==>  ^fs[i - 1] === * fs[i])
//                 && (visited.len() > 0 ==> (
//                         * fs[0] === self.func
//                     &&  ^ fs[visited.len() - 1] === succ.func))
//                 && forall<i : Int>
//                     0 <= i && i < visited.len() ==>
//                     fs[i].postcondition_mut(is[i], visited[i])
//         }
//     }

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.iter.next()  {
//             Some(v) => self.func(v),
//             None => None,
//         }
//     }
// }

struct Range {
    start: isize,
    end: isize,
}

impl Iterator for Range {
    type Item = isize;

    #[predicate]
    fn completed(self) -> bool {
        pearlite! { self.start >= self.end }
    }

    #[predicate]
    fn produces(self, visited: Seq<Self::Item>, o: Self) -> bool {
        pearlite! {
            self.end === o.end && self.start <= o.start
            &&  visited.len() === @(o.start) - @(self.start)
            && forall<i : Int> 0 <= i && i < visited.len() ==>
                @(visited[i]) === @self.start + i
        }
    }

    #[law]
    #[ensures(a.produces(Seq::EMPTY, a))]
    fn produces_refl(a: Self) {}

    #[law]
    #[requires(a.produces(ab, b))]
    #[requires(b.produces(bc, c))]
    #[ensures(a.produces(ab.concat(bc), c))]
    fn produces_trans(a: Self, ab: Seq<Self::Item>, b: Self, bc: Seq<Self::Item>, c: Self) {}

    // #[requires(!(*self).completed())]
    #[ensures(match result {
      None => (^self).completed() && self.resolve(),
      Some(v) => (*self).produces(Seq::singleton(v), ^self) && !(*self).completed()
    })]
    fn next(&mut self) -> Option<isize> {
        if self.start >= self.end {
            None
        } else {
            let r = self.start;
            self.start += 1;
            Some(r)
        }
    }
}

struct IterMut<'a, T> {
    inner: &'a mut [T],
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[predicate]
    fn completed(self) -> bool {
        pearlite! { (@(self.inner)).len() === 0 }
    }

    #[predicate]
    fn produces(self, visited: Seq<Self::Item>, tl: Self) -> bool {
        pearlite! {
            (@*self.inner).len() == visited.len() + (@*tl.inner).len() &&
            (@^self.inner).len() == visited.len() + (@^tl.inner).len() &&
            (@(*self.inner)).subsequence(visited.len(), (@*self.inner).len()) == @*tl.inner &&
            (@(^self.inner)).subsequence(visited.len(), (@^self.inner).len()) == @^tl.inner &&
            (forall<i : Int> 0 <= i && i < visited.len() ==>
                (@*self.inner)[i] === *visited[i] && (@^self.inner)[i] === ^visited[i])
        }
    }

    #[law]
    #[ensures(a.produces(Seq::EMPTY, a))]
    fn produces_refl(a: Self) {}

    #[law]
    #[requires(a.produces(ab, b))]
    #[requires(b.produces(bc, c))]
    #[ensures(a.produces(ab.concat(bc), c))]
    fn produces_trans(a: Self, ab: Seq<Self::Item>, b: Self, bc: Seq<Self::Item>, c: Self) {}

    // #[requires((@*(*self).inner).len() === (@^(*self).inner).len())]
    #[ensures(match result {
      None => (^self).completed(),
      Some(v) => (*self).produces(Seq::singleton(v), ^self)
    })]
    // #[ensures((@*(^self).inner).len() === (@^(^self).inner).len())]
    fn next(&mut self) -> Option<Self::Item> {
        (self.inner).take_first_mut()
    }
}

// extern_spec! {
//     #[requires(@ix < (@self_).len())]
//     #[ensures(@*result.0 === (@*self_).subsequence(0, ix))]
//     #[ensures(@*result.1 === (@*self_).subsequence(ix, (@*self_).len()))]
//     #[ensures(@^result.0 === (@^self_).subsequence(0, ix))]
//     #[ensures(@^result.1 === (@^self_).subsequence(ix, (@*self_).len()))]
//     fn <[T]>::split_at_mut<T>(self_: &mut [T], ix: usize) -> (&mut [T], &mut [T])
// }

extern_spec! {
    #[ensures(match result {
        Some(r) => {
            * r === (@**s)[0] &&
            ^ r === (@^*s)[0] &&
            (@**s).len() > 0 && // ^*s.len === **s.len ? (i dont think so)
            (@^*s).len() > 0 &&
            @*^s === (@**s).tail() && @^^s === (@^*s).tail()
        }
        None => ^s === * s && (@**s).len() === 0
    })]
    fn <[T]>::take_first_mut<'a, T>(s: &mut &'a mut [T]) -> Option<&'a mut T>
}

// #[requires(!it.completed())]
// #[ensures(@(result.1).completed())]
// #[ensures(it.produces(@(result.2), @(result.1)))]
// #[ensures(@(result.2).sum() === @(result.0))]
// fn sum<I: Iterator<Item = u32>>(it: I) -> (u32, Ghost<I>, Ghost<Seq<i32>>) {
//     let mut x = 0;
//     let mut it_ghost = Ghost::record(&it);
//     let mut seq_ghost = Ghost::record(&Seq::EMPTY);
//     // Can't use the for sugar as it requires std iterators
//     #[invariant(xx, it_ghost.produces(@seq_ghost, it))]
//     #[invariant(yy, x === seq_ghost.sum())]
//     while let Some(e) = it.next() {
//         x += e;
//         seq_ghost = seq_ghost.push(e);
//     }
//     (x, Ghost::record(&i), seq_ghost)
// }

#[trusted]
#[ensures(@*result.inner === @*v)]
#[ensures(@^result.inner === @^v)]
fn iter_mut<'a, T>(v: &'a mut Vec<T>) -> IterMut<'a, T> {
    // IterMut { inner : &mut v[..] }
    panic!()
}

fn all_zero(v : &mut Vec<usize>) {
    let mut it = iter_mut(v);
    let it_old = Ghost::record(&it);
    let mut produced = Seq::EMPTY;
    #[invariant(structural, (@it_old).produces(produced, it))]
    loop {
        match it.next() {
            Some(x) => {
                // produced = produced.push(x);
                *x = 0;
            }
            None => break,
        }
    }
}

#[requires(@n >= 0)]
#[ensures(result === n)]
fn sum_range(n: isize) -> isize {
    let mut i = 0;
    {
        // the for loop
        let mut it = Range { start: 0, end: n };
        let it_old = Ghost::record(&it);
        let mut produced = Seq::EMPTY;
        #[invariant(free, (@it_old).produces(produced, it))]
        // user invariant
        #[invariant(user, @i === produced.len() && i <= n)]
        loop {
            match it.next() {
                Some(j) => {
                    produced = produced.push(j);
                    i += 1;
                }
                None => break,
            }
        }
    }
    i
}
