use super::*;
#[test]
fn io_test1() {
    let input_lines = vec![
        "plan food 12:30".to_string(),
        "cook 13".to_string(),
        "eat 14".to_string(),
        "school 17".to_string(),
    ];
    let s = schedule::Schedule::new(input_lines, 10).unwrap();
    println!("{}", s);
}

#[test]
fn simple_test1() {
    let input_lines = vec!["test 11".to_string(), "programming 13".to_string()];
    let s = schedule::Schedule::new(input_lines, 10).unwrap();
    println!("{}", s);
}

#[test]
fn long_test1() {
    let input_lines = vec![
        "test 11".to_string(),
        "test more  12".to_string(),
        "cook 13".to_string(),
        "eat 14".to_string(),
        "more 15".to_string(),
        "test 16".to_string(),
        "school 17".to_string(),
        "asd 18".to_string(),
        "ajsd 19".to_string(),
    ];
    let s = schedule::Schedule::new(input_lines, 10).unwrap();
    println!("{}", s);
}

#[test]
fn small_test1() {
    let input_lines = vec![
        "test1 10:10".to_string(),
        "test2 10:15".to_string(),
        "test3 10:25".to_string(),
        "test4 10:50".to_string(),
    ];
    let s = schedule::Schedule::new(input_lines, 10).unwrap();
    let something = format!("{}", s);
    println!("{}", something);
}
