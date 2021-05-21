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

// This part should do the heavy lifting when it comes to printing 
// the schedule.
impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       write!(f, "a schedule") 
    }
}

impl Schedule {
    /// Creates a new Schedule from a vector of strings (lines).
    pub fn new(input: Vec<String>) -> Self {
        
        // this method handles all the parsing etc
        Schedule {
            activities: vec!["".to_string()],
            times: vec![(3,3)]
        }
    }

    /// This method is a shorthand for just printing the struct.
    /// Same as formatting the schedule with println.
    pub fn display(&self) {
        println!("{}", self);
    }
}
