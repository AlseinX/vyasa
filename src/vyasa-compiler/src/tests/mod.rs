use crate::compile;

const TEST_CODE: &str = include_str!("test.vy");

#[test]
fn it_works() {
    let result = compile(TEST_CODE).unwrap();
    dbg!(result);
}
