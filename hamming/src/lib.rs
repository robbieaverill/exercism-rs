/**
 * Calculate the "hamming" distance between two strings and return as a Result
 */
pub fn hamming_distance(a: &str, b: &str) -> Result<i32, i32> {
    // Basic error handling
    if a.len() != b.len() {
        return Err(0);
    }

    let mut ham_count = 0;
    for (key, character) in a.chars().enumerate() {
        // Compare each character to the equivalent position in b
        if character != b.chars().nth(key).unwrap() {
            ham_count += 1;
        }
    }

    Ok(ham_count)
}
