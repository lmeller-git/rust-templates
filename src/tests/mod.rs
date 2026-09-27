#[cfg(loom)]
mod loom;

#[cfg(shuttle)]
mod shuttle;

use crate::add;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
