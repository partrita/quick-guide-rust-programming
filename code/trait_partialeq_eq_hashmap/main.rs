#![allow(dead_code)]

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct MyKey {
    x: i32,
    y: i32,
}

struct MyVal {
    distance_from_co_origin: f32,
}

fn main() {
    let mut map = HashMap::new();
    map.insert(
        MyKey { x: 3, y: 4 },
        MyVal {
            distance_from_co_origin: 5.0,
        },
    );
}
