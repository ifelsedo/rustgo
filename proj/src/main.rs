fn main() {
    let a = "Mike Rust";
    println!("string is {}", a);
    let i: i32 = 3;
    let j: i32 = 9;
    println!("{} + {} = {}", i, j, num_sum(i, j));
}

fn num_sum(i: i32, j: i32) -> i32 {
    i + j
}

/*fn diverges() -> ! {
    panic!("This function never returns!");
}*/