fn main()
{
    let t: f64 = 78.8;
    let tf = foo(t, 'C');
    println!("摄氏温度换算华氏温度：{} => {}", t, tf);

    bar();

    let s = String::from("hello world rust learn 2017 basic func src main");
    let n: u64 = 6;
    let a = get_word(&s, n);
    println!("第 {} 个单词 {}", n, a);

    // let s: &str = "sjfsslad sdjfs. sadhufoe, ashdf";
    let s: String = String::from("sjfsslad sdjfs. sadhufoe, ashdf");
    println!("{}", s);
}

/// # 示例
///
fn foo(t: f64, o: char) -> f64
{
    let a: f64 = 1.8;
    let b: f64 = 32.0;
    if o == 'F' {
        // t * 1.8 + 32.0
        t.mul_add(a, b)
    } else if o == 'C' {
        // t.mul_add(a.recip(), -b * a.recip())
        t / a - b / a
    } else {
        t
    }
}

fn bar()
{
    println!("这是个函数");
}

fn get_word(s: &String, n: u64) -> &str
{
    let bs = s.as_bytes();
    let l = s.len();
    let mut j: usize = 0;
    let mut m: u64 = 0;
    for (i, &item) in bs.iter().enumerate() {
        if item == b' ' {
            m = m + 1;
            if n == m {
                return &s[j..i];
            }
            j = i + 1;
        } else if i + 1 == l {
            return &s[j..l];
        }
    }
    &s[..]
}