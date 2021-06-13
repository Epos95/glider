use glider::*;

/// A struct representing a drawable schedule.
#[derive(Debug)]
pub struct Schedule {
    activities: Vec<String>,
    times: Vec<(i16, i16)>,
    start_of_day: i16,
}

/* 
 * ISSUES:
 * block height and blocksize are not consequential
 * rounding is fucked in various parts of the program
 * color system still doesnt work
 * print time at the end aswell
 * printing a schedule is still kinda weird
 */


impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let res = self
            .as_string()
            .expect("Could not represent schedule as a string.");
        
        for line in res {
            if line.matches("█").count() > 2 {
                // Print entirety with color
                write!(f, "{}\n", line).unwrap();
            } else {
                // print the first and last character with color
                write!(f, "{}\n",line).unwrap();
            }

        }
        write!(f,"")
    }
}


impl Schedule {
    /// Creates a new Schedule from a vector of strings (lines) and a integer 
    /// of when to start the day.
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
                    self.activities[i].to_owned(),
                    self.times.get(i)?.0,
                    minutes,
                    " ".repeat(get_longest_activity(&self.activities).len()-self.activities[i].len()+1)    
                ));
            } else {
                for c in 0..block_height {
                    if c == block_height / 2 {
                        rstr.push(format!(
                            "{}█ {} // {}:{}{}█",
                            " ".repeat(11 - ctime.len()+1),
                            self.activities[i].to_owned(),
                            self.times.get(i)?.0,
                            minutes,
                            " ".repeat(get_longest_activity(&self.activities).len()-self.activities[i].len()+1)    
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

        Some(rstr)
    }
}