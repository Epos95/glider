use super::*;
#[test]
fn io_test1() {
    let input_lines = vec![
        "plan food 12:30".to_string(),
        "cook 13".to_string(),
        "eat 14".to_string(),
        "school 17".to_string(),
    ];
    let s = schedule::Schedule::new(input_lines, 10)
        .unwrap();
    println!("{}", s);
}

#[test]
fn simple_test1() {
    let input_lines = vec![
        "test 11".to_string(),
        "programming 13".to_string(),
    ];
    let s = schedule::Schedule::new(input_lines, 10)
        .unwrap();
    println!("{}", s);
}