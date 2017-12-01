#[allow(dead_code)]
enum Abc {
    A,
    B,
    C
}

#[allow(dead_code)]
enum Person {
    Male,
    Female,
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 }
}

use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn inspect(p: Person) {
    match p {
        Person::Male => println!("Is male."),
        Person::Female => println!("Is female."),
        Person::Engineer => println!("Is engineer."),
        Person::Scientist => println!("Is scientist."),
        Person::Height(i) => println!("Has a height of {}", i),
        Person::Weight(i) => println!("Has a weight of {}", i),
        Person::Info { name, height } => {
            println!("{} is {} tall", name, height);
        },
    }
}

#[allow(unused_variables)]

fn main() {
    let p1 = Person::Height(20);
    let alice = Person::Female;
    let mike = Person::Male;
    let chandler = Person::Weight(30);
    let joy = Person::Info { name: "Joy".to_owned(), height: 72 };
    let ross = Person::Scientist;
    let peter = Person::Engineer;

    inspect(p1);
    inspect(alice);
    inspect(mike);
    inspect(chandler);
    inspect(joy);
    inspect(ross);
    inspect(peter);

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
