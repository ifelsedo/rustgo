fn main() {
    let a = 2;
    let b = a;
    let c = b.clone() + a.clone();
    println!("a={}, b={}, c={}", a, b, c);
}

/*fn num_sum(i: i32, j:i32) -> i32
{
    i + j
}*/

/*fn diverges() -> ! {
    panic!("This function never returns!");
}*/