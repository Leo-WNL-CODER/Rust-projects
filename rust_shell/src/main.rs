
use std::io::{self, Write};

use crate::shell_functions::redirect::{redirect_command};

mod shell_functions;


fn main() {
    loop {
        let cur_dir=redirect_command("pwd");
        print!("mini-shell:~{}$ ",cur_dir.trim());
        io::stdout().flush().unwrap();
        let mut input=String::new();


        if io::stdin().read_line(&mut input).is_err(){
            continue;
        }

        
        let input=input.trim();

        //exit
        if input=="exit"{
            break;
        }

        //cd
        if input.starts_with("cd"){
            shell_functions::cd::change_directory(input);
            continue
        }

        //redirection
        let result:String=redirect_command(input);
        println!("{}",result);

        
    }
}
