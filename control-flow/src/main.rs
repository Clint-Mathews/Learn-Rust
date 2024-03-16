fn main() {
    let x:i32 = 10;

    if x==0 {
        println!("Zero");
    } else if x > 0 {
        println!("Positive")
    } else {
        println!("Negative")
    }

    if x > 9 {
        println!("true");
    } else {
        println!("false");
    }
    println!("Hello, world!");


    let condition = false;
    // If and else block should return an expression of same type or else it results in error
    let number = if condition {10} else {-10};
    println!("Number: {number}")

}
