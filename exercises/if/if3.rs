// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

//

pub fn animal_habitat(animal: &str) -> &'static str {

    // &：表示一个引用（reference）
    // 'static：是一个生命周期标注，表示数据的生命周期是“静态”的，即数据在整个程序运行期间都有效。
    // str：是一个字符串切片（string slice），表示一段不可变的字符串数据。
    let identifier = 1;

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    }else {
        "Unknown"
    };

    habitat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
