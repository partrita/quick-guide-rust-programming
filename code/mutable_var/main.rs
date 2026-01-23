fn fib(mut index: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut t;

    loop {
        t = a + b;
        a = b;
        b = t;

        index -= 1;
        if index <= 0 {
            break;
        }
    }
    b
}

fn main() {
    println!("{}", fib(10));
}
