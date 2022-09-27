// variables1.rs
// Make me compile!
// Execute `rustlings hint variables1` or use the `hint` watch subcommand for a hint.


// MW: To declare variables in Rust:
//   let <varname> = <value>; // Auto-typed variable
//   let <varname>:<vartype> = <value>; // Explicitly-typed variable
// "Variables" are, by default, immutable; to allow changing the variable, prefix with "mut" :
//   let mut <varname>:<vartype> = <value>;
//

fn main() {
    let x = 5;
    println!("x has the value {}", x);
}
