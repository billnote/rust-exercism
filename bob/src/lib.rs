pub fn reply(message: &str) -> &str {
    match get_message_type(message) {
        MessageType::Question => "Sure.",
        MessageType::Uppercase => "Whoa, chill out!",
        MessageType::Empty => "Fine. Be that way!",
        MessageType::Other => "Whatever.",
    }
}

fn get_message_type(message: &str) -> MessageType {
    if is_question(message) {
        MessageType::Question
    } else if is_uppercase(message) {
        MessageType::Uppercase
    } else if is_empty(message) {
        MessageType::Empty
    } else {
        MessageType::Other
    }
}

enum MessageType {
    Question,
    Uppercase,
    Empty,
    Other,
}

fn is_question(message: &str) -> bool {
    (message.ends_with("?") || message.trim_right().ends_with("?")) && !is_uppercase(message)
}

fn is_uppercase(message: &str) -> bool {
    message == message.to_uppercase() && message != message.to_lowercase()
}

fn is_empty(message: &str) -> bool {
    message.trim().is_empty()
}
