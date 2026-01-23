fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        return Err(String::from("denominator cannot be zero"));
    }
    Ok(numerator / denominator)
}

fn main() {
    let ok_number = divide(10, 2);
    let error_number = divide(10, 0);

    let double_ok = ok_number.map(|x| x * 2);
    let double_error = error_number.map(|x| x * 2);

    println!("Double Ok: {:?}", double_ok); // Double Some: Ok(5)
    println!("Double Error: {:?}", double_error); // Double Error: Error
}