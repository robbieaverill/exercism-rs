pub fn raindrops(number: i64) -> String {
    let mut result = "" . to_string();
    if number % 3 == 0 {
        result.push_str("Pling");
    }
    if number % 5 == 0 {
        result.push_str("Plang");
    }
    if number % 7 == 0 {
        result.push_str("Plong");
    }

    if result.is_empty() {
        result = number.to_string();
    }

    return result
}
