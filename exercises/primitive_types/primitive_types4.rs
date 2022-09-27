// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

// MW: slices - reference the variable, then specify starting and ending indices: &<var>[<start>..<end>]
// NB: the slice is from the starting index to the ending index-1; ie [s..e)

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    //let nice_slice = ???
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
