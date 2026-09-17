fn main() {
    let mut a = 1;

    // This is a value type stored on the stack
    // won't alter the original
    // the copy will be increased not the original
    fn increase(mut input: i32) {
        input += 1;
        println!("input after increase: {}", input);
    }

    increase(a);
    println!("a after increase: {}", a);

    fn increase_by_reference(input: &mut i32) {
        *input += 1;
    }
    increase_by_reference(&mut a);
    println!("a after increase by reference: {}", a);

    let str_slice = "Hello, world!";
    let my_string = String::from(str_slice);
    let slice_from_string = &my_string[0..5];
    println!("slice_from_string: {}", slice_from_string);
}
