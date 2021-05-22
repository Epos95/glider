// maybe treat 15 minutes as the smallest unit of time, then the user can specify how many
// of these they want to use in the schedule and makes drawing the schedule easier
use clap::{Arg, App};
use crossterm::style::*;
use glider::*;
use std::io::{self, Read};

fn main() {

    // use subcommands here instead
    let app = App::new("Glider")
        .version("1.0")
        .author("Max Agnesund <maxagnesund95ATgmailDOTcom")
        .about("Helps you plan your day and present a graphic schedule")
        .subcommand(App::new("new")
            .short_flag('n')
            .about("Creates a new schedule file")
            .arg(Arg::new("filename")
                .about("Specifies what name to save the schedule under.")
                .short('f')
                .long("file")
                .takes_value(true)))
        .subcommand(App::new("read")
            .short_flag('r')
            .about("Reads a schedule file and prints it")
            .arg(Arg::new("filename")
                .about("Specifies what schedule to read by name.")
                .short('f')
                .long("file")
                .takes_value(true)));

    // match subcommands.
    match app.get_matches().subcommand() {
        Some(("new",  command)) => {
            // new command.

            let fname = command
                .value_of("file") // make this return the current date as a string

            println!("{}", fname);

        },
        Some(("read", command)) => {
            // read command.
        },
        _ => {
            println!("TESTING");
            println!("Enter activity and what time to end it.\nFormat: <activity> <time modifier>");

            // first get input untill double newline
            let activites = read_stdin();

            // compile a schedule
            //   use separate schedule datastructure for this
            let s = Schedule::new(activites);

            // print the schedule
            //   do this with a impl for schedule
            println!("{}", s);

            // maybe cache the output for useage in input when re running
        }
    }
}

/// Reads lines from stdin until a empty row is entered.
/// 
/// ## Returns
/// Returns a Vec\<String\>, strings inside have their new lines removed but that is it.
fn read_stdin() -> Vec<String> {
    // Vector to store the read strings in.
    let mut result_vector = vec![];
    let stdin = io::stdin();

    // Read lines indefinetly.
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)
            .expect("Could not read input.");
        
        // Remove newlines from string.
        buffer = buffer
            .trim()
            .to_string();

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