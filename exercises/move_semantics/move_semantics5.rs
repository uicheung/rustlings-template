// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

//

fn main() {
    // let mut x = 100;
    // let y = &mut x; // 第一个可变引用
    // let z = &mut x; // 第二个可变引用，违反借用规则
    // *y += 100; // 解引用操作符 * 用于访问引用指向的实际值。
    // *z += 1000;
    // assert_eq!(x, 1200);

    let mut x = 100;
    let y = &mut x; // 第一个可变引用

    *y += 100;
    let z = &mut x; // 第二个可变引用，违反借用规则
    *z += 1000;
    assert_eq!(x, 1200);
}
