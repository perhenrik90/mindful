use chrono::{DateTime, Utc, NaiveDateTime, Local};

const CONFIG_DIR = "~/.local/mindful/";

fn inn(){
    println!("Checking inn");
    println!("Date is {}...", Local::now());
}

fn main() {

    let first_arg = std::env::args().nth(1).expect("help");
    println!("Hello, world!");


    match & first_arg[..]{

	"in" => inn(),
	"help" => println!("Print help text"),

	_ => println!("No argument given")
    }




}
