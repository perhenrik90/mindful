use std::fs::File;
use std::io::Write;

use chrono::{DateTime, Utc, NaiveDateTime, Local};

const CONFIG_DIR :&str = "~/.local/mindful/";

fn inn()  { 
    println!("Checking inn");

    let now = Local::now();
    let mut f = File::create("test.txt").expect("Unable to open");
    f.write( &now.to_rfc3339().as_bytes());
}

fn main() {

    let first_arg = std::env::args().nth(1).expect("help");

    match & first_arg[..]{

	"in" => inn(),
	"help" => println!("Print help text"),

	_ => println!("No argument given")
    }




}
