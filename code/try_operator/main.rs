fn foo() -> Result<i32, String> {
    let r = bar()?;
    println!("Do something with {}", r);
    return Ok(1);
}

fn bar() -> Result<i32, String> {
    let r = foobar()?;
    println!("Do something with {}", r);
    return Ok(1);
}

fn foobar() -> Result<i32, String> {
    let r = "foobar error".to_string();
    Err(r)
}

fn main() {
    let r = foo();
    match r {
        Ok(n) => println!("Do something with {}", n),
        Err(s) => println!("Do error handling with {}", s),
    }
}
