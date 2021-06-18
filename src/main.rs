use clap::{App, Arg};
use crossterm::style::*;
use std::fs::*;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[cfg(test)]
mod tests;

mod schedule;

fn main() {
    let app = App::new("Glider")
        .version("1.0")
        .author("Max Agnesund <maxagnesund95ATgmailDOTcom")
        .about("Helps you plan your day and present a graphic schedule")
        .subcommand(
            App::new("new")
                .short_flag('n')
                .about("Creates a new schedule file")
                .arg(
                    Arg::new("filename")
                        .about("Specifies what name to save the schedule under.")
                        .short('f')
                        .long("file")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("read")
                .short_flag('r')
                .about("Reads a schedule file and prints it")
                .arg(
                    Arg::new("filename")
                        .about("Specifies what schedule to read by name.")
                        .short('f')
                        .long("file")
                        .required(true)
                        .takes_value(true),
                ),
        );

    // match subcommands.
    match app.get_matches().subcommand() {
        Some(("new", command)) => {
            // new command.

            let fname = command.value_of("file").unwrap_or("date"); // Get the current date here

            println!("{:?}", fname);
        }
        Some(("read", command)) => {
            // read command.
            panic!("just use fs::read_to_string lmao");

            // Get value of command
            let command_value = command.value_of("read").unwrap();

            // Get file pointer
            let fp = if let Ok(f) = File::open(command_value) {
                f
            } else {
                println!(" {}: Couldnt open file.", "Error".red());
                return;
            };
            let b = BufReader::new(fp);

            // Get the contents of the file as a vector
            let contents = b.lines().map(|l| l.unwrap()).collect();

            // Parse the file into a schedule
            let s = if let Some(s) = schedule::Schedule::new(contents, 10) {
                s
            } else {
                println!("{}: Invalid inputs", "Error".red());
                return;
            };

            // Print the parsed schedule
            println!("{}", s);
        }
        _ => {
            println!("Enter activity and what time to end it.\nFormat: <activity> <end time (hour:minute) || (hour)>");

            // first get input untill double newline
            let input_lines = read_stdin();

            // create a schedule with a start of day of 10 o clock
            let s = if let Some(s) = schedule::Schedule::new(input_lines, 10) {
                s
            } else {
                // print error info and return since it makes
                // sense to just have the user restart the program
                // if they want to try again
                println!("{}: Invalid inputs", "Error".red());
                return;
            };

            // print the schedule
            println!("{}", s);
        }
    }
}

/// Reads lines from stdin until a empty row is entered.
///
/// # Returns
/// Returns a Vec\<String\>, strings inside have their new lines removed but that is it.
fn read_stdin() -> Vec<String> {
    // Vector to store the read strings in.
    let mut result_vector = vec![];
    let stdin = io::stdin();

    // Read lines indefinetly.
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Could not read input.");

        // Remove newlines from string.
        buffer = buffer.trim().to_string();

        // Return if buffer is empty, otherwise push the string to the result_vector.
        if buffer.is_empty() {
            break;
        } else {
            result_vector.push(buffer);
        }
    }

    // Return the vector.
    result_vector
}
