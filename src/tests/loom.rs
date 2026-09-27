use crate::add;

#[test]
fn it_works() {
    loom::model(|| assert_eq!(add(2, 2), 4));
}
