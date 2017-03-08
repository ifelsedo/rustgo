use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} 正在吃.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} 吃完了.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),     // 译者注：朱迪斯·巴特勒
        Philosopher::new("Gilles Deleuze", 1, 2),    // 译者注：吉尔·德勒兹
        Philosopher::new("Karl Marx", 2, 3),         // 译者注：卡尔·马克思
        Philosopher::new("Emma Goldman", 3, 4),      // 译者注：爱玛·戈德曼
        Philosopher::new("Michel Foucault", 0, 4),   // 译者注：米歇尔·福柯
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

}
