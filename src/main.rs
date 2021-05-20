// maybe treat 15 minutes as the smallest unit of time, then the user can specify how many
// of these they want to use in the schedule and makes drawing the schedule easier
use clap::{Arg, App};
use crossterm::style::*;


fn main() {

    // what args do we even need?
    // one for creating a new "stored" file: -n?
    // maybe one for reading from a specified schedule file
    // if the one above: a arg for reading from specific schedule

    let matches = App::new("Glider planning programm")
        .version("1.0")
        .author("Max Agnesund <maxagnesund95ATgmailDOTcom")
        .about("Helps you plan your day and present a graphic schedule")
        .arg(Arg::new("config")
    ).get_matches();

    // first get input untill double newline

    // sanitize input

    // compile a schedule
    //   use separate schedule datastructure for this

    // print the schedule
    //   do this with a impl for schedule

    // maybe cache the output for useage in input when re running
}
