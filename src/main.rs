extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
  let matches = App::new("My Super Program")
    .version("1.0")
    .author("Kevin K. <kbknapp@gmail.com>")
    .about("Does awesome things")
    .arg(
      Arg::with_name("INPUT")
        .help("Sets the input file to use")
        .required(true)
        .index(1),
    )
    .get_matches();

  // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
  // required we could have used an 'if let' to conditionally get the value)
  println!("Using input file: {}", matches.value_of("INPUT").unwrap());
}
