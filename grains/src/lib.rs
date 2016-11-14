/**
 * Calculate how many grains would be given for the provided chess board square
 */
pub fn square(square: u32, ) -> u64 {
    get_grains(square)
}

/**
 * Calculates the sum of how many grains would be on an entire chess board
 */
pub fn total() -> u64 {
    let mut result: u64 = 0;
    for i in 1..65 {
        result += get_grains(i);
    }
    result
}

/**
 * Calculate how many grains would be given for the provided chess board square
 */
fn get_grains(square: u32) -> u64 {
    // Idiot proofing
    if square > 64 || square < 1 {
        panic!("Square must be between 1 and 64");
    }

    let mut grains: u64 = 0;
    let mut last_grains: u64 = 1;

    for _ in 1..square + 1 {
        grains += last_grains;
        last_grains = grains;
    }

    grains
}
