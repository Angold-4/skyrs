fn main() {
    let a = 10;
    let b: i32 = 20;
    let mut c = 30i32;
    let d = 30_i32;

    let e = add(add(a, b), add(c, d));

    // println! is a marco call -> ! extract the code block
    // of fn println instead of fn itself
    println!("(a + b) + (c + d) = {}", e);

    let c = 'c';
    println!("size of a char: {}", std::mem::size_of_val(&c)); // 4
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}
