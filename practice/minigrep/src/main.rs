use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); //收集命令行参数

    //println!("{:?}",args);
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1); //不同于panic，exit不会产生额外的输出
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config){
      eprintln!("Application error: {e}");
      process::exit(1);
    }
}




