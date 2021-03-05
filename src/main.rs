use std::fs::File;
use std::fs;
use std::env;
use std::io::Write;
use std::ffi::OsStr;
use std::thread;
use std::time;
use parse_duration::parse;
use chrono::{DateTime, Utc, NaiveDateTime, Local};

/****************************************************************
 * MINDFUL
 *
 * @author Per-Henrik E.K | 2021
 ****************************************************************/

fn get_config(option: &str) -> String{
    let mut  config :String =  env::var_os("HOME").expect("Error in env").into_string().expect("Could not parese to string");

    // Make shure that mindful config directory exists
    config.push_str("/.local/share/mindful/");
    fs::create_dir_all(&config).expect("Can not create config directory");
    config.push_str(option);
    return config
}


fn timer(time_expr: String){
    println!("Timestr {}", time_expr);
    let dur = parse("2s").expect("Could not parse date");

    thread::sleep(dur);
    println!("Done! ");
}

fn check_in() {
    /**
     * Checking mindful in
     **/
    println!("Checking in");

    let now = Local::now();
    let filepath = get_config("in");

    // probably make an alert messages before overwriting checkin timestamp ? 
    let mut f = File::create(filepath).expect("Unable to open");
    f.write( &now.to_rfc3339().as_bytes());
}

fn out() {
    /**
     * Checking mindful out
     **/
    println!("Checking out");
    let filepath = get_config("in");
    let in_content = fs::read_to_string(filepath).expect("Can not open in timestamp");
    let in_date = DateTime::parse_from_rfc3339( &in_content[..]).expect("Could not parse datetime string");

    // Compute time diference
    let now = Local::now();
    let diff = now.signed_duration_since(in_date);
    println!("You have meditated for {} minutes. ^^,", diff.num_minutes());
    
}
fn main() {


    let first_arg = std::env::args().nth(1).expect("help");

    match & first_arg[..]{

	"in" => check_in(),
	"out" => out(),
	"timer" => timer( std::env::args().nth(2).expect("Timer needs a second argument")),
	"help" => println!("mindful <option> | in for checking in, and out for checking out."),

	_ => println!("No argument given")
    }




}
