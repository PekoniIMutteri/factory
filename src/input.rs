use std::io;

pub fn wait_enter(){
    let _ = io::stdin().read_line(&mut String::new());
}
