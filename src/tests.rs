use ::*;

#[test]
fn test_generator() {
    let mut gen = new(0);
    loop {
        println!("{}", gen.generate())
    }
}
