
extern crate creusot_contracts;
use creusot_contracts::{std::vec, std::vec::Vec, std::Clone, *};

// to begin with, let's make hashmaps from u32 to i64

// #[derive(Clone)]
enum List { Nil, Cons(u32,i64,Box<List>) }

impl Clone for List {

    #[trusted]
    fn clone(&self) -> Self {
        panic!()
        // std::clone::Clone::clone(self)
        //            <self as std::Clone>::clone()
    }
}

struct MyHashMap {
    buckets: Vec<List>
}

#[predicate]
fn bucket_inv(l:List, index:Int, size:Int) -> bool {
    pearlite! {
        match l {
            List::Nil => true,
            List::Cons(k,_,tl) => @k % size === index && bucket_inv(*tl, index, size)
        }
    }
}

impl Model for MyHashMap {
    type ModelTy = Mapping<u32,Option<i64>>;

    #[logic]
    #[trusted]
    // TODO, or maybe not needed ?
    // #[ensures(
    //     forall<i:u32>
    //         result[i] ===
    //         (if self.is_elt(i) { Some((@(self.values))[i]) } else { None})
    // )]
    fn model(self) -> Self::ModelTy {
        std::process::abort()
    }
}


impl MyHashMap {

    /* The data invariant of the HashMap structure
     */
    #[predicate]
    fn hashmap_inv(&self) -> bool {
        pearlite! {
            0 < (@(self.buckets)).len() &&
            forall<i: Int> 0 <= i && i < (@(self.buckets)).len() ==>
                bucket_inv((@(self.buckets))[i],i,(@(self.buckets)).len())
        }
    }


    #[requires(self.hashmap_inv())]
    #[ensures(result === (@self).get(key))]
    fn get(&self, key: u32) -> Option<i64> {
        let index : usize = (key as usize) % self.buckets.len();
        let mut l : &List = &self.buckets[index];
        while let List::Cons(k, v, tl) = l {
            if *k == key { return Some(*v) }
            l = &**tl;
        }
        return None;
    }

    #[requires((*self).hashmap_inv())]
    #[ensures((^self).hashmap_inv())]
    #[ensures(forall<i:u32> (@^self).get(i) ===
              (if @i === @key { Some(val) } else { (@*self).get(i) } ))]
    fn add(&mut self, key:u32, val: i64) {
        let index : usize = (key as usize) % self.buckets.len();
        let mut l : &mut List = &mut self.buckets[index];
        while let List::Cons(k, v, tl) = l {
            if *k == key {
                *v = val;
                return;
            }
            else { l = &mut **tl; }
        }
        let l : List = std::mem::replace(&mut self.buckets[index], List::Nil);
         self.buckets[index] = List::Cons(key,val,Box::new(l));
    }

}

#[requires(0 < @size)]
#[ensures(result.hashmap_inv())]
#[ensures(forall<i:u32> (@result).get(i) === None)]
fn create(size: usize) -> MyHashMap
{
    MyHashMap {
        // buckets : vec![List::Nil;size]
        buckets: vec::from_elem(List::Nil,size)
    }
}



fn main() {
    // working around issue #163
    let none = None;
    let some17 = Some(17);
    let some42 = Some(42);
    // real tests
    let mut h1 : MyHashMap = create(17);
    let mut h2 : MyHashMap = create(42);
    let mut x = h1.get(1);
    let mut y = h1.get(2);
    let mut z = h2.get(1);
    let mut t = h2.get(2);
    // assert!(x == none && y == none && z == none && t == none);
    proof_assert!(x === none && y === none && z === none && t === none);
    h1.add(1,17);
    x = h1.get(1);
    y = h1.get(2);
    z = h2.get(1);
    t = h2.get(2);
    // assert!(x === some17 && y === none && z === none && t === none);
    proof_assert!(x === some17 && y === none && z === none && t === none);
    h2.add(1,42);
    x = h1.get(1);
    y = h1.get(2);
    z = h2.get(1);
    t = h2.get(2);
    // assert!(x === some17 && y === none && z === some42 && t === none);
    proof_assert!(x === some17 && y === none && z === some42 && t === none);
}
