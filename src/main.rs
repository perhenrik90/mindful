
fn main() {

    let first_arg = std::env::args().nth(1).expect("help");
    println!("Hello, world!");


    match & first_arg[..]{

	"in" => println!("Check inn..."),
	"help" => println!("Print help text"),

	_ => println!("No argument given")
    }




}
