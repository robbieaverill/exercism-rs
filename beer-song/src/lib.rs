/**
 * Define the limit of bottles - will reset to this when the next bottle is less than zero
 */
const MAX_BOTTLES: i32 = 99;

/**
 * A Bottle represents the current bottle number, a pluralised suffix to use ("s") if relevant,
 * and the next bottle number equivalent as well
 */
struct Bottle {
    current: String,
    next: String,
    current_plural: &'static str,
    next_plural: &'static str
}

/**
 * Gets a single line verse from the song for the "bottle" number provided
 */
pub fn verse(bottle: i32) -> String {
    get_verse(bottle)
}

/**
 * Gets multiple lines from the verse according to the "bottle" number (start), and limit (stop)
 */
pub fn sing(start: i32, stop: i32) -> String {
    if start < stop {
        panic!("Start must be greater than stop... E.g. start at 5 and stop at 2");
    }

    let mut result = String::new();
    let mut i: i32 = start;
    while i >= stop {
        // Weird... but get over it.
        if i != start {
            result.push_str("\n");
        }

        result.push_str(&get_verse(i));
        i -= 1;
    }

    result
}

/**
 * Gets the required Bottle data and returns it as a formatted verse string
 */
fn get_verse(bottle: i32) -> String {
    let my_bottle = get_bottle(bottle);
    let instruction = get_instruction(bottle);

    format!(
        "{} bottle{} of beer on the wall, {} bottle{} of beer.\n{}, {} bottle{} of beer on the wall.\n",
        my_bottle.current,
        my_bottle.current_plural,
        my_bottle.current.to_lowercase(),
        my_bottle.current_plural,
        instruction,
        my_bottle.next.to_lowercase(),
        my_bottle.next_plural,
    )
}

/**
 * Gets a Bottle object containing the populated data for the current and next bottle numbers, and
 * their optional suffixes
 */
fn get_bottle(bottle: i32) -> Bottle {
    let current_bottle = get_bottle_number(bottle);
    // Get the next bottle - resets to the MAX_BOTTLES if it's less than zero
    let mut next_bottle_num = bottle - 1;
    if next_bottle_num < 0 {
        next_bottle_num = MAX_BOTTLES;
    }
    let next_bottle = get_bottle_number(next_bottle_num);

    // Add the optional plurals
    let current_plural = get_bottle_plural(bottle);
    let next_plural = get_bottle_plural(next_bottle_num);

    Bottle {
        current: current_bottle,
        next: next_bottle,
        current_plural: current_plural,
        next_plural: next_plural
    }
}

/**
 * Returns what should we do to/for the next bottle
 */
fn get_instruction(bottle: i32) -> &'static str {
    if bottle == 0 {
        "Go to the store and buy some more"
    } else if bottle == 1 {
        "Take it down and pass it around"
    } else {
        "Take one down and pass it around"
    }
}

/**
 * Swap 0 for "no more" and return as a string
 */
fn get_bottle_number(bottle_num: i32) -> String {
    if bottle_num == 0 {
        "No more".to_string()
    } else {
        bottle_num.to_string()
    }
}

/**
 * Decides whether to add "s" to a bottle number, e.g. "bottle" vs "bottles"
 */
fn get_bottle_plural(bottle: i32) -> &'static str {
    if bottle == 1 {
        ""
    } else {
        "s"
    }
}
