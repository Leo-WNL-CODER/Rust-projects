use std::process::Command;

pub fn redirect_command(input : &str)->String{
    let commands : Vec<&str> = input.split_whitespace().collect();

    let cmd  = commands[0];
    
    let mut new_cmd= Command::new(cmd);
    let output=if commands.len()>1{

        new_cmd.arg(commands[1]).output().expect("failed to execute...")

    }else{
        new_cmd.output().expect("failed to execute...")
    };

    let result =  String::from_utf8_lossy(&output.stdout).replace("\n"," ");
    result
}