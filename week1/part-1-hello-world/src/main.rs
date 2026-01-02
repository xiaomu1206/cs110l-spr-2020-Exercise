use std::iter::Sum;


fn main() {
    println!("Hello, world!");

    let mut s: String = String::from("Hello ");
    s.push_str("world!");

    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(3);

    let mut arr: [i32; 4] = [0, 2, 4, 8];
    arr[0] = -2;
    //println!("{}", arr[0] + arr[1]);

    for i in arr.iter() {
        println!("{}", i);
    }
}
