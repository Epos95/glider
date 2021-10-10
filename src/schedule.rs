use crossterm::style::Attributes;
use crossterm::style::Color;
use crossterm::style::SetAttribute;
use crossterm::style::SetForegroundColor;
use glider::*;
use rand::prelude::SliceRandom;
use rand::Rng;

/// A struct representing a drawable schedule.
#[derive(Debug)]
pub struct Schedule {
    activities: Vec<String>,
    times: Vec<(i16, i16)>,
    start_of_day: i16,
    colors: Vec<Color>,
}

/*
 * ISSUES:
 * logic for creating vector of times is borked
 * block size sometimes doesnt match up with how many rows gets printed!
 * lib.rs needs a touch up
 * proper commenting
 * some type annotations are missing
 * format ctime as bold and text as italics
 * small_test1 fails spectacuraely, needs to handle sub 1hour tasks better
 */

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let res = self
            .as_string()
            .expect("Could not represent schedule as a string.");

        let mut color_ctr = 0;
        for (i, line) in res.iter().enumerate() {
            let color = self.colors.get(color_ctr).unwrap();

            // to fix "ctime gets printed in color" we can check if the 
            // first part of the line is numbers and then print better
            if line.matches("█").count() > 10 && line.chars().nth(3) != Some(':') {
                // Print entirety with color
                write!(
                    f,
                    "{}{}{}\n",
                    SetForegroundColor(*color),
                    line,
                    SetForegroundColor(Color::Reset)
                )
                .unwrap();
            } else if line.chars().nth(3) == Some(':') {
                // print ctime on its own
                let (ctime, rest_of_line) = line.split_at(7);

                write!(
                    f,
                    "{}{}{}{}\n",
                    // maybe make ctime cursive or something here
                    ctime,
                    SetForegroundColor(*color),
                    rest_of_line,
                    SetForegroundColor(Color::Reset)

                ).unwrap();
            } else {
                // print the first and last character with color
                write!(
                    f,
                    "       {}{}{}{} {}{}{}\n",
                    SetForegroundColor(*color),
                    "█".to_string(),
                    SetForegroundColor(Color::Reset),
                    line[10..line.len() - 4].to_string(),
                    SetForegroundColor(*color),
                    "█".to_string(),
                    SetForegroundColor(Color::Reset)
                )
                .unwrap();
            }

            if line.chars().nth(1).unwrap().is_numeric() && i != 0 {
                color_ctr += 1;
            }
        }
        write!(f, "")
    }
}

impl Schedule {
    /// Creates a new Schedule from a vector of strings (lines) and a integer
    /// of when to start the day.
    pub fn new(input: Vec<String>, start_of_day: i16) -> Option<Self> {
        let activities: Vec<String> = get_activities(&input);
        let times: Vec<(i16, i16)> = get_times(&input)?;
        let mut colors: Vec<Color> = vec![
            Color::Blue,
            Color::Cyan,
            Color::Green,
            Color::Magenta,
            Color::Red,
            Color::Yellow,
        ];

        let mut rng = rand::thread_rng();

        if colors.len() < activities.len() {
            // we need more colors, randomly push more of them
            let saved_length = colors.len();

            // push colors untill we have one for every activity
            while colors.len() != activities.len() {
                colors.push(colors[rng.gen_range(0..saved_length)]);
            }

            colors.shuffle(&mut rng);

            // make sure two colors doesnt follow each other; doesnt work
            for i in 1..colors.len() {
                if colors[i] == colors[i - 1] {
                    let e = colors.remove(i);
                    colors.push(e);
                }
            }
        } else {
            // randomly pop elements untill we have the same length as activites
            while colors.len() != activities.len() {
                colors.remove(rng.gen_range(0..colors.len()));
            }

            colors.shuffle(&mut rng);
        }

        if activities.is_empty() || times.is_empty() || start_of_day < 0 || start_of_day > 24 {
            None
        } else {
            Some(Schedule {
                activities,
                times,
                start_of_day,
                colors,
            })
        }
    }

    /// This method gets the schedule as a string.
    ///
    /// as_string is used internally to format a Schedule when printing aswell.
    pub fn as_string(&self) -> Option<Vec<String>> {
        let mut rstr = vec![];
        let mut ctime = format!("{}:00", self.start_of_day);

        for i in 0..self.activities.len() {
            let minutes = if self.times.get(i)?.1.to_string().len() == 1 {
                "00".to_string()
            } else {
                self.times.get(i)?.1.to_string()
            };

            let block_height = if i == 0 {
                if self.start_of_day > self.times[i].0 {
                    return None;
                }

                ((((self.times[i].0 - self.start_of_day) * 60) + self.times[i].1) / 20) as i16
            } else {
                ((((self.times[i].0 - self.times[i - 1].0) * 60)
                    + (self.times[i].1 - self.times[i].1))
                    / 20) as i16
            };

            // DEBUG
            /*
            println!(
                "[DEBUG] Block height is: {} \n[DEBUG] Block time is: {}",
                block_height,
                block_height * 20
            );
            */


            // push row of bars
            rstr.push(format!(
                " {} {} ",
                &ctime,
                &"█".repeat(get_longest_activity(&self.activities).len() + 13)
            ));

            // push the time
            if block_height == 0 {
                rstr.push(format!(
                    "{}█ {} // {}:{}{}█",
                    " ".repeat(11 - ctime.len() + 1),
                    self.activities.get(i)?.to_owned(),
                    self.times.get(i)?.0,
                    minutes,
                    " ".repeat(
                        get_longest_activity(&self.activities).len()
                            - self.activities.get(i)?.len()
                            + 1
                    )
                ));
            } else {
                for c in 0..block_height {
                    if c == block_height / 2 {
                        rstr.push(format!(
                            "{}█ {} // {}:{}{} █",
                            " ".repeat(11 - ctime.len() + 1),
                            self.activities[i].to_owned(),
                            self.times.get(i)?.0,
                            minutes,
                            " ".repeat(
                                get_longest_activity(&self.activities).len()
                                    - self.activities[i].len()
                            )
                        ));
                    } else {
                        rstr.push(format!(
                            "{}{}",
                            " ".repeat(ctime.len() + 2),
                            &"█".repeat(get_longest_activity(&self.activities).len() + 13)
                        ));
                    }
                }
            }
            ctime = format!("{}:{}", self.times.get(i)?.0, minutes);
        }

        let n = rstr.len();
        rstr[n - 1] = format!(
            " {} {}",
            ctime,
            &"█".repeat(get_longest_activity(&self.activities).len() + 13)
        );

        Some(rstr)
    }
}
