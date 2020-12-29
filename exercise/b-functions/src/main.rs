fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    // 1. Try running this code with `cargo run` and take a look at the error.
    //
    // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
    // doing `cargo run` should succeed and print something out.
    let area = area_of(width, height);
    println!("Area is {}", area);

    // 2. The area that was calculated is not correct! Go fix the area_of() function below, then run
    //    the code again and make sure it worked (you should get an area of 28).

    // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
    //    Create the `volume` function!  It should:
    //    - Take three arguments of type i32
    //    - Multiply the three arguments together
    //    - Return the result (which should be 280 when you run the program).
    //
    // If you get stuck, remember that this is *very* similar to what `area_of` does.
    //
    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
