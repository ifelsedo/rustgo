extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜猜看");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is {}", secret_num);

    for i in 1..6 {
        println!("请输入你的猜想值。");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fail to read line");

        println!("第 {} 次，你的猜想是：{}", i, guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_num) {
            Ordering::Less    => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }

        if i >= 5 {
            println!("You lose! The secret number is {}.", secret_num);
            break;
        }
    }
}
