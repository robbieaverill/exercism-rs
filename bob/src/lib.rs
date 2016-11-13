pub fn reply(message: &str) -> &str {
    // Nothing said...
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    // Question asked
    if message.ends_with("?") {
        return "Sure.";
    }

    // Someone is shouting...
    if message.to_uppercase() == message {
        return "Whoa, chill out!";
    }

    // Fallback.
    "Whatever."
}
