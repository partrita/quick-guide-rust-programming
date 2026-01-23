fn get_moved_string(data: &str) {
    println!("{}", data);
}

fn main() {
    let _hello = String::from("Hello, world!");
    let mut _s = String::new();
    let _s = "initial contents".to_string();
    let _hello = String::from("안녕하세요");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);

    let moving_string = String::from("hello");

    get_moved_string(&moving_string);
    println!("{}", moving_string);

    let mut mutable_string = String::from("hello");
    mutable_string.push_str(" world");
    println!("{}", mutable_string.chars().nth(0).unwrap());

    {
        let hello = "hell".to_string();
        let _r1 = &hello;
        //let mut r2 = &mut hello; // Build Error!!!
    }
}
