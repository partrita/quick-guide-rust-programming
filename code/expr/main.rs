#![allow(dead_code)]

fn ret_zero() -> i32 {
    0
}

fn main() {
    let age = 44;
    let gen = match age {
        0..=20 => "MZ",
        21..=50 => "X",
        51..=100 => "A",
        _ => "?",
    };
    println!("generation={}", gen);

    let num = 45;
    let _var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            5
        } else {
            0
        }
    };

    let x = 9;
    let _y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
}
