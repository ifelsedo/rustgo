/* Primitive Type Rust 原生类型 */

fn main()
{
    // bool 布尔型
    /*let t = true;
    let f: bool = false;
    println!("{:?}", f);*/

    // char 字符型 使用单引号 ' 定义 值必须为单独的 Unicode 字符
    // let c = 'a';

    // 整型
    // i8 8位有符号 -2^7=-128 ~ 2^7-1=127
    // u8 8位无符号 0 ~ 2^8-1=255
    // i16 16位有符号 -2^15=-32768 ~ 2^15-1=32767
    // u16 16位无符号 0 ~ 2^16-1=65535
    // i32 32位有符号整型 取值范围 -2147483648 ~ 2147483647
    // u32 32位无符号整型 取值范围 0 ~ 4294967295
    // i64 64位有符号整型 取值范围 -2^63=-9223372036854775808 ~ 2^63-1=9223372036854775807
    // u64 64位无符号整型 取值范围 0 ~ 2^64-1=18446744073709551615

    // isize 可变长有符号整型
    // usize 可变长无符号整型
    // println!("isize 可变长度有符号整型 取值范围 {} ~ {}", isize::min_value(), isize::max_value());

    /*println!("i8 8位有符号整型 取值范围 {} ~ {}", i8::min_value(), i8::max_value());
    println!("u8 8位无符号整型 取值范围 {} ~ {}", u8::min_value(), u8::max_value());
    println!("i16 16位有符号整型 取值范围 {} ~ {}", i16::min_value(), i16::max_value());
    println!("u16 16位无符号整型 取值范围 {} ~ {}", u16::min_value(), u16::max_value());
    println!("i32 32位有符号整型 取值范围 {} ~ {}", i32::min_value(), i32::max_value());
    println!("u32 32位无符号整型 取值范围 {} ~ {}", u32::min_value(), u32::max_value());
    println!("i64 64位有符号整型 取值范围 {} ~ {}", i64::min_value(), i64::max_value());
    println!("u64 64位无符号整型 取值范围 {} ~ {}", u64::min_value(), u64::max_value());*/
    assert_eq!(i8::max_value(), 127);

    // 浮点型
    // f32 32位单精度浮点型
    // f64 64位双精度浮点型

    use std::f32;

    let nan = f32::NAN;
    let f = 7.0_f32;

    // println!("{:?}", nan.is_nan());

    // println!("{:?}", !f.is_infinite());

    assert!(nan.is_nan());
    assert!(!f.is_nan());

    // array 数组 使用 [] 定义索引数组
    let arr = vec!['h', 'e', 'l', 'l', 'o'];

    // println!("{:?}", arr.len());
    // println!("{}", arr[4]);

    let a = &arr[2..];
    println!("{}", a[2]);

    // 元组
    let tup = (1, "premium", false, 'a');
    println!("{:#?}", tup);
}
