// Test that we use fully-qualified type names in error messages.

import core::task::task;

fn bar(x: uint) -> task {
    ret x;
    //!^ ERROR mismatched types: expected `core::task::task`
}

fn main() {
}
