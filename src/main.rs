// maybe treat 15 minutes as the smallest unit of time, then the user can specify how many
// of these they want to use in the schedule and makes drawing the schedule easier
use clap::{Arg, App};
use crossterm::style::*;
use glider::*;
use std::io::{self, Read};

fn main() {

    // what args do we even need?
    // one for creating a new "stored" file: -n?
    // maybe one for reading from a specified schedule file
    // if the one above: a arg for reading from specific schedule

    let matches = App::new("Glider planning programm")
        .version("1.0")
        .author("Max Agnesund <maxagnesund95ATgmailDOTcom")
        .about("Helps you plan your day and present a graphic schedule")
        .arg(Arg::new("new")
            .short('n')
            .long("new")
            .takes_value(false)
            .exclusive(true))
        .arg(Arg::new())
        .arg(Arg::new("config")
        
    ).get_matches();

    println!("Enter activites and times.\nFormat: <activity> <time modifier>");
    println!("{:?}", read_stdin());

    // first get input untill double newline

    // sanitize input

    // compile a schedule
    //   use separate schedule datastructure for this

    // print the schedule
    //   do this with a impl for schedule

    // maybe cache the output for useage in input when re running
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
        if buffer.trim().is_empty() {
            break
        } else {
            result_vector.push(buffer);
        }
    }

    // Return the vector.
    result_vector
}

fn parse_time(input: String) -> (i8,i8){

}

/// Creates a Schedule struct from a given Vec\<String\>.
fn schedule_from_string(input: Vec<String>) -> Schedule {

}