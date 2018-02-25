extern crate clap;
extern crate console;

use std::process::Command;
use clap::{App, SubCommand};
use console::style;

fn main() {

    let matches = App::new("gitrs")
        .version("0.1")
        .author("Josh Crim <jshcrm@gmail.com>")
        .about("Collection of git shortcuts")
        .subcommand(SubCommand::with_name("merge"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("merge") {

        let mut current_branch = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .expect("Failed to retrieve current branch name.");

        let mut current_branch: String = String::from_utf8_lossy(&current_branch.stdout).to_string();
        current_branch.pop();

        println!("{}", style("Stashing current branch...").blue());

        Command::new("git")
            .arg("stash")
            .output()
            .expect("Failed to stash existing changes.");

        println!("{}", style("success").green());
        println!("Checking {}", style("Checking out master...").blue());

        Command::new("git")
            .arg("checkout")
            .arg("master")
            .output()
            .expect("Failed to switch branch to master.");

        println!("{}", style("success").green());
        println!("{}", style("Pulling master...").blue());

        Command::new("git")
            .arg("pull")
            .output()
            .expect("Failed to pull from master.");

        println!("{}", style("success").green());
        println!("{}", style("Checking out branch...").blue());

        Command::new("git")
            .arg("checkout")
            .arg(current_branch)
            .output()
            .expect("Failed to checkout current branch.");

        println!("{}", style("success").green());
        println!("{}", style("Merging with master...").blue());

        Command::new("git")
            .arg("merge")
            .arg("master")
            .output()
            .expect("Failed to merge master.");

        println!("{}", style("success").green());
        println!("{}", style("Popping stash...").blue());

        Command::new("git")
            .arg("stash")
            .arg("pop")
            .output()
            .expect("Failed to pop stash.");

        println!("{}", style("success").green());
    }
}
