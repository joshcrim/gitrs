extern crate clap;

use std::process::Command;
use clap::{App, SubCommand};


fn main() {
    let matches = App::new("gitrs")
        .version("0.1")
        .author("Josh C. <jshcrm@gmail.com>")
        .about("Collection of git shortcuts")
        .subcommand(SubCommand::with_name("merge"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("merge") {
        let mut current_branch = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .expect("fucked up");

        let mut current_branch: String = String::from_utf8_lossy(&current_branch.stdout).to_string();
        current_branch.pop();

        Command::new("git")
            .arg("stash")
            .spawn()
            .expect("fucked up");

        Command::new("git")
            .arg("checkout")
            .arg("master")
            .spawn()
            .expect("fucked up");

        Command::new("git")
            .arg("pull")
            .spawn()
            .expect("fucked up");

        Command::new("git")
            .arg("checkout")
            .arg(current_branch)
            .spawn()
            .expect("fucked up");

        Command::new("git")
            .arg("merge")
            .arg("master")
            .spawn()
            .expect("fucked up");

        Command::new("git")
            .arg("stash")
            .arg("pop")
            .spawn()
            .expect("fucked up");
    }

}
