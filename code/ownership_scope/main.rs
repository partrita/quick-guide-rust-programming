struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct now!");
    }
}

fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
        println!("{}", world_string);
    }
    println!("{} again", hello_string);
}

fn duplicated_names() {
    let hello_string = String::from("hello");
    {
        let hello_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", hello_string);
}

fn main() {
    internal_scope();
    duplicated_names();

    println!("main starts");
    {
        println!("inner-scope starts");
        let _my: MyStruct = MyStruct {};
        println!("inner-scope ends");
    }
    println!("main ends");
}
