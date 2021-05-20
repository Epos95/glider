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
        write!(f, "a schedule")
    }
}

impl Schedule {
}
