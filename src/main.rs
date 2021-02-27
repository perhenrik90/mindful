use std::fs::File;
use std::fs;
use std::env;
use std::io::Write;
use std::ffi::OsStr;

use chrono::{DateTime, Utc, NaiveDateTime, Local};

static CONFIG_DIR :&String = ""


fn cmd_init(){

    let mut home = env::var_os("HOME").expect("Error in env");

    CONFIG_DIR = format!("{}{}", &home.into_string().expect("Can not parse string")[..], CONFIG_DIR);

    println!("Config is {}", );
}
fn inn()  {
    /**
     * Checking mindful inn
     **/
    println!("Checking inn");

    let now = Local::now();
    let filepath = format!("{}{}", CONFIG_DIR, "test.txt");
    println!("{}", filepath);
    let mut f = File::create(filepath).expect("Unable to open");
    f.write( &now.to_rfc3339().as_bytes());
}

fn out() {
    println!("Checking out");

    
}
fn main() {


    cmd_init();
    let first_arg = std::env::args().nth(1).expect("help");

    match & first_arg[..]{

	"in" => inn(),
	"out" => out(),
	"help" => println!("Print help text"),

	_ => println!("No argument given")
    }




}
