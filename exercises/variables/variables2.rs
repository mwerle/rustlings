// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.


// MW - Rust variable types
// integers : i/u 8-128bit/arch; eg i8, u32, isize
//   123 - dec, 0xff - hex; 0o77 - octal, 0b0011 - binary, byte (u8) b'M'
//   default: i32
// float : f32, f64
//   2.0, 
// boolean : bool; true / false
// character : (4-byte unicode) char; 'a', 'Z'
// tuples... () : let t : (i32, f32, char) = (42, 7.35, 'c');
//  t.0, t.1, ...
// arrays [] : let a: [i32, 5] = [...]; (type, number);
//  a[0], a[1], ...
//


fn main() {
    //let x;
    let x = 10;
    //let x:i32 = 42;
    //let x = "What do you get if you multiply six by seven?";
    //let x = 1.4;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
