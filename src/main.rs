use chrono::{DateTime, Utc, NaiveDateTime, Local};

const CONFIG_DIR :&str = "~/.local/mindful/";

fn inn(){
    println!("Checking inn");

    let now = Local::now();
    
    println!("Date is {}...", now);
}

fn main() {

    let first_arg = std::env::args().nth(1).expect("help");

    match & first_arg[..]{

	"in" => inn(),
	"help" => println!("Print help text"),

	_ => println!("No argument given")
    }




}
