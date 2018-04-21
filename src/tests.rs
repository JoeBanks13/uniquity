use ::*;

extern crate test;
use self::test::Bencher;

#[test]
fn test_generator() {
    let mut gen = new(1);
    for _x in 0..10 {
        println!("{}", gen.generate());
        println!("{}", gen.sequence);
    }
}

#[bench]
fn test_bulk_gen(x: &mut Bencher) {
    let mut gen = new(1);
    x.iter(|| gen.generate())
}
