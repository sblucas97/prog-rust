//fn divide(x: u64, y: u64) -> u64 {
//    assert!(y != 0);
//
//    x / y
//}
//
//fn foo() -> u64 {
//    64_u64
//}
use std::env;
use std::str::FromStr;

fn main() {
    let mut my_vec: Vec<u64> = Vec::new();
    for arg in env::args().skip(1) {
        //println!("arg: {}", std::any::type_name_of_val(&arg)); 
        // alloc::string::String
        my_vec.push(u64::from_str(&arg).expect("Cannot convert string to u64"));
    }

    for x in my_vec.iter() {
        println!("Vec value: {}", x);
    }
}

//#[test]
//fn test_foo() {
//    assert_eq!(64_u64, foo());
//}

