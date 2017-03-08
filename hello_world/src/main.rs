// use std::thread;

fn main() {
    // 一个简易计算器
    // `+` 或 `-` 意味着加减1
    // `*` 或 `/` 意味着乘除2

    let program = "+ + * - /";
    let mut accumulator = 1;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* 忽略其他 */ }
        }
    }

    println!("程序 \"{}\" 的结果是 {}",
              program, accumulator);
}

/*fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in 0..50_000_000 {
                x += 1
            }
            x
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
        h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
}*/
