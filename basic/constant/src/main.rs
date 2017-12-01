const A_USERID: u32 = 1000;
static A_USER: &'static str = "Mike";

fn is_insider(uid: u32) -> bool {
    uid < A_USERID
}

fn main() {
    let uid = 1311u32;
    let r = is_insider(uid);
    println!("{} is insider? {}", uid, r);
    println!("Hello, {}, {}!", A_USERID, A_USER);

    let a: f64 = 6.0;
    println!("a={}", a);
    let a = a * 5.0;
    println!("a={}", a);
    let a = a as i32 + 8.1 as i32;
    println!("a={}", a);

    // let a = a as bool;
    println!("a={}", a);

    let a = true;
    println!("a={}", a);
    let a = a as i32;
    println!("a={}", a);

    // A_USERID = 122;
}
