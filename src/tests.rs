use crate::*;
#[test]
fn test1() {
    let one = vec!["Something".to_string()];
    let two = vec![(1,1)];
    let x = Schedule{
        activities: one, 
        times: two
    };
    println!("{}", x);
}