fn main() {
    // Expressions vs Statements
    let a = 1;
    let b = a + 1;

    // conditional expressions
    if a == 1 {
        println!("a");
    } else {
        println!("b");
    }

    // Assigning a value form an if
    let result = if a + 1 == 2 {a} else {b};
    println!("Conditional expression evaluated {}", result);

    let mut counter = 0;

    loop {
        if counter == 10 { break; }
        counter += 1;
        println!("{}", counter);
    }

    // Assigning values from a loop
    let mut start = 0;
    let loop_result = loop {
        start += 1;
        if start == 10 {
            break start;
        }
    };

    println!("Loop Result: {}", loop_result);

    // While
    let mut next_counter = 0;
    while next_counter < 10 {
        println!("Next loop: {}", next_counter);
        next_counter += 1;
    }

    // Array iteration
    let my_arr = [1,2,3,4,5];

    for num in my_arr {
        println!("Number: {}", num);
    }

    // Change the items in an array
    let mut my_other_arr = [1,2,3,4,5];
    for num in &mut my_other_arr {
        *num += 2;
        println!("Mutated Number: {}", num);
    }
    println!("Mutated Array {:?}", my_other_arr);
}
