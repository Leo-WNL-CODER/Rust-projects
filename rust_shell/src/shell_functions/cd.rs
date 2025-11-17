use std::path::Path;
use std::env;

pub fn change_directory(input:&str){
    // println!("s");
    let part : Vec<&str> = input.split_whitespace().collect();
    let path = if part.len()>1{
        part[1]
    }else{
        println!("{}",env::var("HOME").unwrap());
        &env::var("HOME").unwrap()
    };
    if let Err(e) = env::set_current_dir(Path::new(path)) {
        println!("cd failed: {}", e);
    }
}
