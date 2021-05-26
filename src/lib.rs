use crossterm::style::*;

/// A struct representing a drawable schedule.
#[derive(Debug)]
pub struct Schedule {
    activities: Vec<String>,
    times: Vec<(i8, i8)>
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let res = self.as_string().red();

        write!(f, "{}", res)
    }
}

impl Schedule {
    /// Creates a new Schedule from a vector of strings (lines).
    pub fn new(input: Vec<String>) -> Option<Self> {
        let activities: Vec<String> = get_activities(&input);
        let times: Vec<(i8, i8)> = get_times(&input)?;

        if activities.is_empty() || times.is_empty() {
            None
        } else {
            Some(Schedule {
                activities,
                times
            })
        }
    }

    /// This method gets the schedule as a string.
    /// as_string is used internally to format a Schedule when printing aswell.
    pub fn as_string(&self) -> String {
        
        
       let rstr = String::from("this is a something");

       rstr
    }
}

fn get_activities(input: &Vec<String>) -> Vec<String> {
    let mut a: Vec<String> = vec![];
    for line in input.iter() {
        let mut s: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
        s.pop(); // popping elements like this is ugly but might be the best option
        a.push(s.join(" "));
    }
    a
}

fn get_times(input: &Vec<String>) -> Option<Vec<(i8,i8)>> {
    let mut times = vec![];
    for line in input.iter() {
        let mut splat: Vec<String> = line
            .split(" ")
            .map(|x| x.to_string())
            .collect();
        
        let time: (i8, i8);
        
        // first case: gets the singular number (time)
        // second case: (time:time)
        if splat.len() > 0 && splat.last().unwrap().chars().all(char::is_numeric) {
            time = if let Ok(n) = splat.pop().unwrap().parse() {
                (n, 0)
            } else {
                // invalid input
                return None;
            }; 
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
            let duo = (twin.get(0).unwrap().parse().unwrap_or(-1), twin.get(1).unwrap().parse().unwrap_or(-1));

            if 24 > duo.0 && duo.0 > 0 && 60 > duo.1 && duo.1 > 0 {
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