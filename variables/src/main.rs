fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 *60 * 3;
    let mut x = 5;
    println!("The value of x is {x}");
    x = 7;
    println!("The value of x is {x}");

    let y = 8;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "  ";
    let spaces = spaces.len();
    println!("Spaces required: {spaces}");
}
