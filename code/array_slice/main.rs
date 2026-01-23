fn sum_array_ref(nums: &[i32]) -> i32 {
    let mut s = 0;
    let len = nums.len();
    let mut index = 0;
    loop {
        if index >= len {
            return s;
        }
        s += nums[index];
        index += 1;
    }
}

fn sum_slice(nums: &[i32]) -> i32 {
    let mut s = 0;
    for i in nums.iter() {
        s += i;
    }
    s
}

fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];

    println!("Array: {:?}", numbers);
    println!("Slice: {:?}", slice);

    println!("{}", sum_array_ref(&numbers));
    println!("{}", sum_slice(slice));
}
