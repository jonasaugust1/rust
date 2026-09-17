enum MessageState {
    Pending(u32),
    Sending(u32),
    Received(u32)
}

fn main() {
    let message = MessageState::Pending(1);

    match message {
        MessageState::Pending(value) => println!("Message state is {}", value),
        MessageState::Sending(value) => println!("Message state is {}", value),
        MessageState::Received(value) => println!("Message state is {}", value),
        _ => println!("Message state is not pending")
    }

    if let MessageState::Pending(value) = message {
        println!("Message state is {}", value);
    }
}
