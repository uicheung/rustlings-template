// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

//

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = match number {
        "T-H-R-E-E" =>3,
        _=>{ // _代表一个通配符模式，可以匹配所有剩下的情况 , 这里不能使用None ,因为None是Option 类型
            5
        }
    }; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
