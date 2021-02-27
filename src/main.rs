use std::fs::File;
use std::fs;
use std::env;
use std::io::Write;
use std::ffi::OsStr;

use chrono::{DateTime, Utc, NaiveDateTime, Local};


fn get_config(option: &str) -> String{
    let mut  config :String =  env::var_os("HOME").expect("Error in env").into_string().expect("Could not parese to string");

    // Make shure that mindful config directory exists
    config.push_str("/.local/share/mindful/");
    fs::create_dir_all(&config).expect("Can not create config directory");
    config.push_str(option);
    return config
}




fn check_in() {
    /**
     * Checking mindful in
     **/
    println!("Checking in");

    let now = Local::now();
    let filepath = get_config("in");

    println!("{}", filepath);
    let mut f = File::create(filepath).expect("Unable to open");
    f.write( &now.to_rfc3339().as_bytes());
}

fn out() {
    println!("Checking out");

    
}
fn main() {


    let first_arg = std::env::args().nth(1).expect("help");

    match & first_arg[..]{

	"in" => check_in(),
	"out" => out(),
	"help" => println!("Print help text"),

	_ => println!("No argument given")
    }




}
