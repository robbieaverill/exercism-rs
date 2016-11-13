/**
 * Return the squared value of the sum of first n natural numbers
 */
pub fn square_of_sum(n: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..n + 1 {
        sum += i;
    }
    sum * sum
}

/**
 * Return the sum of the squared values of the first n natural numbers
 */
pub fn sum_of_squares(n: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..n + 1 {
        sum += i * i;
    }
    sum
}

/**
 * Return the difference of the squared sum and the sum of squares from the first n natural numbers
 */
pub fn difference(n: i32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}
