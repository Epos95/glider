mod schedule;
#[cfg(test)]
mod tests;

use clap::{App, Arg, ArgMatches};
use crossterm::style::*;
use schedule::Schedule;
use std::io;
use std::fs;
use glider::GliderError;

fn main() {
    let app = App::new("Glider")
        .version("1.0")
        .author("Epos")
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

    match handle_command(app.get_matches()) {
        Err(GliderError::InvalidInput) => {
            println!("{}: Invalid inputs", "Error".red());
        }
        Err(GliderError::OpenFileError) => {
            println!(" {}: Couldnt open file.", "Error".red());
        }
        Ok(s) => {
            println!("{}", s);
        }
    }
}

fn handle_command(matches: ArgMatches) -> Result<Schedule, GliderError> {
    match matches.subcommand() {
        Some(("new", command)) => {
            // new command.

            let fname = command.value_of("file").unwrap_or("date"); // Get the current date here

            println!("{:?}", fname);

            return Err(GliderError::OpenFileError);
        }
        Some(("read", command)) => {
            // read command.
            //panic!("just use fs::read_to_string lmao");

            // Get value of command
            let file_name = command.value_of("read").unwrap();

            // Could implement into for `GliderError`
            let content: Vec<String> = if let Ok(s) = fs::read_to_string(file_name) {
                s.lines().map(|x| x.to_string()).collect()
            } else {
                return Err(GliderError::OpenFileError);
            };

            // Parse the file into a schedule
            let s = if let Some(s) = schedule::Schedule::new(content, 10) {
                s
            } else {
                return Err(GliderError::InvalidInput);
            };

            // Print the parsed schedule
            return Ok(s);
        }
        _ => {
            println!("Enter activity and what time to end it.\nFormat: <activity> <end time (hour:minute) || (hour)>");

            // first get input untill double newline
            let input_lines = read_stdin();

            // create a schedule with a start of day of 10 o clock
            let s: Schedule = if let Some(s) = Schedule::new(input_lines, 10) {
                s
            } else {
                // print error info and return since it makes
                // sense to just have the user restart the program
                // if they want to try again
                return Err(GliderError::InvalidInput);
            };

            return Ok(s);
        }
    }
}

/// Reads lines from stdin until a empty row is entered.
///
///
/// # Returns
///
/// * Returns a `Vec<String>`, strings inside have their new lines removed but that is it.
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
