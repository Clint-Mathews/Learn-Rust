fn main() {
    println!("Hello, world!");
    lets_test(10);
    print_value_and_unit(10, 'c');
}

fn lets_test(x:i32) {
    println!("Another fucntion! x: {x}");
}

fn print_value_and_unit(val:i32, unit:char) {
    println!("val :{val} , unit: {unit}");
    let y = {
        let x =10;
        x+1
    };
    println!("{y}");
    let u = return_five_plus_one(10);

    println!("u : {u}");
}

fn return_five_plus_one(i : i32) -> i32 {
    i+1
}

