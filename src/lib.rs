
/// Parses activites from a input vector of strings
///
/// Input is a reference to the vector of unparsed lines directly from read_stdin().
pub fn get_activities(input: &Vec<String>) -> Vec<String> {
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
pub fn get_longest_activity(v: &Vec<String>) -> String {
    let mut s = String::from("");

    for string in v {
        if string.len() > s.len() {
            s = string.clone();
        }
    }

    s
}

/// Parses times from a Vec\<String\>
/// 
/// (is very dirtily written)
pub fn get_times(input: &Vec<String>) -> Option<Vec<(i16, i16)>> {
    let mut times = vec![];
    for line in input.iter() {
        let mut splat: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();

        // maybe assign this at declare time
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
                if duo.1 > 0 && duo.1 < 21 {
                    duo.1 = 20;
                } else if duo.1 >= 21 && duo.1 < 41 {
                    duo.1 = 40;
                } else if duo.1 >= 41 && duo.1 < 60 {
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
