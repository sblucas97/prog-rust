fn divide(x: u64, y: u64) -> u64 {
    assert!(y != 0);

    x / y
}

fn foo() -> u64 {
    64_u64
}
fn main() {
    //divide(10, 0);
    println!("{}", foo());
    println!("Hello, World!");

}

#[test]
fn test_foo() {
    assert_eq!(64_u64, foo());
}

