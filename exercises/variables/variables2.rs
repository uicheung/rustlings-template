// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let x:Option<i32> = None;
    let x =  if custom_condition() {
        Some(10)
    } else {
        Some(20)
    };

    match x {
        Some(value)=>{
            if(value == 10){
                println!("这是一个 {}", value)
            }else {
                println!("这不是10")
            }
        },
        None=>{
            println!("什么都没有")
        }
    }
}
fn custom_condition()->bool{
    true
}
