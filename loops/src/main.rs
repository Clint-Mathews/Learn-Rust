fn main() {
    // loop{
    // println!("Hello, world!");
    // }
    let test = return_counted_value();
    println!("{test}");

    loop_label();
    while_loop();

    for_loop();
}


fn return_counted_value() -> i32 {
    let mut counter :i32 = 0;

    let result = loop {
        counter += 1;
        if counter == 100 {
            break counter + 1;
        }
    };

    result
}


fn loop_label() {
    let mut count = 0;
    'counting_up: loop{
        println!("count : {count}");
        let mut rem = 100;

        loop {
            println!("rem : {rem}");
            if rem == 0 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            rem -= 1;
        }
        count +=1;
    }
    println!("count : {count}");
}


// How loop label break works!
// 'loopp_label: loop{
//     break 'loopp_lable;
// }


fn while_loop() {
    let mut count = 3;
    while count > 0 {
        println!("while count : {count}");
        count -= 1;
    }
}


fn for_loop() {
    let a = [1,2,3,4,5,6];
    let mut index = 0;
    while index < 6{
        println!("while array val: {}", a[index]);
        index += 1;
    }

    for elem in a {
        println!("for array val: {}", elem);
    }


    println!("Count down!");

    for num in (1..4).rev() {
        println!("{}", num);
    }
    println!("Lift off!!");

    // to create an range is using (start..end)
    //.rev() reverses the range data

}