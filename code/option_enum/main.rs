fn second(s: &[i32]) -> Option<i32> {
    if s.len() == 0 {
        None
    } else {
        Some(s[1])
    }
}

fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(n) => println!("x is {}", n),
        None => println!("x is not present"),
    }

    match y {
        Some(n) => println!("y is {}", n),
        None => println!("y is not present"),
    }

    if let Some(n) = x {
        println!("x is {}", n);
    }

    if let Some(n) = y {
        println!("y is {}", n);
    } else {
        println!("y is not present");
    }

    let x: Option<i32> = Some(5);
    let _y: Option<i32> = None;

    println!("x is {}", x.unwrap());
    //println!("y is {}", _y.unwrap()); // panic!!!

    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x is {}", x.unwrap_or(-1));
    println!("y is {}", y.unwrap_or_default());

    let y: Option<i32> = second(&[]);
    let item = y.expect("An argument of second should not be empty");
    println!("This line is not reachable because item is {}", item);
}
