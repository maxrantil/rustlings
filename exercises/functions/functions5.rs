// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.


fn main() {
    let answer = square(9);
    println!("The square of 9 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
