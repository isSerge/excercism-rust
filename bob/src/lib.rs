pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_question = message.ends_with('?');
    let is_yelling = message.to_uppercase() == message;
    let has_letters = message.chars().any(char::is_alphabetic);
    let is_empty = message.is_empty();

    match message {
        _ if is_empty => "Fine. Be that way!",
        _ if is_question && is_yelling && has_letters => "Calm down, I know what I'm doing!",
        _ if is_yelling && has_letters => "Whoa, chill out!",
        _ if is_question => "Sure.",
        _ => "Whatever."
    }
}
