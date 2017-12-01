fn main() {
    let a = 1i32;
    let b = false;
    let c = 'c';

    let d = if (a >= 1 && b) || (c == 'a') {
        println!("yes, b={}", b);
        a
    } else {
        println!("no, c={}", c);
        a - 1
    };

    println!("d={}", d);

    let mut i = 0i32;

    /*loop {
        i = i + 1;

        println!("i={}, loop", i);
        if i == 3 {
            println!("i={}, continue", i);
            continue;
        }

        if i >= 6 {
            println!("i={}, break", i);
            break;
        }
    }*/

    /*for i in 1..101 {
        if i % 15 == 0 {
            println!("i={}, fizzbuzz", i);
        } else if i % 5 == 0 {
            println!("i={}, buzz", i);
        } else if i % 3 == 0 {
            println!("i={}, fizz", i);
        } else {
            println!("i={}", i);
        }
    }*/

    if i > 0 {
        let x = i;
        i = i * 2;
        println!("in, true, i={}, x={}", i, x);
    } else {
        let x = 0;
        println!("in, false, i={}, x={}", i, x);
    }

    // println!("out, i={}, x={}", i, x);

    let number = 2;
    print!("Tell me about {}, then ", number);
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 | 4 | 5 => println!("This is a prime"),
        13...19 => println!("A teenager"),
        _ => println!("Others!"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        // true => 1,
    };
    println!("{} -> {}", boolean, binary);

}
