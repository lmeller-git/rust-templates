use crate::add;

#[test]
fn it_works() {
    shuttle::check_random(|| assert_eq!(add(2, 2), 4), 1)
}
