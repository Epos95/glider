this script is meant to take some activites and the time allocated for them and then generate a schedule visible in the terminal with color and proper time etc

ways of input:
    text file of lines,
    built in line reader via cli/curses

format of input:
    <name of task> <allocated time>

info:
    allocated time distinguish how big the time block should be in the terminal

### Todo list:
* Rewrite to use actual errors with a proper returning "main" function
* let GliderError::InvalidInput print the input which is invalid
* Fix the smallest number increment error (see `cargo test -- --show-output` for more info)
