// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

//

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return  num * num; // 不写return变成了一个语句，而不是一个表达式，因此函数实际上没有返回值，这在Rust中会导致编译错误
    // 语句 和 表达式区别，    语句执行一个操作，但不会返回一个值  ，表达式会计算出一个值。它们可以组合成更复杂的表达式
}
