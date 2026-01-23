#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

impl<T> std::ops::Add for Pair<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            first: self.first + rhs.first,
            second: self.first + rhs.second,
        }
    }
}

fn main() {
    let left_pair: Pair<i32> = Pair {
        first: 5,
        second: 10,
    };

    let right_pair: Pair<i32> = Pair {
        first: 10,
        second: 5,
    };

    let result = add(left_pair, right_pair);
    println!("Sum: {:?}", result);

    let left_pair_u32: Pair<u32> = Pair {
        first: 5,
        second: 10,
    };

    let right_pair_u32: Pair<u32> = Pair {
        first: 10,
        second: 5,
    };

    let result = add(left_pair_u32, right_pair_u32);
    println!("Sum: {:?}", result);
}
