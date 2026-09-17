//Evry value is owned
//One owner at a time
//When the owner leaves scope, what is owned gets dropped

#[derive(Debug, Clone, Copy)]
struct Coffe {
    id: i32,
    count: i32
}

fn main() {
    // Moves/Copies primitives, structs
    let a = 1;
    let b = a;
    //The primitive value type is implicitly copied over here
    println!("a: {}", a);
    println!("b: {}", b);

    let string_a = String::from("hello");
    let string_b = string_a;
    // Program will not compile because string_a value was moved
    // println!("a: {}", string_a);
    println!("b: {}", string_b);

    let string_c = String::from("hello");
    let string_d = string_c.clone();
    println!("c: {}", string_c);
    println!("d: {}", string_d);

    let coffee_a = Coffe { id: 1, count: 1 };
    let coffee_b = coffee_a;
    println!("coffee_b: {:?}", coffee_b);
    // It will only work if struct is Copy
    println!("coffee_a: {:?}", coffee_a);

    // If the struct has fields that allocates memory on
    // the heap it should be used Clone
    let coffee_c = Coffe { id: 2, count: 2 };
    let coffee_d = coffee_c.clone();
    println!("coffee_d: {:?}", coffee_d);
}
