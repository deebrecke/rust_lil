use std::fmt;

fn get_displayable(choice: bool) -> impl fmt::Display{
    if choice{
        13
    }else{
        "thirteen"
    }
}

fn main() {
    println!("Output is {}", get_displayable())
}