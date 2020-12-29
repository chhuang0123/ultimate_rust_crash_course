// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut)]

fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("{} is plural", s)
    } else {
        println!("{} is singular", s)
    }
}

fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

fn eat(s: String) -> bool {
    s.ends_with("s") && s.contains("a")
}

fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args()
        .collect::<Vec<String>>()
        .iter()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Please supply an argument to this program.");
            std::process::exit(-1);
        })
        .to_owned();

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "add" that takes *references* to two integer arguments,
    // dereferences them and adds them together, and returns the result.
    //
    println!("1 + 2 = {}, even via references", add(&1, &2));
}
