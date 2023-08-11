fn main() {
    println!("Hello, world!");
    another_function();
}
fn another_function(){
    println!("No.2!!");
    let x = another_function2(32);
}
fn another_function2(x : i32) -> i32 {
    println!("{}",x);
    x
}