use crossterm::style::*;

/// A struct representing a drawable schedule.
#[derive(Debug)]
pub struct Schedule {
    activities: Vec<String>,
    times: Vec<(i16, i16)>,
    start_of_day: i16,
}

// ISSUES:
// block height and block time are not consequential
// rounding is fucked in various parts of the program


impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // add pretty colors etc here

        let res = self
            .as_string()
            .expect("Could not represent schedule as a string.");
        
        for line in res {
            if line.contains("█") {
                write!(f, "{}\n", line).unwrap();
            } else {
                let mut x = line.chars().collect::<Vec<char>>();
                x[7] = '█';
                write!(f, "{}\n", x.into_iter().collect::<String>()).unwrap();
            }

        }
        write!(f,"")

        //write!(f, "{}", res.join("\n"))
    }
}


impl Schedule {
    /// Creates a new Schedule from a vector of strings (lines).
    pub fn new(input: Vec<String>, start_of_day: i16) -> Option<Self> {
        let activities: Vec<String> = get_activities(&input);
        let times: Vec<(i16, i16)> = get_times(&input)?;

        if activities.is_empty() || times.is_empty() || start_of_day < 0 || start_of_day > 24 {
            None
        } else {
            Some(Schedule {
                activities,
                times,
                start_of_day,
            })
        }
    }

    /// This method gets the schedule as a string.
    ///
    /// as_string is used internally to format a Schedule when printing aswell.
    pub fn as_string(&self) -> Option<Vec<String>> {
        let mut rstr = vec![];
        let mut ctime = format!("{}:00", self.start_of_day);
        // use string.push() instead of format!

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
            println!(
                "[DEBUG] Block height is: {} \n[DEBUG] Block time is: {}",
                block_height,
                block_height * 20
            );
            rstr.push(format!(
                " {} {} ",
                &ctime,
                &"█".repeat(get_longest_activity(&self.activities).len() + 11)
            ));

            if block_height == 0 {
                rstr.push(format!(
                    "{}{} | {}:{}",
                    " ".repeat(11 - ctime.len() + 2),
                    self.activities[i].to_owned(),
                    self.times.get(i)?.0,
                    minutes
                ));
            } else {
                for c in 0..block_height {
                    if c == block_height / 2 {
                        rstr.push(format!(
                            "{}{} // {}:{}",
                            " ".repeat(11 - ctime.len() + 2),
                            self.activities[i].to_owned(),
                            self.times.get(i)?.0,
                            minutes
                        ));
                    } else {
                        rstr.push(format!(
                            "{}{}",
                            " ".repeat(ctime.len() + 2),
                            &"█".repeat(get_longest_activity(&self.activities).len() + 11)
                        ));
                    }
                }
            }
        }

        Some(rstr)
    }
}

/// Parses activites from a input vector of strings
///
/// Input is a reference to the vector of unparsed lines directly from read_stdin().
fn get_activities(input: &Vec<String>) -> Vec<String> {
    // This vector contains our parsed strings
    let mut a: Vec<String> = vec![];

    // push all the parsed lines
    for line in input.iter() {
        let mut s: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
        s.pop(); // popping elements like this is ugly but might be the best option
        a.push(s.join(" "));
    }
    a
}

/// Gets the longest string of a vector of strings
fn get_longest_activity(v: &Vec<String>) -> String {
    let mut s = String::from("");

    for string in v {
        if string.len() > s.len() {
            s = string.clone();
        }
    }

    s
}

/// Parses times from a Vec\<String\>
/// (is very dirtily written)
fn get_times(input: &Vec<String>) -> Option<Vec<(i16, i16)>> {
    let mut times = vec![];
    for line in input.iter() {
        let mut splat: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();

        let time: (i16, i16);

        // first case: gets the singular number (time)
        // second case: (time:time)
        if splat.len() > 0 && splat.last().unwrap().chars().all(char::is_numeric) {
            let r = splat.pop()?.parse::<i16>().ok()?;

            if r > 24 || r < 0 {
                return None;
            }

            time = (r, 0);
        } else if splat.len() > 0 && splat.last().unwrap().contains(':') {
            let twin: Vec<String> = splat
                .pop()
                .unwrap()
                .split(":")
                .map(|x| x.to_string())
                .collect();

            if twin.len() != 2 {
                return None;
            }

            // This does the job but its probably not the best way of doing things
            // tuples are kinda unwieldy ime so far
            let mut duo = (
                twin.get(0).unwrap().parse().unwrap_or(-1),
                twin.get(1).unwrap().parse().unwrap_or(-1),
            );

            if 24 > duo.0 && duo.0 > 0 && 60 > duo.1 && duo.1 > 0 {
                // we need to round duo.1 to a 20 min interval here
                if duo.1 > 0 && duo.1 < 20 {
                    duo.1 = 20;
                } else if duo.1 >= 20 && duo.1 < 40 {
                    duo.1 = 40;
                } else if duo.1 >= 40 && duo.1 < 60 {
                    duo.1 = 59;
                }
                time = duo;
            } else {
                return None;
            }
        } else {
            // invalid input
            return None;
        }
        times.push(time);
    }
    Some(times)
}
