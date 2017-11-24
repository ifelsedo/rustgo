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

fn inspect(p: Person) {
    match p {
        Person::Male => println!("Is male.");
        Person::Female => println!("Is female.");
    }
}


fn main() {
    println!("Hello, world!");
}
