// SHOULD_SUCCEED: parse-print
extern crate creusot_contracts;

use creusot_contracts::*;

// Here we want to ensure that `program` properly shares
// the implementation of simple between itself and `uses_simple`.

#[logic]
fn simple() -> bool {
    true
}

#[logic]
fn uses_simple() -> bool {
    simple()
}

#[requires(uses_simple())]
#[ensures(simple())]
fn program() {}
