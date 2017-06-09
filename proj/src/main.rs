fn main() {
    let a: char = '💕';
    println!("char is {}", a);
    let i: i32 = 3;
    let j: i32 = 9;
    println!("{} + {} = {}", i, j, num_sum(i, j));
}

fn num_sum(i: i32, j: i32) -> i32 {
    i + j
}