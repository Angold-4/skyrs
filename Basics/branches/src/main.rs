fn main() {
    let mut count = 0;

    // label -> counting_up
    let ret: i32 = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up count;
            }
            remaining -= 1;
        }

        if remaining % 2 == 0 {
            break remaining - 1;
        }
        count += 1;
    };

    println!("ret = {ret}");
}
