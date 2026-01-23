fn main() {
    let some_number = Some(5);
    let none_number: Option<i32> = None;

    let double_some = some_number.map(|x| x * 2);
    let double_none = none_number.map(|x| x * 2);

    println!("Double Some: {:?}", double_some); // Double Some: Some(10)
    println!("Double None: {:?}", double_none); // Double None: None
}