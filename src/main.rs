extern crate clap;

use std::process::Command;
use clap::{Arg, App};


fn main() {
    let matches = App::new("gitrs")
        .version("0.1")
        .author("Josh C. <jshcrm@gmail.com>")
        .about("Collection of git shortcuts")
        .arg(Arg::with_name("command")
            .index(1))
            .help("Calls git [command]")
        .get_matches();

    let command = matches.value_of("command").unwrap();

    println!(" ");
    println!("{:?}", matches);
    println!(" ");
    println!("{}", command);

    Command::new("git")
        .arg(command)
        .spawn()
        .expect("fucked up");
}
