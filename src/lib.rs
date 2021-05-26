mod tests;

// this needs to be serializeable aswell
/// A struct representing a drawable schedule.
/// 
/// 
#[derive(Debug)]
pub struct Schedule {
    activities: Vec<String>,
    times: Vec<(i8, i8)>
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_string = String::from("something goes here");
        let res = self.as_string().unwrap_or(error_string);

        write!(f, "{}", res)
    }
}

impl Schedule {
    /// Creates a new Schedule from a vector of strings (lines).
    pub fn new(input: Vec<String>) -> Self {
        
        // this method handles all the parsing etcdefault: T
        Schedule {
            activities: vec!["".to_string()],
            times: vec![(3,3)]
        }
    }

    /// This method gets the schedule as a string.
    /// Printing it later will use this.
    pub fn as_string(&self) -> Option<String> {
        if self.times.len() > 0 && self.activities.len() != self.times.len() {
            return None;
        }

        Some("x".to_string())
    }
}
