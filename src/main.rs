fn main() {
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p}",ptr);
    println!("{:?}",len);
}
