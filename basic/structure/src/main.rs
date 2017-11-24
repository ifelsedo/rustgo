#[allow(dead_code)]
struct Nil;

#[allow(dead_code)]
struct Pair(i32, f32);


#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
#[allow(unused_variables)]
fn main() {

    let point1: Point = Point { x: 4, y: 5 };
    println!("point coordinates: ({}, {})", point1.x, point1.y);

    let Point {x, y} = point1;
    println!("point coordinates: ({}, {})", x, y);

    let r1 = Rectangle {
        p1: Point { x: x, y: y },
        p2: point1,
    };

    // println!("Rectangle: p1({:#?}), p2({:#?})", r1.p1, r1.p2);

    let _nil = Nil;

    let pair1 = Pair(1, 3.14);

    println!("pair contains {} and {}", pair1.0, pair1.1);

    let Pair(i, d) = pair1;

    println!("pair contains {} and {}", i, d);

    let r2 = Rectangle {
        p1: Point { x: x, y: y },
        p2: Point { x: 11, y: 8 },
    };

    println!("r2's area is {}", rectangle_area(r2));
}

fn rectangle_area(rec: Rectangle) -> i32 {
    let Rectangle { p1, p2 } = rec;
    let Point { x: x1, y: y1 } = p1;
    let Point { x: x2, y: y2 } = p2;

    (x2 - x1) * (y2 - y1)
}
