use std::fmt;

#[derive(Debug)]
struct A1(i64, i64);

impl fmt::Display for A1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct A2D {
    x: f64,
    y: f64,
}

impl fmt::Display for A2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;

        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                try!(write!(f, ", "));
            }
            try!(write!(f, "{}: {}", count, v));
        }

        write!(f, "]")
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { "北纬" } else { "南纬" };
        let lon_c = if self.lon >= 0.0 { "东经" } else { "西经" };

        write!(f, "{}: {}{:.3}° {}{:.3}°",
               self.name, lat_c, self.lat.abs(), lon_c, self.lon.abs())
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0X{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.r, self.g, self.b)
    }
}

fn main() {
    let a = A1(4, 16);

    println!("Display: {}", a);
    println!("Debug: {:?}", a);

    let b = A2D {x: 3.14, y: 8.88};

    println!("Display: {}", b);
    println!("Debug: {:?}", b);

    let c = Complex {real: 3.0, imag: 9.2};

    println!("Display: {}", c);
    println!("Debug: {:?}", c);

    let d = List(vec![1, 2, 3, 4, 5, 11, 99]);

    println!("{}", d);

    for city in [
        City { name: "北京", lat: 39.67, lon: 116.40},
        City { name: "上海", lat: 31.23, lon: 121.47},
        City { name: "广州", lat: 23.13, lon: 113.26},
        City { name: "深圳", lat: 22.54, lon: 114.05},
        City { name: "杭州", lat: 30.27, lon: 120.15},
        City { name: "伦敦", lat: 51.50, lon: -0.12},
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { r: 128, g: 255, b: 90 },
        Color { r: 0, g: 0, b: 0 },
        Color { r: 255, g: 0, b: 0 },
        Color { r: 0, g: 255, b: 0 },
        Color { r: 255, g: 255, b: 255 },
    ].iter() {
        println!("{}", *color);
    }
}
