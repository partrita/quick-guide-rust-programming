fn main() {
    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.iter() {
        println!("{}", c);
    }
    println!("{:?}", user);

    let a = String::from("Hello");
    let t = a + ",world";
    println!("{}", t);
    //println!("{}", a); // compile error!!!
}
