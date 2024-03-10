use std::io;

fn main() {
    // let guess = "45".parse().expect("Not a number");
    let guess:u32 = "45".parse().expect("Not a number");
    let x = 2.0;
    let y:f32 = 23.09;

    let tq = 12/3;
    let q = 12.9/3.0;
    println!("{tq}");
    println!("{q}");

    let t = true;
    let f: bool = false;
    let c = 'z';
    let o:char = 'H';

    let tup: (u32, f64, u8) = (122,34.2,2);
    let (x1,y1,z1) = tup;
    println!("{x1},{y1},{z1}");

    let k: (i32, f64, char) = (300,23.23,'J');
    let first = k.0;
    let second = k.1;
    let third = k.2;
    println!("{first} {second} {third}");

    let a = [1,2,3,4,5];
    let months = ["Januvary", "February", "March", "April", "May", "June", "July"];

    let b: [i32; 5] = [1,2,3,4,5];
    let dup = [3; 5];

    let third = dup[3];
    println!("{third}");

    let a = [1,2,3,4,5];
    println!("Enter inde to access: ");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Error reading index");

    let index: usize = index.trim().parse().expect("Index entered is not a number!");

    let element = a[index];
    println!("Value of index {index} is {element}.")


}
